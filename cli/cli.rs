use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use sibling::{Error, Result};

use crate::LogLevel;

#[derive(Debug, Parser)]
#[clap(
    version,
    author,
    about,
    arg_required_else_help = true,
    after_help = "Exit status:
  0  the next directory was found (printed to stdout),
  1  no more sibling directory was found,
  2  the given command line arguments were wrong, and
  3  the command failed (the reason is printed to stderr)."
)]
pub struct SiblingOpts {
    #[clap(flatten)]
    pub(crate) cli_opts: CliOpts,

    #[clap(flatten)]
    pub(crate) init_script: InitOpts,

    #[clap(flatten)]
    pub(crate) log_opts: LogOpts,

    #[cfg(debug_assertions)]
    #[clap(flatten)]
    pub(crate) compopts: CompletionOpts,
}

#[derive(Debug, Parser)]
pub(crate) struct CliOpts {
    #[clap(flatten)]
    pub(crate) p_opts: PrintingOpts,

    #[clap(flatten)]
    pub(crate) nexter_opts: NexterOpts,

    #[clap(flatten)]
    pub(crate) base_opts: BaseOpts,
}

impl SiblingOpts {
    pub fn init(&mut self) {
        self.log_opts.init();
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
        help = "generate the initialize script for the shell [possible values: bash, zsh, fish, powershell, elvish]",
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
        help = "specify the number of times to execute sibling. The negative count traverses in the opposite direction, and 0 means the current directory",
        value_name = "COUNT",
        default_value_t = 1,
        allow_negative_numbers = true
    )]
    pub step: i32,

    #[arg(short = 't', long = "type", help = "specify the nexter type", value_enum, default_value_t = sibling::NexterType::Next, value_name = "TYPE", ignore_case = true)]
    pub nexter: sibling::NexterType,
}

#[derive(Parser, Debug)]
pub(crate) struct BaseOpts {
    #[arg(
        short = 'a',
        long,
        help = "Set the targets to all directories from the given list. By default, the sibling skips non-existent directories.",
        default_value_t = false
    )]
    pub all: bool,

    #[arg(
        short = 'b',
        alias = "parent",
        long,
        help = "specify the parent directory of DIR (default: the parent directory of DIR)",
        value_name = "DIR"
    )]
    pub base_path: Option<PathBuf>,

    #[arg(
        long,
        help = "specify the action when the current directory is not in the target directories, such as DIR which is not in --base-path",
        value_enum,
        default_value_t = sibling::factory::NotOnDirs::Error,
        value_name = "ACTION",
        ignore_case = true
    )]
    pub not_on_dirs: sibling::factory::NotOnDirs,

    #[arg(index = 1, help = "the directory to find its siblings, or the file of directory list", value_name = "DIR|FILE", default_value = ".")]
    pub input: String,
}

impl BaseOpts {
    /// Build the [`Config`](sibling::factory::Config) from the command line arguments.
    ///
    /// The `DIR` argument means the directory itself, not its parent;
    /// the siblings of `DIR` are the child directories of the parent of `DIR`.
    /// Therefore, the parent directory is the value of `--base-path`, if it is given,
    /// otherwise the parent directory of `DIR`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoParent`] if `DIR` has no parent directory (e.g., the root directory).
    pub fn config(&self) -> Result<sibling::factory::Config> {
        let target = self.target_path();
        let base = match &self.base_path {
            Some(base) => base.clone(),
            None => match target.parent() {
                Some(parent) => parent.to_path_buf(),
                None => return Err(Error::NoParent(target)),
            },
        };
        log::debug!("target: {}, base: {}", target.display(), base.display());
        if target.is_dir() {
            Ok(sibling::factory::Config::new_with_wd(base, self.all, target).not_on_dirs(self.not_on_dirs))
        } else {
            Ok(sibling::factory::Config::new(base, self.all).not_on_dirs(self.not_on_dirs))
        }
    }

    /// Return the path of the `DIR` argument.
    ///
    /// The path is resolved to the absolute path if its parent directory is not
    /// obvious from the given string; that is, the path is `.`, `..`, or a name
    /// without any separator (their parent is the current working directory).
    /// Otherwise, the given path is used as is, for printing the resultant paths
    /// in the same style as the given one.
    fn target_path(&self) -> PathBuf {
        let path = PathBuf::from(&self.input);
        if is_parent_obvious(&path) {
            path
        } else {
            std::fs::canonicalize(&path).unwrap_or(path)
        }
    }
}

/// Return true if the parent directory is derivable from the given path by [`Path::parent`].
fn is_parent_obvious(path: &Path) -> bool {
    use std::path::Component::{CurDir, ParentDir};

    let has_parent = path.parent().is_some_and(|p| !p.as_os_str().is_empty());
    has_parent && !path.components().any(|c| c == CurDir || c == ParentDir)
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
}
