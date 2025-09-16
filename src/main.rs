mod cli;
mod command;

use std::env;

use anyhow::{bail, Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use rizzybox::consts::INSTALLABLE_BINS;

use {
    cli::{Cli, Commands},
    command::{
        basename::basename_command,
        cat::cat_command,
        clear::clear_command,
        dirname::dirname_command,
        echo::echo_command,
        env::env_command,
        expand::expand_command,
        ls::ls_command,
        mkdir::mkdir_command,
        nproc::nproc_command,
        pathmunge::pathmunge_command,
        r#false::false_command,
        r#true::true_command,
        sh::sh_command,
        sleep::sleep_command,
        stem::stem_command,
        uname::{arch_command, uname_command},
        which::which_command,
        yes::yes_command,
    },
};

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let binary_name = args.first().map(String::as_str).unwrap_or_default();

    // determine if invoked as subcommand directly: `/bin/echo`
    let args = if INSTALLABLE_BINS
        .iter()
        .any(|&bin| binary_name.ends_with(bin))
    {
        // shift binary name to subcommand name
        let subcommand_name = binary_name.split('/').next_back().unwrap_or(binary_name);
        let mut new_args = vec![binary_name.to_string(), subcommand_name.to_string()];
        new_args.extend(args.into_iter().skip(1));
        new_args
    } else {
        args
    };

    let cli = Cli::parse_from(args);

    let mut sudo_str = "";
    if cli.install_with_sudo {
        sudo_str = "sudo ";
    }

    if cli.install | cli.install_with_sudo {
        println!("# to install rizzybox bins, paste the following in your shell:\n");
        // `export` works with more shells, like fish
        println!("export RIZZYBOX_INSTALL_DIR=/usr/local/bin # change this to the desired installation path");
        for bin in INSTALLABLE_BINS {
            println!(
                "{}ln -sf {} $RIZZYBOX_INSTALL_DIR/{}",
                sudo_str,
                std::env::current_exe()
                    .context("rizzybox should exist")?
                    .display(),
                bin,
            );
        }
        return Ok(());
    }

    if cli.list {
        let mut print_str = String::new();
        for bin in INSTALLABLE_BINS {
            print_str.push_str(bin);
            print_str.push(' ');
        }
        println!("{}", print_str.trim_end());
    }

    if let Some(command) = cli.command {
        match command {
            Commands::Arch {} => {
                arch_command();
            }
            Commands::Basename {
                multiple,
                name,
                suffix,
                zero,
            } => {
                basename_command(multiple, &name, suffix.as_ref(), zero);
            }
            Commands::Cat {
                file,
                language,
                theme,
                show_all,
                list_themes,
                number_lines,
            } => {
                cat_command(
                    &file,
                    &language,
                    &theme,
                    show_all,
                    list_themes,
                    number_lines,
                );
            }
            Commands::Clear {} => clear_command(),
            Commands::Completions { shell } => {
                let Some(shell) = shell.or_else(Shell::from_env) else {
                    bail!("Couldn't automatically detect the shell. Run `{} completions --help` for more info.", std::env::args().collect::<Vec<String>>()[0]);
                };
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();
                clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
            }
            Commands::Dirname { name, zero } => {
                dirname_command(&name, zero);
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
                    disable_backslash_escapes,
                    enable_backslash_escapes,
                    &language,
                    nonewline,
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
                    argv0.as_ref(),
                    chdir.as_ref(),
                    &command,
                    ignore_environment,
                    null,
                    &unset,
                    &kv_pair,
                )?;
            }
            Commands::False {} => false_command(),
            Commands::Expand { file, tabs } => {
                expand_command(&file, &tabs)?;
            }
            Commands::Ls { all, path } => {
                ls_command(all, &path)?;
            }
            Commands::Mkdir { dirs, parents } => {
                mkdir_command(dirs, parents)?;
            }
            Commands::Nproc {
                all,
                ignore,
                omp_num_limit,
                omp_num_threads,
            } => nproc_command(all, ignore, omp_num_limit, omp_num_threads),
            Commands::Pathmunge { command } => pathmunge_command(command),
            Commands::Sh {} => {
                sh_command()?;
            }
            Commands::Sleep { number } => {
                sleep_command(&number)?;
            }
            Commands::Stem { nonewline, words } => {
                stem_command(nonewline, &words);
            }
            Commands::True {} => {
                true_command();
            }
            Commands::Uname {
                all,
                kernel,
                nodename,
                kernel_release,
                kernel_version,
                machine,
                operating_system,
                isa_format,
            } => {
                uname_command(
                    all,
                    kernel,
                    nodename,
                    kernel_release,
                    kernel_version,
                    machine,
                    operating_system,
                    isa_format,
                );
            }
            Commands::Which {
                all_occurrences,
                command,
                silent,
            } => {
                let result = which_command(all_occurrences, &command, silent)?;
                if result.is_none() {
                    std::process::exit(1);
                }
            }
            Commands::Yes {
                text,
                amount,
                duration,
            } => {
                yes_command(&text, amount, duration);
            }
        }
    }
    Ok(())
}
