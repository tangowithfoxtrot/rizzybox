use anyhow::{Result, bail};
use bat::PrettyPrinter;
use std::{
    env::{remove_var, set_current_dir, vars},
    fmt::Display,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::Command,
};

use crate::which_command;
use rizzybox::handle_error;

#[derive(Debug)]
struct KVPair<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> KVPair<'a> {
    fn from(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
    fn parse(line: &'a str) -> Result<Self> {
        if let Some((k, v)) = line.split_once('=') {
            Ok(Self { key: k, value: v })
        } else {
            bail!("failed to parse key-value pair in line: {line}")
        }
    }
    fn as_string(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

impl Display for KVPair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

pub fn env_command(
    argv0: Option<&String>,
    chdir: Option<&String>,
    command: &[String],
    ignore_environment: bool,
    null: bool,
    unset: &Vec<String>,
    kv_pair: &[String],
) -> Result<()> {
    let line_ending = if null { "" } else { "\n" };

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
        eprintln!("command:            {command:?}");
        eprintln!("argv0:              {argv0:?}");
        eprintln!("chdir:              {chdir:?}");
        eprintln!("ignore_environment: {ignore_environment:?}");
        eprintln!("null:               {null:?}");
        eprintln!("unset:              {unset:?}");
        eprintln!("kv_pair:            {kv_pair:?}");
    }

    if let Some(mut cmd) = command.split_first() {
        let mut symlink_path_str: String = String::new();

        if let Some(arg) = argv0 {
            let temp_dir = std::env::temp_dir();
            let symlink_path = temp_dir.join(arg);
            let cmd_path = cmd.0;

            let cmd_path_abs = match which_command(false, cmd_path, true) {
                Ok(Some(path)) => path,
                Ok(None) => {
                    eprintln!("'{cmd_path}': No such file or directory");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
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
            unsafe { remove_var(key) };
        }

        let mut command = Command::new(cmd.0);
        command.args(cmd.1);

        if ignore_environment {
            command.env_clear();
        }

        for line in kv_pair {
            let kv_pair = KVPair::parse(line)?;
            command.env(kv_pair.key, kv_pair.value);
        }

        let Ok(status) = command.status() else {
            bail!("failed to execute command")
        };
        std::process::exit(status.code().unwrap_or(1));
    }

    let mut kv_pairs = String::new();
    for (key, value) in vars() {
        let kv_pair = KVPair::from(&key, &value);
        kv_pairs.push_str(&kv_pair.as_string());
        kv_pairs.push_str(line_ending);
    }

    PrettyPrinter::new()
        .input_from_bytes(kv_pairs.as_bytes())
        .language("env")
        .print()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[allow(unused_imports)]
    use rizzybox::*;

    #[test]
    fn success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("env");

        // Assert
        cmd.assert().success();
        // TODO: make a better test
        cmd.assert().stdout(predicates::str::contains("="));
    }
}
