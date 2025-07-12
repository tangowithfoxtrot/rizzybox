use std::path::PathBuf;

use anyhow::Result;

pub fn mkdir_command(dirs: Vec<PathBuf>, parents: bool) -> Result<()> {
    for dir in dirs {
        if parents {
            std::fs::create_dir_all(&dir)?;
        } else {
            std::fs::create_dir(&dir)?;
        }
    }
    Ok(())
}
