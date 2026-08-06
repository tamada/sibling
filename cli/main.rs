use std::path::PathBuf;

use crate::cli::{CliOpts, NexterOpts, PrintingOpts};
use clap::Parser;
use sibling::{Dirs, Error, Result, Nextable};
use sibling::factory::{DirsFactory};

mod cli;
mod gencomp;
mod init;
// pub(crate) mod minisib;
pub(crate) mod printer;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// The exit status of the `sibling` command.
///
/// Note that the status code 2 is not used here; it is the status code of the
/// usage error, and clap exits with it before running the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Status {
    /// The next directory was found, or the requested script was generated.
    Success = 0,
    /// No more sibling directory was found.
    NoMoreSibling = 1,
    /// The command failed; the given directory was not found, it has no parent
    /// directory, an I/O error occurred, and so on.
    Error = 3,
}

/// The result of the command; the text to print to stdout, and the exit status.
/// The `text` is empty if nothing should be printed.
#[derive(Debug)]
pub(crate) struct Outcome {
    pub(crate) text: String,
    pub(crate) status: Status,
}

impl Outcome {
    fn new(text: String, status: Status) -> Self {
        Outcome { text, status }
    }

    /// Print the text to stdout unless it is empty, and return the exit status.
    fn print(&self) -> Status {
        if !self.text.is_empty() {
            println!("{}", self.text);
        }
        self.status
    }
}

fn perform_from_file(opts: CliOpts) -> Result<Outcome> {
    let (base, nexter, p_opts) = (opts.base_opts, opts.nexter_opts, opts.p_opts);
    let config = base.config()?;
    let file = base.input;

    match DirsFactory::create_from_file(&file, &config) {
        Err(e) => Err(e),
        Ok(dirs) => Ok(perform_impl(
            &dirs,
            nexter,
            &p_opts,
        )),
    }
}

fn perform_impl(dirs: &Dirs, nexter: NexterOpts, p_opts: &PrintingOpts) -> Outcome {
    let (nexter, step) = (nexter.nexter, nexter.step);
    let next = dirs.next_with(nexter, step);
    let status = if next.is_some() {
        Status::Success
    } else {
        Status::NoMoreSibling
    };
    Outcome::new(printer::result_string(dirs, next, p_opts), status)
}

/// Find the sibling directories of the given directory.
/// The given directory is the target directory itself, hence, the siblings are
/// the child directories of its parent directory.
fn perform_sibling(opts: CliOpts) -> Result<Outcome> {
    let (base, nexter, p_opts) = (opts.base_opts, opts.nexter_opts, opts.p_opts);
    let config = base.config()?;
    match DirsFactory::create_with(&config) {
        Err(e) => Err(e),
        Ok(dirs) => Ok(perform_impl(&dirs, nexter, &p_opts)),
    }
}

fn perform(opts: CliOpts) -> Result<Outcome> {
    let target = opts.base_opts.input.clone();
    let path = PathBuf::from(&target);
    if target == "." || path.is_dir() {
        perform_sibling(opts)
    // } else if let Some(minisib) = opts.minisib {
    //     vec![minisib.perform()]
    } else if target == "-" || path.is_file() {
        perform_from_file(opts)
    } else {
        Err(Error::Fatal(format!("{target}: No such file or directory")))
    }
}

fn run(opts: cli::SiblingOpts) -> Status {
    #[cfg(debug_assertions)]
    if opts.compopts.completion {
        gencomp::generate(&opts.compopts.dest);
        return Status::Success;
    }
    if let Some(shell) = opts.init_script.init {
        return match init::generate_init_script(&shell) {
            Ok(script) => Outcome::new(script, Status::Success).print(),
            Err(e) => print_error(&e),
        };
    }
    match perform(opts.cli_opts) {
        Ok(outcome) => outcome.print(),
        Err(e) => print_error(&e),
    }
}

fn print_error(e: &Error) -> Status {
    eprintln!("{e}");
    Status::Error
}

