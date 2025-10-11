pub fn ln_command(force: bool, symlink: bool, source: &str, destination: &str) {
    if force {
        // TODO: --force is not implemented yet, but we don't want to panic if the user passes -f
    }

    if symlink {
        let symlink_result = std::os::unix::fs::symlink(source, destination);
        match symlink_result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to create symlink: {e}");
                std::process::exit(1);
            }
        }
    } else {
        todo!("Creating hardlinks is not supported yet...");
    }
}
