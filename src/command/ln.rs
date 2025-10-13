use anyhow::{bail, Result};

pub fn ln_command(force: bool, symlink: bool, source: &str, destination: &str) -> Result<()> {
    if force {
        // if a file doesn't exist, remove_file will fail, so ensure it exists first
        if std::fs::metadata(destination).is_ok() {
            let _ = std::fs::remove_file(destination);
        }
    }

    if symlink {
        let symlink_result = std::os::unix::fs::symlink(source, destination);
        match symlink_result {
            Ok(_) => Ok(()),
            Err(e) => {
                bail!("Failed to create symlink: {e}");
            }
        }
    } else {
        todo!("Creating hardlinks is not supported yet...");
    }
}