fn main() {
    let mut opts = cli::SiblingOpts::parse();
    opts.init();
    std::process::exit(run(opts) as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perform_from(args: &[&str]) -> Result<Outcome> {
        let opts = cli::CliOpts::try_parse_from(args).expect("Failed to parse arguments");
        perform(opts)
    }

    fn perform_ok(args: &[&str]) -> Outcome {
        perform_from(args).expect("Failed to perform sibling")
    }

    /// The `DIR` argument is the target directory itself, and its siblings are
    /// the child directories of its parent directory.
    #[test]
    fn test_nexter_example() {
        let r = perform_ok(&["sibling", "testdata/basic/c"]);
        assert_eq!(r.text, "testdata/basic/d");
        assert_eq!(r.status, Status::Success);
    }

    #[test]
    fn test_previous() {
        let r = perform_ok(&["sibling", "--type", "previous", "--step", "2", "testdata/basic/c"]);
        assert_eq!(r.text, "testdata/basic/a");
        assert_eq!(r.status, Status::Success);
    }

    /// The `--base-path` option overrides the parent directory of the `DIR` argument.
    #[test]
    fn test_base_path() {
        let r = perform_ok(&["sibling", "--base-path", "testdata/basic", "testdata/basic/c"]);
        assert_eq!(r.text, "testdata/basic/d");
        assert_eq!(r.status, Status::Success);
    }

    /// The `default` format prints nothing when no more sibling directory was found.
    #[test]
    fn test_no_more_siblings() {
        let r = perform_ok(&["sibling", "testdata/basic/z"]);
        assert_eq!(r.text, "");
        assert_eq!(r.status, Status::NoMoreSibling);
    }

    /// The other formats print the result even if no more sibling directory was found.
    #[test]
    fn test_no_more_siblings_in_csv() {
        let r = perform_ok(&["sibling", "--format", "csv", "testdata/basic/z"]);
        assert_eq!(r.text, r#""testdata/basic/z","",26,-1,26"#);
        assert_eq!(r.status, Status::NoMoreSibling);
    }

    /// Every directory in the list file is skipped since none of them exists.
    /// The resultant list is empty, and it must not panic with any format and type.
    #[test]
    fn test_empty_list() {
        for nexter in ["first", "last", "previous", "next", "random", "keep"] {
            let r = perform_ok(&["sibling", "--type", nexter, "testdata/no_exist_list.txt"]);
            assert_eq!(r.text, "", "--type {nexter}");
            assert_eq!(r.status, Status::NoMoreSibling, "--type {nexter}");
        }
    }

    #[test]
    fn test_empty_list_in_json() {
        let r = perform_ok(&["sibling", "--format", "json", "testdata/no_exist_list.txt"]);
        assert_eq!(
            r.text,
            r#"{"current":{"path":"","index":-1},"next":{"path":"","index":-1},"total":0}"#
        );
        assert_eq!(r.status, Status::NoMoreSibling);
    }

    #[test]
    fn test_empty_list_in_csv() {
        let r = perform_ok(&["sibling", "--format", "csv", "testdata/no_exist_list.txt"]);
        assert_eq!(r.text, r#""","",-1,-1,0"#);
        assert_eq!(r.status, Status::NoMoreSibling);
    }

    /// The `--all` option makes the non-existent directories the targets.
    #[test]
    fn test_no_exist_list_with_all() {
        let r = perform_ok(&["sibling", "--all", "testdata/no_exist_list.txt"]);
        assert_eq!(r.text, "testdata/no_such_dir_b");
        assert_eq!(r.status, Status::Success);
    }

    #[test]
    fn test_not_found() {
        let r = perform_from(&["sibling", "testdata/basic/not_exist_dir"]);
        assert!(r.is_err());
    }

    /// The root directory has no parent directory, hence, it has no siblings.
    #[test]
    fn test_no_parent() {
        let r = perform_from(&["sibling", "/"]);
        assert!(matches!(r, Err(Error::NoParent(_))));
    }

    #[test]
    fn test_from_file() {
        let r = perform_ok(&["sibling", "--type", "next", "testdata/basic/dirlist.txt"]);
        assert_eq!(r.text, "testdata/basic/b");
        assert_eq!(r.status, Status::Success);
    }
}
