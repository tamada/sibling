use std::vec;

use crate::cli::{CliOpts, PrintingOpts};
use clap::Parser;
use sibling::{Dirs, Error, Nexter, Result};

mod cli;
mod gencomp;
mod init;
pub(crate) mod printer;
pub(crate) mod minisib;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn perform_impl(
    dirs: &Dirs,
    nexter: &dyn Nexter,
    step: i32,
    opts: &PrintingOpts,
) -> String {
    let next = dirs.next_with(nexter, step);
    printer::result_string(dirs, next, opts)
}

fn perform_from_file(opts: CliOpts) -> Vec<Result<String>> {
    let nexter = sibling::NexterFactory::create(opts.nexter_opts.nexter);
    let r = match opts.nexter_opts.input {
        None => Err(Error::Fatal("input is not specified".into())),
        Some(file) => match Dirs::new_from_file_with(file, opts.nexter_opts.all) {
            Err(e) => Err(e),
            Ok(dirs) => Ok(perform_impl(&dirs, nexter.as_ref(), opts.nexter_opts.step, &opts.p_opts)),
        },
    };
    vec![r]
}

fn perform_each(
    dir: std::path::PathBuf,
    nexter: &dyn Nexter,
    step: i32,
    opts: &PrintingOpts,
) -> Result<String> {
    match Dirs::new(dir) {
        Err(e) => Err(e),
        Ok(dirs) => Ok(perform_impl(&dirs, nexter, step, opts)),
    }
}

fn perform_sibling(opts: CliOpts) -> Vec<Result<String>> {
    let nexter = sibling::NexterFactory::create(opts.nexter_opts.nexter);
    let target_dirs = if opts.dirs.is_empty() {
        vec![std::env::current_dir().unwrap()]
    } else {
        opts.dirs
    };
    let mut result = vec![];
    for dir in target_dirs {
        let dir = if dir == std::path::Path::new(".") {
            std::env::current_dir().unwrap()
        } else {
            dir
        };
        let r = perform_each(dir, nexter.as_ref(), opts.nexter_opts.step, &opts.p_opts);
        result.push(r);
    }
    result
}

fn perform(opts: CliOpts) -> Vec<Result<String>> {
    if let Some(shell) = opts.init_script.init {
        vec![init::generate_init_script(&shell)]
    } else if opts.nexter_opts.input.is_some() {
        perform_from_file(opts)
    } else if let Some(minisib) = opts.minisib {
        vec![minisib.perform()]
    } else {
        perform_sibling(opts)
    }
}

fn main() {
    let mut opts = cli::CliOpts::parse();
    opts.init();
    if cfg!(debug_assertions) {
        #[cfg(debug_assertions)]
        if opts.compopts.completion {
            return gencomp::generate(&opts.compopts.dest);
        }
    }
    for item in perform(opts) {
        match item {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("{e}"),
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
            eprintln!("{e}");
        }
        assert!(opts_r.is_ok());
        let r = perform(opts_r.unwrap());
        assert_eq!(r.len(), 1);
        match r.first().unwrap() {
            Err(e) => eprintln!("{e}"),
            Ok(result) => println!("{result}"),
        }
    }

    #[test]
    fn test_from_file() {
        let opts_r = cli::CliOpts::try_parse_from(vec![
            "sibling",
            "--input",
            "testdata/basic/dirlist.txt",
            "--type",
            "previous",
        ]);

        if let Err(e) = &opts_r {
            eprintln!("{e}");
        }
        assert!(opts_r.is_ok());
        let r = perform(opts_r.unwrap());
        assert_eq!(r.len(), 1);
        match r.first().unwrap() {
            Err(e) => eprintln!("{e}"),
            Ok(result) => assert_eq!(result, "testdata/basic/a"),
        }
    }
}
