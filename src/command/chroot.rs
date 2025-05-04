#![allow(dead_code, unused)]
use std::{path::Path, process::Command};

use anyhow::{bail, Result};
use log::{debug, warn};
use rizzybox::UserGroupPair;
use rustix::process::geteuid;
// use rustix::process::{chroot, geteuid};
use styrolite::{
    config::{Config, CreateRequest, IdMapping, Wrappable},
    namespace::{self, Namespace},
    runner::{CreateRequestBuilder, Runner},
};

// Group all Linux-specific imports and traits together
#[cfg(target_os = "linux")]
use {
    anyhow::Context,
    rustix::{
        mount::{mount, unmount, MountFlags, UnmountFlags},
        process::pivot_root,
    },
    std::ffi::CString,
    std::os::unix::fs::MetadataExt,
};

pub fn chroot_command(
    new_root: &str,
    userspec: Option<&UserGroupPair>,
    command: Vec<String>,
) -> Result<()> {
    if !geteuid().is_root() {
        bail!("chroot: you must be root to use this command");
    }

    let new_root_path = Path::new(new_root);
    if !new_root_path.exists() || !new_root_path.is_dir() {
        bail!("chroot: '{}' is not a directory", new_root);
    }

    let new_root = if !new_root.starts_with('/') {
        std::fs::canonicalize(new_root)?
            .to_string_lossy()
            .to_string()
    } else {
        new_root.to_string()
    };

    debug!("Using absolute rootfs path: {}", new_root);

    let (uid, gid) = match userspec {
        Some(user_group) => user_group.resolve_ids()?,
        None => (65534, 65534), // default to nobody
    };

    #[cfg(target_os = "macos")]
    {
        chroot(new_root)?;
        std::env::set_current_dir("/")?;

        // Handle userspec if provided
        if let Some(spec) = userspec {
            // Set group first, then user (because unprivileged users can't change group)
            if let Some(group) = &spec.group {
                set_group(group)?;
            }

            if !spec.user.is_empty() {
                set_user(&spec.user)?;
            }
        }

        // Execute the command
        if !command.is_empty() {
            let status = Command::new(&command[0]).args(&command[1..]).status()?;
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    #[cfg(target_os = "linux")]
    {
        #[allow(unused_mut)]
        let executable = &command[0];
        let args: Vec<&str> = command.iter().skip(1).map(|s| s.as_str()).collect();

        let mut container_request: CreateRequest = CreateRequestBuilder::new()
            .set_rootfs(&new_root)
            .set_uid(uid)
            .set_gid(gid)
            .push_namespace(Namespace::Mount)
            .push_namespace(Namespace::Time)
            .push_namespace(Namespace::Uts)
            .push_namespace(Namespace::Pid)
            .push_namespace(Namespace::Ipc)
            .push_namespace(Namespace::Cgroup)
            .push_namespace(Namespace::Net)
            // .push_namespace(Namespace::User) // sadly, this breaks the mount permissions
            .set_setgroups_deny(false)
            // .push_mapping(IdMapping::default())
            // .push_gid_mapping(IdMapping::default())
            .set_workload_id("rizzybox-chroot")
            .set_hostname("rizzybox-container")
            .push_environment(
                "PATH",
                "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
            )
            .set_executable(executable)
            .set_arguments(args)
            .set_working_directory("/")
            .to_request();

        container_request.wrap()?; // exec the container

        // if let Err(e) = setup_and_pivot_root(new_root) {
        //     warn!("chroot: pivot_root failed: {}. Trying chroot instead.", e);
        //     chroot(new_root)?;
        //     std::env::set_current_dir("/")?;
        // }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn setup_and_pivot_root(new_root: &str) -> Result<()> {
    let new_root_path = std::path::PathBuf::from(new_root);

    // Check if new_root is a directory
    if !new_root_path.is_dir() {
        bail!("new_root must be a directory");
    }

    // Create put_old directory inside new_root
    let put_old = new_root_path.join(".old_root");
    if !put_old.exists() {
        std::fs::create_dir_all(&put_old).context("Failed to create .old_root directory")?;
    }

    // Check if new_root is already a mount point
    let is_mount_point = is_mount_point(&new_root_path)?;

    if !is_mount_point {
        eprintln!(
            "Bind mounting '{}' to itself to make it a mount point",
            new_root
        );

        // Convert strings to CString
        let source = CString::new(new_root).context("Failed to create source CString")?;
        let target = source.clone();
        let fs_type = CString::new("none").context("Failed to create fs_type CString")?;

        // Call mount with proper CString parameters
        mount(
            &source,
            &target,
            &fs_type,
            MountFlags::BIND,
            None::<&std::ffi::CStr>,
        )
        .context("Failed to bind mount new_root to itself")?;
    }

    // Change to new root directory before pivot_root
    std::env::set_current_dir(&new_root_path).context("Failed to change directory to new_root")?;

    // Now call pivot_root with relative paths
    pivot_root(".", ".old_root").context("pivot_root failed")?;

    // Change to the new root
    std::env::set_current_dir("/").context("Failed to change directory to / after pivot_root")?;

    // Optionally unmount old root to fully detach from the old filesystem
    // This is often desired but may fail if processes are still using the old root
    if let Err(e) = unmount(".old_root", UnmountFlags::empty()) {
        eprintln!("Warning: Failed to unmount .old_root: '{}'", e);
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn is_mount_point(path: &Path) -> Result<bool> {
    // Get metadata for the path itself
    let path_meta = std::fs::metadata(path).context("Failed to get metadata for path")?;

    // Create a path for the parent directory
    let parent_path = if let Some(parent) = path.parent() {
        parent
    } else {
        // Root has no parent, but it's definitely a mount point
        return Ok(true);
    };

    // Get metadata for the parent directory
    let parent_meta =
        std::fs::metadata(parent_path).context("Failed to get metadata for parent path")?;

    // If the device IDs are different, it's a mount point
    // st_dev is the device ID on which the file resides
    Ok(path_meta.dev() != parent_meta.dev())
}

fn set_user(user: &str) -> Result<()> {
    let uid = if let Ok(id) = user.parse::<u32>() {
        // User is specified as numeric ID
        id
    } else {
        // User is specified as name, look it up
        match get_user_id(user) {
            Some(id) => id,
            None => bail!("chroot: unknown user: {}", user),
        }
    };

    // Use libc's setuid for both platforms
    let result = unsafe { libc::setuid(uid) };
    if result != 0 {
        bail!("failed to set user ID to '{}'", uid);
    }

    Ok(())
}

fn set_group(group: &str) -> Result<()> {
    let gid = if let Ok(id) = group.parse::<u32>() {
        // Group is specified as numeric ID
        id
    } else {
        // Group is specified as name, look it up
        match get_group_id(group) {
            Some(id) => id,
            None => bail!("chroot: unknown group: '{}'", group),
        }
    };

    // Use libc's setgid for both platforms
    let result = unsafe { libc::setgid(gid) };
    if result != 0 {
        bail!("failed to set group ID to '{}'", gid);
    }

    Ok(())
}

fn get_user_id(username: &str) -> Option<u32> {
    // TODO: Don't rely on external bin
    let output = Command::new("id").arg("-u").arg(username).output().ok()?;

    if output.status.success() {
        let uid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        uid_str.parse::<u32>().ok()
    } else {
        None
    }
}

fn get_group_id(groupname: &str) -> Option<u32> {
    // TODO: Don't rely on external bin
    let output = Command::new("getent")
        .arg("group")
        .arg(groupname)
        .output()
        .ok()?;

    if output.status.success() {
        let group_info = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = group_info.split(':').collect();
        if parts.len() >= 3 {
            parts[2].parse::<u32>().ok()
        } else {
            None
        }
    } else {
        None
    }
}
