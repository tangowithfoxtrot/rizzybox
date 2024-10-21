use std::env;

use cat_command::cat_command;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use echo_command::echo_command;
use env_command::env_command;
use which_command::which_command;
use yes_command::yes_command;

mod cat_command;
mod echo_command;
mod env_command;
mod which_command;
mod yes_command;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Cat {
        file: Vec<String>,
        #[arg(long = "language", short = 'l', default_value = "txt")]
        language: String,
        #[arg(long = "theme", short = 't', default_value = "Dracula")]
        theme: String,
    },
    Completions {
        shell: Option<Shell>,
    },
    Echo {
        #[arg(short = 'E', default_value_t = true)]
        disable_backslash_escapes: bool,
        #[arg(short = 'e', default_value_t = false)]
        enable_backslash_escapes: bool,
        #[arg(long = "language", short = 'l', default_value = "txt")]
        language: String,
        #[arg(long = "nonewline", short = 'n', default_value_t = false)]
        nonewline: bool,
        #[arg(default_value = "")]
        text: String,
        #[arg(long = "theme", short = 't', default_value = "Dracula")]
        theme: String,
    },
    Env {},
    Which {
        #[arg(short = 'a', default_value_t = false)]
        all_occurrences: bool,
        bin: String,
        #[arg(short = 's', default_value_t = false)]
        silent: bool,
    },
    Yes {
        #[arg(default_value = "y")]
        text: String,
    },
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binary_name = args.first().map(|s| s.as_str()).unwrap_or_default();

    // determine if invoked as subcommand directly: `/bin/echo`
    let args = if binary_name.ends_with("cat")
        || binary_name.ends_with("echo")
        || binary_name.ends_with("env")
        || binary_name.ends_with("which")
        || binary_name.ends_with("yes")
    {
        // shift binary name to subcommand name
        let subcommand_name = binary_name.split('/').last().unwrap_or(binary_name);
        let mut new_args = vec![binary_name.to_string(), subcommand_name.to_string()];
        new_args.extend(args.into_iter().skip(1));
        new_args
    } else {
        args
    };

    let cli = Cli::parse_from(args);

    if let Some(command) = cli.command {
        match command {
            Commands::Cat {
                file,
                language,
                theme,
            } => {
                cat_command(&file, &language, &theme);
            }
            Commands::Completions { shell } => {
                // FIXME: this probably won't work when commands are invoked in their symlinked form (`echo`, `env`, `cat`)
                let Some(shell) = shell.or_else(Shell::from_env) else {
                    eprintln!("Couldn't automatically detect the shell. Run `rizzybox completions --help` for more info.");
                    std::process::exit(1);
                };
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();
                clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
            }
            Commands::Echo {
                disable_backslash_escapes,
                enable_backslash_escapes,
                language,
                nonewline,
                text,
                theme,
            } => {
                echo_command(
                    &disable_backslash_escapes,
                    &enable_backslash_escapes,
                    &language,
                    &nonewline,
                    &text,
                    &theme,
                );
            }
            Commands::Env {} => {
                env_command();
            }
            Commands::Which {
                all_occurrences,
                bin,
                silent,
            } => {
                which_command(&all_occurrences, &bin, &silent);
            }
            Commands::Yes { text } => {
                yes_command(&text);
            }
        }
    }
}
