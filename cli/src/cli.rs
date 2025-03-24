use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, author, about, arg_required_else_help = true)]
pub struct CliOpts {
    #[clap(flatten)]
    pub(crate) p_opts: PrintingOpts,

    #[arg(
        short,
        long,
        help = "specify the number of times to execute sibling",
        value_name = "COUNT",
        default_value_t = 1
    )]
    pub step: usize,

    #[arg(
        long,
        help = "generate the initialize script for the shell",
        value_name = "SHELL",
        hide = true,
        default_missing_value = "bash"
    )]
    pub init: Option<String>,

    #[arg(short = 't', long = "type", help = "specify the nexter type", value_enum, default_value_t = sibling::NexterType::Next, value_name = "TYPE", ignore_case = true)]
    pub nexter: sibling::NexterType,

    #[arg(
        short,
        long,
        help = "directory list from file, if FILE is \"-\", reads from stdin.",
        value_name = "FILE"
    )]
    pub input: Option<String>,

    #[arg(index = 1, help = "the target directory", value_name = "DIR")]
    pub dirs: Vec<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct PrintingOpts {
    #[arg(
        long,
        help = "print the result in the csv format",
        default_value_t = false,
        hide = true
    )]
    pub csv: bool,

    #[arg(
        short,
        long,
        help = "print the directory name in the absolute path",
        default_value_t = false
    )]
    pub absolute: bool,

    #[arg(
        short,
        long,
        help = "list the sibling directories",
        default_value_t = false
    )]
    pub list: bool,

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
