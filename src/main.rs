use std::path::PathBuf;

use clap::Parser;
use crate::cli::{CliOpts, PrintingOpts, Result, SiblingError};
use crate::nexter::Nexter;

mod cli;
mod dirs;
mod init;
mod nexter;
mod printer;

fn perform_impl(mut dirs: dirs::Dirs, nexter: &Box<dyn Nexter>, step: i32, opts: &PrintingOpts) -> Result<String> {
    nexter.next(&mut dirs, step);
    printer::result_string(&dirs, opts)
}

fn perform_each(dir: std::path::PathBuf, nexter: &Box<dyn Nexter>, step: i32, opts: &PrintingOpts) -> Result<String> {
    match dirs::Dirs::new(dir) {
        Err(e) => Err(e),
        Ok(dirs) => perform_impl(dirs, nexter, step, opts),
    }
}

fn perform_sibling(opts: CliOpts) -> Vec<Result<String>> {
    let nexter = nexter::build_nexter(opts.nexter);
    let target_dirs = if opts.dirs.is_empty() {
        vec![std::env::current_dir().unwrap()]
    } else {
        opts.dirs
    };
    let mut result = vec![];
    for dir in target_dirs {
        let dir = if dir == PathBuf::from(".") {
            std::env::current_dir().unwrap()
        } else {
            std::path::PathBuf::from(dir)
        };
        let r = perform_each(dir, &nexter, opts.step, &opts.p_opts);
        result.push(r);
    }
    result
}

fn perform(opts: CliOpts) -> Vec<Result<String>> {
    if let Some(shell) = opts.init {
        vec![init::generate_init_script(shell)]
    } else {
        perform_sibling(opts)
    }
}

fn print_error(e: &SiblingError) {
    match e {
        SiblingError::Io(e) => eprintln!("I/O error: {}", e),
        SiblingError::NotDir(path) => eprintln!("{:?}: Not a directory", path),
        SiblingError::NoParent(path) => eprintln!("{:?}: no parent directory", path),
        SiblingError::Array(array) => {
            array.iter().for_each(print_error);
        },
        SiblingError::NotFound(path) => eprintln!("{:?}: not found", path),
        SiblingError::Fatal(message) => eprintln!("fatal error: {}", message)
    }
}

fn main() {
    let mut args = std::env::args();
    let args = if args.len() == 1 {
        vec![args.next().unwrap(), ".".into()]
    } else {
        args.collect()
    };
    let opts = cli::CliOpts::parse_from(args);
    for item in perform(opts) {
        match item {
            Ok(result) => println!("{}", result),
            Err(e) => print_error(&e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nexter_example() {
        let opts_r = cli::CliOpts::try_parse_from(vec!["sibling", "."]);

        if let Err(e) = &opts_r {
            eprintln!("{}", e);
        }
        assert!(opts_r.is_ok());
        let r = perform(opts_r.unwrap());
        assert_eq!(r.len(), 1);
        match r.get(0).unwrap() {
            Err(e) => print_error(&e),
            Ok(result) => println!("{}", result),
        }
    }
}