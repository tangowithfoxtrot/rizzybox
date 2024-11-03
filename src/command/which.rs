use anyhow::Result;
use std::path::Path;

pub(crate) fn which_command(
    all_occurrences: &bool,
    command: &str,
    silent: &bool,
) -> Result<Option<String>> {
    let path = std::env::var("PATH").unwrap_or_else(|_| "/bin:/usr/bin".to_string());
    let delimiter = ":";
    let paths: Vec<_> = path.split(delimiter).collect();

    let command_path = Path::new(command);
    if command_path.is_absolute() || command_path.exists() {
        let full_path = std::fs::canonicalize(command_path)?;
        if !*silent {
            println!("{}", full_path.display());
        }
        return Ok(Some(full_path.to_string_lossy().to_string()));
    }

    for path in paths {
        let full_path = format!("{}/{}", path, command);
        if Path::new(&full_path).exists() {
            if !*silent {
                println!("{}", &full_path);
            }
            if !*all_occurrences {
                return Ok(Some(full_path));
            }
        }
    }
    Ok(None)
}
