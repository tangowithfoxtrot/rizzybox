use anyhow::Result;
use clap::Parser;
use std::{
    env,
    io::{stdin, stdout, Write},
    process::Command,
};

mod builtins {
    use clap::Parser;

    #[derive(Parser)]
    /// Change the shell working directory
    pub struct CdCommand {
        /// Directory to change to
        #[clap(default_value = "$HOME")]
        pub dir: String,
    }

    #[derive(Parser)]
    /// Exit the shell
    pub struct ExitCommand {
        /// Exit code
        #[clap(default_value = "0")]
        pub code: i32,
    }

    #[derive(Parser)]
    /// Print the name of the current working directory.
    pub struct PwdCommand {
        /// Print the value of $PWD if it names the current working directory
        #[clap(short = 'L', long, default_value = "true")]
        pub logical: bool,
        /// Print the name of the current working directory
        #[clap(short = 'P', long)]
        pub physical: bool,
    }
}

pub fn sh_command() -> Result<()> {
    let mut already_prompted = false;
    loop {
        render_prompt(already_prompted);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap_or(0);
        let input = input.trim();

        if input.is_empty() {
            already_prompted = true;
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "cd" => match parse_command::<builtins::CdCommand>("cd", args) {
                Ok(cmd) => {
                    let target_dir = match cmd.dir.as_str() {
                        "$HOME" | "~" => env::var("HOME").map_err(|_| "HOME not set"),
                        "-" => env::var("OLDPWD").map_err(|_| "OLDPWD not set"),
                        "." => Ok(".".to_string()), // Stay in current directory
                        ".." => Ok("..".to_string()),
                        _ => Ok(cmd.dir.clone()),
                    };

                    match target_dir {
                        Ok(dir) => {
                            // Store current directory as OLDPWD before changing directories
                            if let Ok(current) = env::current_dir() {
                                if let Some(path) = current.to_str() {
                                    env::set_var("OLDPWD", path);
                                }
                            }

                            if let Err(e) = env::set_current_dir(&dir) {
                                eprintln!("cd: {}: {}", dir, e);
                            }
                        }
                        Err(msg) => eprintln!("cd: {}", msg),
                    }
                }
                Err(e) => eprintln!("{}", e),
            },
            "exit" => match parse_command::<builtins::ExitCommand>("exit", args) {
                Ok(cmd) => std::process::exit(cmd.code),
                Err(_) => return Ok(()),
            },
            "pwd" => match parse_command::<builtins::PwdCommand>("pwd", args) {
                Ok(cmd) => {
                    let current_dir =
                        env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let path = current_dir.to_str().unwrap_or(".");
                    if cmd.logical {
                        println!("{}", path);
                    } else if cmd.physical {
                        // Get the physical path
                        let physical_path = std::fs::canonicalize(current_dir)
                            .unwrap_or_else(|_| std::path::PathBuf::from("."));
                        println!("{}", physical_path.display());
                    } else {
                        // Default behavior
                        println!("{}", path);
                    }
                }
                Err(e) => eprintln!("{}", e),
            },
            _ => {
                // Handle external commands
                match Command::new(command).args(args).spawn() {
                    Ok(mut child) => {
                        child.wait()?;
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::NotFound {
                            eprintln!("{}: command not found", command);
                        } else {
                            eprintln!("{}: {}", command, e);
                        }
                    }
                }
            }
        }
    }
}

fn render_prompt(already_prompted: bool) {
    let prompt = std::env::var("PS1").unwrap_or_else(|_| "λ ".to_owned());
    if already_prompted {
        eprint!("\n\r");
    }
    eprintln!();
    if rustix::process::geteuid().is_root() {
        eprint!("# ");
    } else {
        eprint!("{prompt}");
    }

    let _ = stdout().flush();
}

fn parse_command<T: Parser>(cmd_name: &str, args: &[&str]) -> Result<T, clap::Error> {
    T::try_parse_from(std::iter::once(cmd_name).chain(args.iter().copied()))
}
