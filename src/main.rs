use std::env;

use anyhow::{Context, Result};
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use rizzybox::{consts::INSTALLABLE_BINS, parse_kv_pair};

use crate::command::{
    basename::*, cat::*, clear::*, dirname::*, echo::*, env::*, expand::*, ls::*, r#false::*,
    r#true::*, sh::*, stem::*, uname::*, which::*, yes::*,
};

mod command;

#[derive(Parser)]
#[command(
    version,
    about = "A multi-call binary containing common coreutils",
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

    #[arg(long, help = "print install script for sudo")]
    install_with_sudo: bool,

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
            help = "support multiple arguments and treat each as a NAME",
            visible_short_alias = 'm'
        )]
        multiple: bool,

        #[arg(help = "print NAME with any leading directory components removed")]
        name: Vec<String>,

        #[arg(long, short, help = "remove a trailing SUFFIX; implies -a")]
        suffix: Option<String>,

        #[arg(
            long,
            short,
            help = "end each output line with NUL, not newline",
            visible_short_alias = '0'
        )]
        zero: bool,
    },
    Cat {
        #[arg(help = "file to concatenate")]
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
        #[arg(help = "the shell to generate completions for")]
        shell: Option<Shell>,
    },
    #[command(
        about = "Output each NAME with its last non-slash component and trailing slashes
removed; if NAME contains no /'s, output '.' (meaning the current directory)."
    )]
    Dirname {
        #[arg(help = "strip last component from file name", required = true)]
        name: Vec<String>,

        #[arg(
            long,
            short,
            help = "end each output line with NUL, not newline",
            visible_short_alias = '0'
        )]
        zero: bool,
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
        string: Vec<String>,

        #[arg(
            long,
            short,
            default_value = "Dracula",
            help = "theme to use for colored output"
        )]
        theme: String,
    },
    #[command(about = "run a program in a modified environment")]
    Env {
        #[arg(long, short, help = "pass ARG as the zeroth argument of COMMAND")]
        argv0: Option<String>,

        #[arg(long, short, help = "change working directory to DIR")]
        chdir: Option<String>,

        #[arg(
            action = clap::ArgAction::SetTrue,
            long,
            short = 'i',
            help = "start with an empty environment",
        )]
        ignore_environment: bool,

        #[arg(
            long,
            short = '0',
            help = "end echo output line with NUL, not newline",
            visible_alias = "zero",
            visible_short_alias = '0'
        )]
        null: bool,

        #[arg(action = ArgAction::Append, long, short, help = "remove variable from the environment")]
        unset: Vec<String>,

        #[arg(help = "KEY=VALUE to set in the environment", value_parser = parse_kv_pair)]
        kv_pair: Vec<String>,

        #[arg(help = "command to run in the environment", last = true)]
        // FIXME: this requires the command to be passed via `--`, which differs from coreutils env
        command: Vec<String>,
    },
    #[command(about = "Convert tabs in each FILE to spaces, writing to standard output.")]
    Expand {
        #[arg(default_value = "-", help = "file to concatenate")]
        file: String,

        #[arg(long, short, value_name = "N,LIST", value_delimiter = ',', num_args = 1.., help = "have tabs N characters apart, not 8")]
        tabs: Option<Vec<String>>,
    },
    False {},
    #[command(about = "List information about the FILEs (the current directory by default).")]
    Ls {
        #[arg(short, long, help = "do not ignore entries starting with '.'")]
        all: bool,

        #[arg(help = "path to list", default_value = ".")]
        path: String,
    },
    #[command(about = "A shell.")]
    Sh {},
    #[command(about = "Reduce a word to its stem")]
    Stem {
        string: String,
    },
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

        #[arg(help = "command to search for in PATH")]
        command: String,

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

    let cli = Cli::parse_from(args);
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

    if cli.install_with_sudo {
        println!("# to install rizzybox bins, paste the following in your shell:\n");
        println!("export RIZZYBOX_INSTALL_DIR=/usr/local/bin # change this to the desired installation path");
        for bin in INSTALLABLE_BINS.iter() {
            println!(
                "sudo ln -sf {} $RIZZYBOX_INSTALL_DIR/{}",
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
            Commands::Dirname { name, zero } => {
                dirname_command(&name, &zero)?;
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
            Commands::Env {
                argv0,
                chdir,
                command,
                ignore_environment,
                null,
                unset,
                kv_pair,
            } => {
                env_command(
                    &argv0,
                    &chdir,
                    &command,
                    &ignore_environment,
                    &null,
                    &unset,
                    &kv_pair,
                )?;
            }
            Commands::False {} => false_command()?,
            Commands::Expand { file, tabs } => {
                expand_command(&file, &tabs)?;
            }
            Commands::Ls { all, path } => {
                ls_command(&all, &path)?;
            }
            Commands::Sh {} => {
                sh_command()?;
            }
            Commands::Stem { string } => {
                stem_command(&string)?;
            }
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
                command,
                silent,
            } => {
                let result = which_command(&all_occurrences, &command, &silent)?;
                if result.is_none() {
                    std::process::exit(1);
                }
            }
            Commands::Yes { text } => {
                yes_command(&text)?;
            }
        }
    };
    Ok(())
}
