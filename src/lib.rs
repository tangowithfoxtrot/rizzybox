use std::{fs::remove_file, string::String};

#[derive(Debug, Clone)]
pub struct UserGroupPair {
    pub user: String,
    pub group: Option<String>,
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
