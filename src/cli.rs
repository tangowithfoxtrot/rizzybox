use std::path::PathBuf;

use clap::{
    builder::{
        styling::{AnsiColor, Effects, Style},
        Styles,
    },
    ArgAction, Parser, Subcommand,
};
use clap_complete::Shell;
use rizzybox::parse_kv_pair;

use crate::command::uname::IsaFormat;

// https://github.com/crate-ci/clap-cargo/blob/master/src/style.rs
const CARGO_STYLING: Styles = Styles::styled()
    .error(ERROR)
    .header(HEADER)
    .invalid(INVALID)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .usage(USAGE)
    .valid(VALID);

const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);

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
"#,
styles = CARGO_STYLING
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// print install script
    #[arg(long)]
    pub install: bool,

    /// print install script for sudo
    #[arg(long)]
    pub install_with_sudo: bool,

    /// list included binaries
    #[arg(long)]
    pub list: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display machine architecture
    Arch {},

    /// Print NAME with any leading directory components removed
    Basename {
        /// support multiple arguments and treat each as a NAME
        #[arg(long = "multiple", short = 'a', visible_short_alias = 'm')]
        multiple: bool,

        /// the NAME of the directory to use
        name: Vec<String>,

        #[arg(long, short, help = "remove a trailing SUFFIX; implies -a")]
        suffix: Option<String>,

        /// end each output line with NUL, not newline
        #[arg(long, short, visible_short_alias = '0')]
        zero: bool,
    },

    /// Concatenate and print file contents
    Cat {
        /// file to concatenate
        file: Vec<String>,

        /// language to use for syntax highlighting
        #[arg(long, short, default_value = "txt")]
        language: String,

        /// theme to use for colored output
        #[arg(long, short, default_value = "Dracula")]
        theme: String,

        /// show non-printable characters
        #[arg(long, short = 'A', default_value = "false")]
        show_all: bool,

        /// list available themes
        #[arg(long)]
        list_themes: bool,

        /// number all output lines
        #[arg(long, short)]
        number_lines: bool,
    },

    /// Clear the terminal screen
    Clear {},

    /// Generate completions for your shell
    Completions {
        /// the shell to generate completions for
        shell: Option<Shell>,
    },

    /// Output each NAME with its last non-slash component and trailing slashes
    /// removed; if NAME contains no /'s, output '.' (meaning the current directory).
    #[clap(verbatim_doc_comment)]
    Dirname {
        #[arg(required = true)]
        name: Vec<String>,

        /// end each output line with NUL, not newline
        #[arg(long, short, visible_short_alias = '0')]
        zero: bool,
    },

    /// Write arguments to standard output
    Echo {
        /// disable interpretation of backslash escapes
        #[arg(short = 'E', default_value_t = true)]
        disable_backslash_escapes: bool,

        /// enable interpretation of backslash escapes
        #[arg(short = 'e', default_value_t = false)]
        enable_backslash_escapes: bool,

        /// language to use for syntax highlighting
        #[arg(long, short, default_value = "txt")]
        language: String,

        /// do not output a trailing newline
        #[arg(long, short, default_value_t = false)]
        nonewline: bool,

        #[arg(default_value = "")]
        string: Vec<String>,

        /// theme to use for colored output
        #[arg(long, short, default_value = "Dracula")]
        theme: String,
    },

    /// Run a program in a modified environment
    Env {
        /// pass ARG as the zeroth argument of COMMAND
        #[arg(long, short)]
        argv0: Option<String>,

        /// change working directory to DIR
        #[arg(long, short)]
        chdir: Option<String>,

        /// start with an empty environment
        #[arg(long, short = 'i')]
        ignore_environment: bool,

        /// end echo output line with NUL, not newline
        #[arg(long, short = '0', visible_alias = "zero", visible_short_alias = '0')]
        null: bool,

        /// remove variable from the environment
        #[arg(action = ArgAction::Append, long, short)]
        unset: Vec<String>,

        /// KEY=VALUE to set in the environment
        #[arg(value_parser = parse_kv_pair)]
        kv_pair: Vec<String>,

        /// command to run in the environment
        #[arg(last = true)]
        command: Vec<String>,
        // FIXME: `last` requires the command to be passed via `--`, which differs from coreutils env
    },

    ///  Convert tabs in each FILE to spaces, writing to standard output
    Expand {
        /// file to concatenate
        #[arg(default_value = "-")]
        file: String,

        /// have tabs N characters apart, not 8
        #[arg(long, short, value_name = "N,LIST", value_delimiter = ',', num_args = 1..)]
        tabs: Vec<String>,
    },

    /// Do nothing and exit with a failure status
    False {},

    /// List information about the FILEs (the current directory by default)
    Ls {
        /// do not ignore entries starting with '.'
        #[arg(long, short)]
        all: bool,

        /// the PATH to list
        #[arg(default_value = ".", hide_default_value = true)]
        path: String,
    },

    /// Create directories if they do not already exist
    Mkdir {
        /// directories to create
        #[arg(required = true)]
        dirs: Vec<PathBuf>,

        /// create parent directories as needed
        #[arg(long, short, env = "RZ_MKDIR_PARENTS")]
        parents: bool,
    },

    /// Print the number of cores available to the current process
    Nproc {
        /// print the number of cores available to the system
        #[arg(long, short)]
        all: bool,

        /// ignore up to N cores
        #[arg(
            long,
            short,
            value_name = "N",
            hide_default_value = true,
            default_value_t = 0
        )]
        ignore: usize,

        /// maximum threads to report
        #[arg(long, env = "OMP_NUM_LIMIT")]
        omp_num_limit: Option<usize>,

        /// minimum threads to report
        #[arg(long, env = "OMP_NUM_THREADS")]
        omp_num_threads: Option<usize>,
    },

    /// An incomplete shell
    Sh {},

    /// Pause for NUMBER of seconds
    Sleep {
        /// NUMBER of seconds to sleep
        number: String,
    },

    /// Reduce word(s) to their stem(s)
    #[clap(disable_help_subcommand = true)] // someone may pass 'help' as a word
    Stem {
        /// do not output a trailing newline
        #[arg(long, short, default_value_t = false)]
        nonewline: bool,

        /// words that you would like to stem
        words: Vec<String>,
    },

    /// Do nothing and exit with a success status
    True {},

    /// Print system information
    Uname {
        /// print all information
        #[arg(long, short, default_value_t = false)]
        all: bool,

        ///print the kernel name
        #[arg(long, short = 's')]
        kernel: bool,

        /// print the network node hostname
        #[arg(long, short = 'n', default_value_t = false)]
        nodename: bool,

        /// print the kernel release
        #[arg(long = "kernel-release", short = 'r', default_value_t = false)]
        kernel_release: bool,

        /// print the kernel version
        #[arg(long = "kernel-version", short = 'v', default_value_t = false)]
        kernel_version: bool,

        /// print the machine hardware name
        #[arg(long, short, default_value_t = false)]
        machine: bool,

        /// print the operating system
        #[arg(long, short, default_value_t = false)]
        operating_system: bool,

        /// the ISA format to use for CPU info
        #[arg(long, short, default_value_t)]
        isa_format: IsaFormat,
    },

    /// Write the full path of COMMAND to standard output
    /* TODO: GNU which accepts multiple commands */
    Which {
        /// print all matching pathnames of each argument
        #[arg(short, default_value_t = false)]
        all_occurrences: bool,

        /// command to search for in PATH
        command: String,

        /// silently return 0 if all of the executables were found or 1 otherwise
        #[arg(short, default_value_t = false)]
        silent: bool,
    },

    /// Repeatedly output lines with TEXT
    #[clap(disable_help_subcommand = true)] // in case someone passes 'help' as a word
    Yes {
        /// AMOUNT of TEXT to output
        #[arg(long, short, default_value = "0", group = "yes_group")]
        amount: usize,

        /// output TEXT for a DURATION in seconds
        #[arg(long, short, group = "yes_group")]
        duration: Option<f32>, // Optional because ArgGroup requiring one or the other Arg from the group

        #[arg(default_value = "y")]
        text: String,
    },
}
