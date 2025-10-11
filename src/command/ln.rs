use anyhow::{bail, Result};

pub fn ln_command(force: bool, symlink: bool, source: &str, destination: &str) -> Result<()> {
    if force {
        match std::fs::remove_file(destination) {
            Ok(_) => {}
            Err(e) => bail!("{e}"),
        }
    }

    if symlink {
        let symlink_result = std::os::unix::fs::symlink(source, destination);
        match symlink_result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to create symlink: {e}");
                std::process::exit(1);
            }
        }
    } else {
        todo!("Creating hardlinks is not supported yet...");
    }
}
