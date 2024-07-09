use clap::Parser;
use crate::cli::{CliOpts, PrintingOpts, Result, SiblingError};
use crate::nexter::Nexter;

mod cli;
mod dirs;
mod init;
mod nexter;
mod printer;

fn perform_impl(mut dirs: dirs::Dirs, nexter: Box<dyn Nexter>, step: i32, opts: PrintingOpts) -> Result<String> {
    nexter.next(&mut dirs, step);
    printer::result_string(&dirs, opts)
}

fn perform(opts: CliOpts) -> Result<String> {
    if let Some(shell) = opts.init {
        init::generate_init_script(shell)
    } else {
        let nexter = nexter::build_nexter(opts.nexter);
        match dirs::Dirs::new(opts.dir) {
            Err(e) => Err(e),
            Ok(dirs) => perform_impl(dirs, nexter, opts.step, opts.p_opts),
        }
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
    let opts = cli::CliOpts::parse();
    match perform(opts) {
        Ok(result) => println!("{}", result),
        Err(e) => print_error(&e),
    }
}
