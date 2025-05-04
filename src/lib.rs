use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::{fs::remove_file, string::String};
use users::{get_group_by_gid, get_group_by_name, get_user_by_name, get_user_by_uid};

#[derive(Debug, Clone)]
pub struct UserGroupPair {
    pub user: String,
    pub group: Option<String>,
}

impl UserGroupPair {
    pub fn new(user: String, group: Option<String>) -> Self {
        Self { user, group }
    }

    // fn default() -> Self {
    //     Self {
    //         user: "65534".to_string(), // nobody
    //         group: None,
    //     }
    // }

    pub fn resolve_ids(&self) -> std::io::Result<(u32, u32)> {
        // If the uid and optional gid are already numeric, return them
        if let Some(uid) = self.user.parse::<u32>().ok() {
            if let Some(gid) = self.group.as_ref().and_then(|g| g.parse::<u32>().ok()) {
                return Ok((uid, gid));
            }
        }

        // Resolve user ID
        let uid = match u32::from_str(&self.user) {
            // If user is a numeric string
            Ok(uid) => {
                // Verify the UID exists
                if get_user_by_uid(uid).is_none() {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("No user with uid {}", uid),
                    ));
                }
                uid
            }
            // If user is not numeric, look up by name
            Err(_) => match get_user_by_name(&self.user) {
                Some(user) => user.uid(),
                None => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("User not found: {}", self.user),
                    ))
                }
            },
        };

        // Resolve group ID if provided, otherwise use the user's primary group
        let gid = if let Some(group) = &self.group {
            match u32::from_str(group) {
                // If group is a numeric string
                Ok(gid) => {
                    // Verify the GID exists
                    if get_group_by_gid(gid).is_none() {
                        return Err(Error::new(
                            ErrorKind::NotFound,
                            format!("No group with gid {}", gid),
                        ));
                    }
                    gid
                }
                // If group is not numeric, look up by name
                Err(_) => match get_group_by_name(group) {
                    Some(group) => group.gid(),
                    None => {
                        return Err(Error::new(
                            ErrorKind::NotFound,
                            format!("Group not found: {}", group),
                        ))
                    }
                },
            }
        } else {
            // If no group provided, use the user's primary group
            match get_user_by_uid(uid) {
                Some(user) => user.primary_group_id(),
                None => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Failed to get primary group for user id {}", uid),
                    ))
                }
            }
        };

        log::debug!("uid: {}, gid: {}", uid, gid);

        Ok((uid, gid))
    }
}

pub mod consts {
    /// Binaries that can be installed with `--install`
    /// Example: `ln -sf /full/path/to/rizzybox /usr/local/bin/cat`
    pub const INSTALLABLE_BINS: [&str; 19] = [
        "arch", "basename", "cat", "clear", "chroot", "dirname", "echo", "env", "expand", "false",
        "ls", "mkdir", "sh", "sleep", "stem", "true", "uname", "which", "yes",
    ];
}

/// A simple error handler with formatting
pub fn handle_error<T>(result: Result<T, std::io::Error>, message: &str) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            eprintln!(
                "{}: {}: {}",
                std::env::current_exe()
                    .unwrap_or(env!("CARGO_PKG_NAME").into())
                    .display(),
                message,
                e
            );
            std::process::exit(1);
        }
    }
}

pub fn parse_kv_pair(s: &str) -> Result<String, String> {
    if s.contains('=') {
        Ok(s.to_string())
    } else {
        Err(format!("Invalid key-value pair: {s}"))
    }
}

pub fn parse_colon_separated_pair(s: &str) -> Result<UserGroupPair, String> {
    if s.contains(':') {
        let parts: Vec<&str> = s.split(':').collect();
        match parts.len() {
            2 => {
                let user = parts[0].to_string();
                let group = if parts[1].is_empty() {
                    None
                } else {
                    Some(parts[1].to_string())
                };
                Ok(UserGroupPair { user, group })
            }
            _ => Err(format!("Invalid colon-separated pair: {s}")),
        }
    } else {
        // No colon, treat as just user
        Ok(UserGroupPair {
            user: s.to_string(),
            group: None,
        })
    }
}

/// `TestCleanup` is a struct that implements the Drop trait to run cleanup code when it goes out of scope.
/// This is useful for removing temporary files or directories created during tests.
/// ONLY USE THIS FOR TESTS!
pub struct TestCleanup {
    pub file: Option<String>,
}

impl Drop for TestCleanup {
    fn drop(&mut self) {
        eprintln!("CLEANUP:");
        eprintln!("Removing file: {:?}", &self.file);
        if let Some(file) = &self.file {
            if let Err(e) = remove_file(file) {
                eprintln!("Failed to remove file: {e}");
            }
        }
    }
}
