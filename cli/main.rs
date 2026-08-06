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

fn perform_from_file(opts: CliOpts) -> Result<String> {
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

fn perform_impl(dirs: &Dirs, nexter: NexterOpts, p_opts: &PrintingOpts) -> String {
    let (nexter, step) = (nexter.nexter, nexter.step);
    let next = dirs.next_with(nexter, step);
    printer::result_string(dirs, next, p_opts)
}

/// Find the sibling directories of the given directory.
/// The given directory is the target directory itself, hence, the siblings are
/// the child directories of its parent directory.
fn perform_sibling(opts: CliOpts) -> Result<String> {
    let (base, nexter, p_opts) = (opts.base_opts, opts.nexter_opts, opts.p_opts);
    let config = base.config()?;
    match DirsFactory::create_with(&config) {
        Err(e) => Err(e),
        Ok(dirs) => Ok(perform_impl(&dirs, nexter, &p_opts)),
    }
}

fn perform(opts: CliOpts) -> Result<String> {
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

fn main() {
    let mut opts = cli::SiblingOpts::parse();
    opts.init();
    if cfg!(debug_assertions) {
        #[cfg(debug_assertions)]
        if opts.compopts.completion {
            gencomp::generate(&opts.compopts.dest);
        }
    } else if let Some(shell) = opts.init_script.init {
        match init::generate_init_script(&shell) {
            Ok(script) => println!("{script}"),
            Err(e) => eprintln!("{e}"),
        }
    } else {
        match perform(opts.cli_opts) {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perform_from(args: &[&str]) -> Result<String> {
        let opts = cli::CliOpts::try_parse_from(args).expect("Failed to parse arguments");
        perform(opts)
    }

    /// The `DIR` argument is the target directory itself, and its siblings are
    /// the child directories of its parent directory.
    #[test]
    fn test_nexter_example() {
        let r = perform_from(&["sibling", "testdata/basic/c"])
            .expect("Failed to perform sibling");
        assert_eq!(r, "testdata/basic/d");
    }

    #[test]
    fn test_previous() {
        let r = perform_from(&["sibling", "--type", "previous", "--step", "2", "testdata/basic/c"])
            .expect("Failed to perform sibling");
        assert_eq!(r, "testdata/basic/a");
    }

    /// The `--base-path` option overrides the parent directory of the `DIR` argument.
    #[test]
    fn test_base_path() {
        let r = perform_from(&["sibling", "--base-path", "testdata/basic", "testdata/basic/c"])
            .expect("Failed to perform sibling");
        assert_eq!(r, "testdata/basic/d");
    }

    #[test]
    fn test_no_more_siblings() {
        let r = perform_from(&["sibling", "testdata/basic/z"])
            .expect("Failed to perform sibling");
        assert_eq!(r, "no more sibling directory");
    }

    #[test]
    fn test_not_found() {
        let r = perform_from(&["sibling", "testdata/basic/not_exist_dir"]);
        assert!(r.is_err());
    }

    #[test]
    fn test_from_file() {
        let r = perform_from(&["sibling", "--type", "next", "testdata/basic/dirlist.txt"])
            .expect("Failed to perform sibling");
        assert_eq!(r, "testdata/basic/b");
    }
}
