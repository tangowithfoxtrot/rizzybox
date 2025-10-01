use std::io::{stdin, IsTerminal};

use crate::cli::PathmungeCommand;

struct PathEnv {
    paths: Vec<String>,
}

impl std::fmt::Display for PathEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.paths.join(":"))
    }
}

impl Default for PathEnv {
    fn default() -> Self {
        let stdin = stdin();
        let path = if stdin.is_terminal() {
            std::env::var("PATH").unwrap_or("/bin:/sbin:/usr/bin:/usr/sbin".to_owned())
        } else {
            let mut path = String::new();
            let _ = stdin
                .read_line(&mut path)
                .expect("reading from stdin should not fail");
            path
        };
        let path_vec: Vec<String> = path.trim().split(':').map(|s| s.to_string()).collect();
        Self { paths: path_vec }
    }
}

pub fn pathmunge_command(command: PathmungeCommand) {
    let mut path_env = PathEnv::default();

    match command {
        PathmungeCommand::After { path: upath, force } => {
            if path_env.paths.contains(&upath) && !upath.is_empty() {
                if force {
                    path_env.paths.retain(|path| path != &upath);
                    let last = path_env.paths.len();
                    path_env.paths.insert(last, upath);
                }

                println!("{path_env}");
            } else {
                println!("{path_env}:{upath}");
            }
        }
        PathmungeCommand::Before { path: upath, force } => {
            if path_env.paths.contains(&upath) && !upath.is_empty() {
                if force {
                    path_env.paths.retain(|path| path != &upath);
                    path_env.paths.insert(0, upath);
                }

                println!("{path_env}");
            } else {
                println!("{upath}:{path_env}");
            }
        }
        PathmungeCommand::Delete { path: upath } => {
            path_env.paths.retain(|path| path != &upath);
            println!("{path_env}");
        }
    }
}
