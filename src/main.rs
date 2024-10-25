use std::env;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

use crate::command::{
    basename::*, cat::*, clear::*, echo::*, env::*, r#false::*, r#true::*, uname::*, which::*,
    yes::*,
};

mod command;

/// Binaries that can be installed with --install
/// Example: ln -sf /full/path/to/rizzybox /usr/local/bin/cat
const INSTALLABLE_BINS: [&str; 11] = [
    "arch", "basename", "cat", "clear", "echo", "env", "false", "true", "uname", "which", "yes",
];

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

    #[arg(long, help = "print install script")]
    install: bool,

    #[arg(long, help = "list included binaries")]
    list: bool,
}

#[derive(Subcommand)]
enum Commands {
    Arch {},
    Basename {
        #[arg(
            long = "multiple",
            short = 'a',
            help = "support multiple arguments and treat each as a NAME"
        )]
        multiple: bool,
        name: Vec<String>,
        #[arg(long, short, help = "remove a trailing SUFFIX; implies -a")]
        suffix: Option<String>,
        #[arg(long, short, help = "end each output line with NUL, not newline")]
        zero: bool,
    },
    Cat {
        file: Vec<String>,
        #[arg(
            long,
            short,
            default_value = "txt",
            help = "language to use for syntax highlighting"
        )]
        language: String,
        #[arg(
            long,
            short,
            default_value = "Dracula",
            help = "theme to use for colored output"
        )]
        theme: String,
    },
    Clear {},
    Completions {
        shell: Option<Shell>,
    },
    Echo {
        #[arg(
            short = 'E',
            default_value_t = true,
            help = "disable interpretation of backslash escapes"
        )]
        disable_backslash_escapes: bool,
        #[arg(
            short = 'e',
            default_value_t = false,
            help = "enable interpretation of backslash escapes"
        )]
        enable_backslash_escapes: bool,
        #[arg(
            long,
            short,
            default_value = "txt",
            help = "language to use for syntax highlighting"
        )]
        language: String,
        #[arg(
            long,
            short,
            default_value_t = false,
            help = "do not output the trailing newline"
        )]
        nonewline: bool,
        #[arg(default_value = "")]
        string: String,
        #[arg(
            long,
            short,
            default_value = "Dracula",
            help = "theme to use for colored output"
        )]
        theme: String,
    },
    Env {},
    False {},
    True {},
    Uname {
        #[arg(long, short, default_value_t = false, help = "print all information")]
        all: bool,
        #[arg(
            long,
            short = 's',
            default_value_t = true,
            help = "print the kernel name"
        )]
        kernel: bool,
        #[arg(
            long,
            short = 'n',
            default_value_t = false,
            help = "print the network node hostname"
        )]
        nodename: bool,
        #[arg(
            long = "kernel-release",
            short = 'r',
            default_value_t = false,
            help = "print the kernel release"
        )]
        kernel_release: bool,
        #[arg(
            long = "kernel-version",
            short = 'v',
            default_value_t = false,
            help = "print the kernel version"
        )]
        kernel_version: bool,
        #[arg(
            long,
            short,
            default_value_t = false,
            help = "print the machine hardware name"
        )]
        machine: bool,
        #[arg(
            long,
            short,
            default_value_t = false,
            help = "print the operating system"
        )]
        operating_system: bool,
    },
    Which {
        #[arg(
            short,
            default_value_t = false,
            help = "print all matching pathnames of each argument"
        )]
        all_occurrences: bool,
        bin: String,
        #[arg(
            short,
            default_value_t = false,
            help = "silently return 0 if all of the executables were found or 1 otherwise"
        )]
        silent: bool,
    },
    Yes {
        #[arg(default_value = "y")]
        text: String,
    },
}

fn main() -> Result<()> {
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
                "ln -sf {} $RIZZYBOX_INSTALL_DIR/{}",
                std::env::current_exe()
                    .context("rizzybox should exist")?
                    .display(),
                bin,
            )
        }
        return Ok(());
    }

    if cli.list {
        for bin in INSTALLABLE_BINS.iter() {
            print!("{bin} ");
        }
        println!();
    }

    if let Some(command) = cli.command {
        match command {
            Commands::Arch {} => {
                arch_command()?;
            }
            Commands::Basename {
                multiple,
                name,
                suffix,
                zero,
            } => {
                basename_command(&multiple, &name, &suffix, &zero)?;
            }
            Commands::Cat {
                file,
                language,
                theme,
            } => {
                cat_command(&file, &language, &theme)?;
            }
            Commands::Clear {} => clear_command()?,
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
                string: text,
                theme,
            } => {
                echo_command(
                    &disable_backslash_escapes,
                    &enable_backslash_escapes,
                    &language,
                    &nonewline,
                    &text,
                    &theme,
                )?;
            }
            Commands::Env {} => {
                env_command()?;
            }
            Commands::False {} => false_command()?,

            Commands::True {} => {
                true_command()?;
            }
            Commands::Uname {
                all,
                kernel,
                nodename,
                kernel_release,
                kernel_version,
                machine,
                operating_system,
            } => {
                uname_command(
                    &all,
                    &kernel,
                    &nodename,
                    &kernel_release,
                    &kernel_version,
                    &machine,
                    &operating_system,
                )?;
            }
            Commands::Which {
                all_occurrences,
                bin,
                silent,
            } => {
                which_command(&all_occurrences, &bin, &silent)?;
            }
            Commands::Yes { text } => {
                yes_command(&text)?;
            }
        }
    };
    Ok(())
}
