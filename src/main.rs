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

/// Binaries that can be installed with --install
/// Example: ln -sf /full/path/to/rizzybox /usr/local/bin/cat
const INSTALLABLE_BINS: [&str; 5] = ["cat", "echo", "env", "which", "yes"];

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = r#"

8888888b.  d8b                            888888b.
888   Y88b Y8P                            888  "88b
888    888                                888  .88P
888   d88P 888 88888888 88888888 888  888 8888888K.   .d88b.  888  888
8888888P"  888    d88P     d88P  888  888 888  "Y88b d88""88b `Y8bd8P'
888 T88b   888   d88P     d88P   888  888 888    888 888  888   X88K
888  T88b  888  d88P     d88P    Y88b 888 888   d88P Y88..88P .d8""8b.
888   T88b 888 88888888 88888888  "Y88888 8888888P"   "Y88P"  888  888
                                      888
                                 Y8b d88P
                                  "Y88P"
"#
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long)]
    install: bool,
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
    let args = if INSTALLABLE_BINS
        .iter()
        .any(|&bin| binary_name.ends_with(bin))
    {
        // shift binary name to subcommand name
        let subcommand_name = binary_name.split('/').last().unwrap_or(binary_name);
        let mut new_args = vec![binary_name.to_string(), subcommand_name.to_string()];
        new_args.extend(args.into_iter().skip(1));
        new_args
    } else {
        args
    };

    let cli = Cli::parse_from(args.clone());
    // TODO: write actual install logic instead of providing a script
    if cli.install {
        println!("# to install rizzybox bins, paste the following in your shell:\n");
        // `export` works with more shells, like fish
        println!("export RIZZYBOX_INSTALL_DIR=/usr/local/bin # change this to the desired installation path");
        for bin in INSTALLABLE_BINS.iter() {
            println!(
                "ln -sf {}/{} $RIZZYBOX_INSTALL_DIR/{}",
                std::env::current_dir().unwrap().display(),
                args.first().unwrap(),
                bin,
            )
        }
    }

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
