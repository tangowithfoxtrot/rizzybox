use anyhow::{bail, Result};
use bat::PrettyPrinter;
use std::{
    env::{remove_var, set_current_dir, vars},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::Command,
};

use crate::which_command;
use rizzybox::handle_error;

pub(crate) fn env_command(
    argv0: &Option<String>,
    chdir: &Option<String>,
    command: &[String],
    ignore_environment: &bool,
    null: &bool,
    unset: &Vec<String>,
    kv_pair: &[String],
) -> Result<()> {
    if let Some(dir) = chdir {
        let path = Path::new(dir);
        handle_error(Ok(path.exists()), "Directory does not exist");
        handle_error(Ok(path.is_dir()), "Not a directory");
        handle_error(
            set_current_dir(PathBuf::from(dir)),
            "Failed to change directory",
        );
    }
    if cfg!(debug_assertions) {
        eprintln!("command: {:?}", command);
        eprintln!("argv0: {:?}", argv0);
        eprintln!("chdir: {:?}", chdir);
        eprintln!("ignore_environment: {:?}", ignore_environment);
        eprintln!("null: {:?}", null);
        eprintln!("unset: {:?}", unset);
        eprintln!("kv_pair: {:?}", kv_pair);
    }

    let mut cmd = command
        .split_first()
        .expect("some command should be provided");
    let line_ending = if *null { "" } else { "\n" };

    #[allow(unused_assignments)] // this actually gets used in the if-let below
    let mut symlink_path_str: String = String::new();

    if let Some(arg) = argv0 {
        let temp_dir = std::env::temp_dir();
        let symlink_path = temp_dir.join(arg);
        let cmd_path = cmd.0;

        let cmd_path_abs = match which_command(&false, cmd_path, &true) {
            Ok(Some(path)) => path,
            Ok(None) => {
                eprintln!("'{}': No such file or directory", cmd_path);
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        if symlink_path.exists() {
            handle_error(
                std::fs::remove_file(&symlink_path),
                "Failed to remove existing symlink",
            );
        }

        handle_error(
            symlink(&cmd_path_abs, &symlink_path),
            "Failed to create symlink",
        );

        symlink_path_str = symlink_path.to_string_lossy().to_string();
        cmd = (&symlink_path_str, cmd.1);
    }

    for key in unset {
        remove_var(key);
    }

    if !cmd.0.is_empty() {
        let mut command = Command::new(cmd.0);
        command.args(cmd.1);

        if *ignore_environment {
            command.env_clear();
        }

        for var in kv_pair.iter() {
            let mut split = var.splitn(2, '=');
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                command.env(key, value);
            }
        }

        let status = match command.status() {
            Ok(s) => s,
            Err(_) => bail!("failed to execute command"),
        };
        std::process::exit(status.code().unwrap_or(1));
    } else {
        let mut kv_pairs = String::new();
        for (key, value) in vars() {
            let kv_pair = format!("{}={}", key, value);
            kv_pairs.push_str(&kv_pair);
            kv_pairs.push_str(line_ending);
        }

        PrettyPrinter::new()
            .input_from_bytes(kv_pairs.as_bytes())
            .language("env")
            .print()?;
    }

    Ok(())
}
