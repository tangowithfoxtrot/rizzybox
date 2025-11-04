mod cli;
mod command;

use std::{
    env::{self, current_exe},
    fs::File,
    path::PathBuf,
    process::Command,
};

use anyhow::{Context, Result, bail};
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use rizzybox::consts::INSTALLABLE_BINS;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        r#false::false_command,
        ln::ln_command,
        ls::ls_command,
        mkdir::mkdir_command,
        nproc::nproc_command,
        pathmunge::pathmunge_command,
        sh::sh_command,
        sleep::sleep_command,
        stem::stem_command,
        r#true::true_command,
        uname::{arch_command, uname_command},
        which::which_command,
        yes::yes_command,
    },
};

#[expect(clippy::too_many_lines)]
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
        sudo_str = if which_command(false, "doas", true)?.is_some() {
            "doas "
        } else if which_command(false, "sudo", true)?.is_some() {
            "sudo "
        } else {
            bail!(
                "Neither doas nor sudo were found in PATH. Consider running as root with the --install arg instead."
            )
        };
    }

    if cli.install | cli.install_with_sudo {
        println!("# to install rizzybox bins, paste the following in your shell:\n");
        // `export` works with more shells, like fish
        println!(
            "export RIZZYBOX_INSTALL_DIR=/usr/local/bin # change this to the desired installation path"
        );
        for bin in INSTALLABLE_BINS {
            println!(
                "{sudo_str}ln -sf {} $RIZZYBOX_INSTALL_DIR/{bin}",
                std::env::current_exe()
                    .context("rizzybox should exist")?
                    .display(),
            );
        }
        return Ok(());
    }

    if let Some(installation_dir) = cli.install_self {
        let path = current_exe().expect("failed to get path to current executable");
        let binary_name = path.to_str().expect("binary name should be valid unicode");

        // assume that the existence of /.dockerenv means we're running in a container
        if File::open("/.dockerenv").is_ok() {
            // create a dir for the symlinks and add it to PATH so that we don't conflict
            // with any bins that may exist in the image
            let rbin_dir = vec![PathBuf::from(installation_dir.to_string())];
            mkdir_command(rbin_dir, true)?;

            let mut path = std::env::var("PATH")
                .unwrap_or(
                    "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".to_owned(),
                )
                .trim_end_matches(':')
                .to_owned();
            path.push_str(&format!(":{installation_dir}"));
            unsafe { std::env::set_var("PATH", path) };

            for bin in INSTALLABLE_BINS {
                ln_command(
                    true,
                    true,
                    binary_name,
                    &format!("{installation_dir}/{bin}"),
                )?;
            }
            // drop into an interactive shell session
            sh_command()?;
        } else {
            // we're not running in a container, so just create the symlinks
            // where specified
            for bin in INSTALLABLE_BINS {
                ln_command(
                    true,
                    true,
                    binary_name,
                    &format!("{installation_dir}/{bin}"),
                )?;
            }
        }
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
                    bail!(
                        "Couldn't automatically detect the shell. Run `{} completions --help` for more info.",
                        std::env::args().collect::<Vec<String>>()[0]
                    );
                };
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();
                clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
            }
            Commands::Debug { command } | Commands::Rebug { command } => {
                let path = current_exe().expect("failed to get path to current executable");
                let binary_name = path.to_str().expect("binary name should be valid unicode");
                let container_engine = if which_command(false, "docker", true)?.is_some() {
                    "docker"
                } else if which_command(false, "podman", true)?.is_some() {
                    "podman"
                } else {
                    bail!(
                        "No container engine was detected. Please ensure that one of (docker, podman) are available in PATH"
                    )
                };

                let container_command = if command.len() > 1 {
                    command // pass the user's `command` as an arg to rizzybox
                } else {
                    // invoke --install-self and drop into an interactive shell
                    vec![
                        command.iter().map(|c| c.to_owned()).collect(),
                        "--install-self".to_owned(),
                        "/tmp/rbin".to_owned(),
                    ]
                };

                Command::new(container_engine)
                    .args([
                        "run",
                        "--rm",
                        "-it",
                        "--user", // we need to be root to ensure symlinks can be created in /bin
                        "0:0",
                        "-v",
                        &format!("{binary_name}:/bin/rizzybox"),
                        "--entrypoint=/bin/rizzybox",
                    ])
                    .args(container_command)
                    .status()?;
            }
            Commands::Dirname { name, zero } => {
                dirname_command(&name, zero);
            }
            Commands::DockerCliPluginMetadata {} => {
                println!(
                    r#"{{ "SchemaVersion": "0.1.0", "Vendor": "If You're Reading This, You Shouldn't Be", "Version": "{VERSION}", "ShortDescription": "Poor folks' docker-debug" }}"#
                );
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
            Commands::Ln {
                force,
                symlink,
                source,
                destination,
            } => {
                ln_command(force, symlink, &source, &destination)?;
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

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use rizzybox::TestCleanup;
    use std::{
        env::{self},
        os::unix::fs::symlink,
        path::PathBuf,
    };

    /// tests the ability to invoke `rizzybox COMMAND` as `COMMAND` directly
    #[test]
    fn test_argshift() {
        // // FIXME: this is yikes, but I think running the tests in QEMU is
        // // preventing us from being able to create the symlinks we need
        // if cfg!(target_os = "linux")
        //     && std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() != "x86_64"
        // {
        //     eprintln!("Skipping test on non-x86_64 Linux");
        //     return;
        // }

        // Arrange
        let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let rizzybox_path = PathBuf::from(rizzybox_cmd.get_program());

        let temp_dir = env::temp_dir();
        let symlink_path = temp_dir.join("echo");
        let _ = symlink(rizzybox_path, &symlink_path);
        let symlinked_bin = symlink_path.to_string_lossy().to_string();

        let _cleanup = TestCleanup {
            file: Some(symlinked_bin.clone()),
        };

        // Act
        let mut cmd = Command::new(&symlinked_bin);
        cmd.arg("using `echo` like a normal binary B-)");

        // Assert
        cmd.assert().success();
        cmd.assert()
            .stdout("using `echo` like a normal binary B-)\n");
    }
}
