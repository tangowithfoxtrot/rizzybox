use anyhow::Result;
use bat::PrettyPrinter;
use std::collections::HashMap;
use std::env::remove_var;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn env_command(
    // argv0: &Option<String>, // TODO: implement this
    chdir: &Option<String>,
    command: &[String],
    ignore_environment: &bool,
    null: &bool,
    unset: &Vec<String>,
) -> Result<()> {
    let mut environment: HashMap<String, String> = HashMap::new();
    let mut kv_pairs = String::new();

    if let Some(dir) = chdir {
        let path = Path::new(dir);

        if !path.exists() {
            eprintln!(
                "{}: cannot change to directory '{}': No such file or directory",
                std::env::current_exe()?
                    .to_string_lossy()
                    .rsplit_once('/')
                    .unwrap()
                    .1,
                dir
            );
            std::process::exit(1);
        }
        if !path.is_dir() {
            eprintln!(
                "{}: cannot change to directory '{}': Not a directory",
                std::env::current_exe()?
                    .to_string_lossy()
                    .rsplit_once('/')
                    .unwrap()
                    .1,
                dir
            );
            std::process::exit(1);
        }
        let path_buf = PathBuf::from(dir);
        std::env::set_current_dir(path_buf)?;
    }

    let cmd = command.split_first();
    let args = cmd.map(|(_, args)| args).unwrap_or_default();
    let vars = std::env::vars();
    let line_ending = if *null { "" } else { "\n" };

    if !unset.is_empty() {
        for key in unset {
            remove_var(key);
        }

        if cmd.is_some() {
            let status = Command::new(cmd.unwrap().0)
                .args(args)
                .status()
                .expect("failed to execute process");
            std::process::exit(status.code().unwrap());
        }
    }

    if *ignore_environment || !unset.is_empty() {
        if cmd.is_some() {
            let status = Command::new(cmd.unwrap().0)
                .args(args)
                .env_clear()
                .status()
                .expect("failed to execute process");
            std::process::exit(status.code().unwrap());
        }
    } else if cmd.is_some() {
        let status = Command::new(cmd.unwrap().0)
            .args(args)
            .status()
            .expect("failed to execute process");
        std::process::exit(status.code().unwrap());
    } else {
        for (key, value) in vars {
            let kv_pair = format!("{}={}", key, value);
            kv_pairs.push_str(&kv_pair);
            kv_pairs.push_str(line_ending);
            environment.insert(key, value);
        }

        PrettyPrinter::new()
            .input_from_bytes(kv_pairs.as_bytes())
            .language("env")
            .print()
            .unwrap();
    }
    Ok(())
}
