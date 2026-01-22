use std::vec;

use crate::cli::{CliOpts, PrintingOpts};
use clap::Parser;
use sibling::{Dirs, Error, Nexter, Result};

mod cli;
mod gencomp;
mod init;
mod printer;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn init_log(level: &LogLevel) {
    use LogLevel::*;
    if std::env::var_os("RUST_LOG").is_none() {
        match level {
            Error => std::env::set_var("RUST_LOG", "error"),
            Warn => std::env::set_var("RUST_LOG", "warn"),
            Info => std::env::set_var("RUST_LOG", "info"),
            Debug => std::env::set_var("RUST_LOG", "debug"),
            Trace => std::env::set_var("RUST_LOG", "trace"),
        };
    }
    env_logger::init();
    log::info!("Log level set to {level:?}");
}

fn perform_impl(
    dirs: Dirs,
    nexter: &dyn Nexter,
    step: usize,
    opts: &PrintingOpts,
) -> Result<String> {
    let next = dirs.next_with(nexter, step);
    printer::result_string(&dirs, next, opts)
}

fn perform_from_file(opts: CliOpts) -> Vec<Result<String>> {
    let nexter = sibling::NexterFactory::build(opts.nexter);
    let r = match opts.input {
        None => Err(Error::Fatal("input is not specified".into())),
        Some(file) => match Dirs::new_from_file(file) {
            Err(e) => Err(e),
            Ok(dirs) => perform_impl(dirs, nexter.as_ref(), opts.step, &opts.p_opts),
        },
    };
    vec![r]
}

fn perform_each(
    dir: std::path::PathBuf,
    nexter: &dyn Nexter,
    step: usize,
    opts: &PrintingOpts,
) -> Result<String> {
    match Dirs::new(dir) {
        Err(e) => Err(e),
        Ok(dirs) => perform_impl(dirs, nexter, step, opts),
    }
}

fn perform_sibling(opts: CliOpts) -> Vec<Result<String>> {
    let nexter = sibling::NexterFactory::build(opts.nexter);
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
        let r = perform_each(dir, nexter.as_ref(), opts.step, &opts.p_opts);
        result.push(r);
    }
    result
}

fn perform(opts: CliOpts) -> Vec<Result<String>> {
    if let Some(shell) = opts.init {
        vec![init::generate_init_script(shell)]
    } else if opts.input.is_some() {
        perform_from_file(opts)
    } else {
        perform_sibling(opts)
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
    init_log(&opts.log);
    if cfg!(debug_assertions) {
        #[cfg(debug_assertions)]
        if opts.compopts.completion {
            return gencomp::generate(opts.compopts.dest);
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
