use std::{fs::remove_file, string::String};
pub mod consts {
    /// Binaries that can be installed with --install
    /// Example: ln -sf /full/path/to/rizzybox /usr/local/bin/cat
    pub const INSTALLABLE_BINS: [&str; 11] = [
        "arch", "basename", "cat", "clear", "echo", "env", "false", "true", "uname", "which", "yes",
    ];
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
