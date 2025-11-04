use anyhow::Result;
use clap::Parser;
use std::{
    env,
    io::{Write, stdin, stdout},
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
            "cd" => cd_command(args),
            "exit" => {
                if let Some(value) = exit_command(args) {
                    return value;
                }
            }
            "pwd" => pwd_command(args),
            "(" => {
                eprintln!("opening paren");
            }
            ")" => {
                eprintln!("closing paren");
            }
            _ => run_command(command, args)?,
        }
    }
}

fn run_command(command: &str, args: &[&str]) -> Result<(), anyhow::Error> {
    let _: () = match Command::new(command).args(args).spawn() {
        Ok(mut child) => {
            child.wait()?;
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("{command}: command not found");
            } else {
                eprintln!("{command}: {e}");
            }
        }
    };
    Ok(())
}

fn cd_command(args: &[&str]) {
    match parse_command::<builtins::CdCommand>("cd", args) {
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
                    if let Ok(current) = env::current_dir()
                        && let Some(path) = current.to_str()
                    {
                        unsafe { env::set_var("OLDPWD", path) };
                    }

                    if let Err(e) = env::set_current_dir(&dir) {
                        eprintln!("cd: {dir}: {e}");
                    }
                }
                Err(msg) => eprintln!("cd: {msg}"),
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}

fn exit_command(args: &[&str]) -> Option<std::result::Result<(), anyhow::Error>> {
    match parse_command::<builtins::ExitCommand>("exit", args) {
        Ok(cmd) => std::process::exit(cmd.code),
        Err(_) => Some(Ok(())),
    }
}

fn pwd_command(args: &[&str]) {
    match parse_command::<builtins::PwdCommand>("pwd", args) {
        Ok(cmd) => {
            let current_dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let path = current_dir.to_str().unwrap_or(".");
            if cmd.logical {
                println!("{path}");
            } else if cmd.physical {
                // Get the physical path
                let physical_path = std::fs::canonicalize(current_dir)
                    .unwrap_or_else(|_| std::path::PathBuf::from("."));
                println!("{}", physical_path.display());
            } else {
                // Default behavior
                println!("{path}");
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}

fn render_prompt(already_prompted: bool) {
    let prompt = std::env::var("PS1").unwrap_or("λ ".to_owned());
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
