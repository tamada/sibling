use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::{LogLevel, minisib};

#[derive(Debug, Parser)]
#[clap(version, author, about, arg_required_else_help = true)]
pub struct CliOpts {
    #[clap(flatten)]
    pub(crate) p_opts: PrintingOpts,

    #[clap(flatten)]
    pub(crate) nexter_opts: NexterOpts,

    #[clap(flatten)]
    pub(crate) init_script: InitOpts,

    #[clap(flatten)]
    pub(crate) log_opts: LogOpts,

    #[arg(
        short = 'w',
        long = "working-dir",
        help = "set the current working directory.",
        hide = true,
        value_name = "DIR",
        long_help = "This option is applied before any other processing. Therefore, other options that specify paths should use the relative path from this option value. If this option is not specified, the current directory is used."
    )]
    pub(crate) cwd: Option<PathBuf>,

    #[arg(index = 1, help = "the target directory", value_name = "DIR")]
    pub dirs: Vec<PathBuf>,

    #[cfg(debug_assertions)]
    #[clap(flatten)]
    pub(crate) compopts: CompletionOpts,

    #[clap(subcommand)]
    pub(crate) minisib: Option<minisib::MiniSibCommand>,
}

impl CliOpts {
    pub fn init(&mut self) {
        self.log_opts.init();
        if let Some(cwd) = &self.cwd && let Err(e) = std::env::set_current_dir(cwd) {
            log::error!("Failed to set current directory to {cwd:?}: {e}, use \".\"");
        }
        if self.dirs.is_empty() {
            self.dirs.push(std::env::current_dir().unwrap());
        }
    }
}

#[derive(Parser, Debug)]
pub(crate) struct LogOpts {
    #[arg(
        long,
        help = "set the log level",
        value_enum,
        default_value_t = LogLevel::Warn,
        value_name = "LEVEL",
        ignore_case = true
    )]
    pub log: LogLevel,
}

impl LogOpts {
    pub fn init(&self) {
        use LogLevel::{Debug, Error, Info, Trace, Warn};
        if std::env::var_os("RUST_LOG").is_none() {
            unsafe {
                match self.log {
                    Error => std::env::set_var("RUST_LOG", "error"),
                    Warn => std::env::set_var("RUST_LOG", "warn"),
                    Info => std::env::set_var("RUST_LOG", "info"),
                    Debug => std::env::set_var("RUST_LOG", "debug"),
                    Trace => std::env::set_var("RUST_LOG", "trace"),
                };
            }
        }
        env_logger::init();
        log::info!("Log level set to {:?}", self.log);
    }
}

#[derive(Parser, Debug)]
pub(crate) struct InitOpts {
    #[arg(
        long,
        help = "generate the initialize script for the shell",
        value_name = "SHELL",
        hide = true,
        default_missing_value = "bash"
    )]
    pub init: Option<String>,
}

#[derive(Parser, Debug)]
pub(crate) struct NexterOpts {
    #[arg(
        short,
        long,
        help = "specify the number of times to execute sibling",
        value_name = "COUNT",
        default_value_t = 1
    )]
    pub step: i32,

    #[arg(short = 't', long = "type", help = "specify the nexter type", value_enum, default_value_t = sibling::NexterType::Next, value_name = "TYPE", ignore_case = true)]
    pub nexter: sibling::NexterType,

    #[arg(
        short,
        long,
        help = "directory list from file, if FILE is \"-\", reads from stdin.",
        value_name = "FILE"
    )]
    pub input: Option<String>,

    #[arg(
        short = 'a',
        long,
        help = "Set the targets to all directories from the given list. By default, the sibling skips non-existent directories.",
        default_value_t = false
    )]
    pub all: bool,
}

#[cfg(debug_assertions)]
#[derive(Parser, Debug)]
pub(crate) struct CompletionOpts {
    #[arg(
        long = "generate-completion-files",
        help = "Generate completion files",
        hide = true
    )]
    pub(crate) completion: bool,

    #[arg(
        long = "completion-out-dir",
        value_name = "DIR",
        default_value = "assets/completions",
        help = "Output directory of completion files",
        hide = true
    )]
    pub(crate) dest: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Json,
    Csv,
    List,
    Default,
}

#[derive(Debug, Parser)]
pub(crate) struct PrintingOpts {
    #[arg(
        short, long,
        help = "print the result in the specified format",
        default_value_t = Format::Default,
        value_enum,
    )]
    pub format: Format,

    #[arg(
        short = 'A',
        long,
        help = "print the directory name in the absolute path",
        default_value_t = false
    )]
    pub absolute: bool,

    #[arg(
        short,
        long,
        help = "print the progress of traversing directories",
        default_value_t = false
    )]
    pub progress: bool,

    #[arg(
        short = 'P',
        long,
        help = "print parent directory, when no more sibling directories are found",
        default_value_t = false
    )]
    pub parent: bool,
}
