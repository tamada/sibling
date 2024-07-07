use std::path::PathBuf;

use clap::{Parser, ValueEnum};

pub type Result<T> = std::result::Result<T, SiblingError>;

#[derive(Debug)]
pub enum SiblingError {
    Io(std::io::Error),
    NotFound(PathBuf),
    NoParent(PathBuf),
    NotDir(PathBuf),
    Fatal(String),
    Array(Vec<SiblingError>),
}

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
pub enum NexterType {
    First,
    Last,
    Previous,
    Next,
    Random,
    Keep,
}

#[derive(Debug, Parser)]
#[clap(
    version,
    author,
    about,
    arg_required_else_help = true,
)]
pub struct CliOpts {
    #[clap(flatten)]
    pub(crate) p_opts: PrintingOpts,

    #[arg(short, long, help = "specify the number of times to execute sibling", value_name = "COUNT", default_value_t = 1)]
    pub step: i32,

    #[arg(short, long, help = "generate the initialize script for the shell", value_name = "SHELL", hide = true)]
    pub init: Option<String>,

    #[arg(short = 't', long = "type", help = "specify the nexter type", value_enum, default_value_t = NexterType::Next, value_name = "TYPE", ignore_case = true)]
    pub nexter: NexterType,

    #[arg(index = 1, help = "the directory for listing the siblings", value_name = "DIR", default_value = ".")]
    pub dir: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct PrintingOpts {
    #[arg(long, help = "print the result in the csv format", default_value_t = false, hide = true)]
    pub csv: bool,

    #[arg(short, long, help = "print the directory name in the absolute path", default_value_t = false)]
    pub absolute: bool,

    #[arg(short, long, help = "list the sibling directories", default_value_t = false)]
    pub list: bool,

    #[arg(short, long, help = "print the progress of traversing directories", default_value_t = false)]
    pub progress: bool,

    #[arg(short = 'P', long, help = "print parent directory, when no more sibling directories are found", default_value_t = false)]
    pub parent: bool,
}

