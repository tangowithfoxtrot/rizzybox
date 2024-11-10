use std::{fs::remove_file, string::String};

pub mod consts {
    /// Binaries that can be installed with --install
    /// Example: ln -sf /full/path/to/rizzybox /usr/local/bin/cat
    pub const INSTALLABLE_BINS: [&str; 15] = [
        "arch", "basename", "cat", "clear", "dirname", "echo", "env", "expand", "false", "ls",
        "sh", "true", "uname", "which", "yes",
    ];
}

/// A simple error handler with formatting
pub fn handle_error<T>(result: Result<T, std::io::Error>, message: &str) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            eprintln!(
                "{}: {}: {}",
                std::env::current_exe().unwrap().display(),
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
        Err(format!("Invalid key-value pair: {}", s))
    }
}

/// TestCleanup is a struct that implements the Drop trait to run cleanup code when it goes out of scope.
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
            remove_file(file).expect("failed to remove file");
        }
    }
}
