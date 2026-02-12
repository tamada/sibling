//! minisib: A minimal sibling command implementation
//!
//! This program is used for directory traversing in the shell scripts.
//!
//! Usage:
//! minisib <NEXTER_TYPE> [NUM] [INPUT_FILE]
//! NEXTER_TYPE: next, previous, first, last, random, keep.
//! NUM:         specify the number of times to execute sibling (default: 1)
//!              -1 means minus one step, -2 means minus two steps, and +2 means plus two steps.
//! INPUT_FILE:  file containing the list of directories (if not provided, uses current directory.
//! NUM and INPUT_FILE can be in any order.
use clap::{Parser, Subcommand};
use sibling::{Dirs, Error, NexterType, Result};

#[derive(Subcommand, Debug)]
pub(crate) enum MiniSibCommand {
    #[command(
        name = "minisib",
        about = "Great Finding! A minimal sibling command implementation. This command is a helper utility for the shell. It assumes the user will not use it.",
        hide = true
    )]
    MiniSib(RawOpts),
}

impl MiniSibCommand {
    pub(crate) fn perform(&self) -> Result<String> {
        match self {
            MiniSibCommand::MiniSib(opts) => opts.parse().and_then(|item| item.perform()),
        }
    }
}

#[derive(Parser, Debug)]
pub(crate) struct RawOpts {
    #[arg(allow_hyphen_values = true)]
    args: Vec<String>,
}

impl RawOpts {
    pub(crate) fn parse(&self) -> Result<MiniSibOpts> {
        if self.args.is_empty() {
            Err(Error::Fatal("NEXTER_TYPE is required".into()))
        } else if self.args.len() > 3 {
            Err(Error::Fatal("Too many arguments".into()))
        } else if self.args.len() == 1 {
            let nexter = self.args.first().unwrap().parse::<NexterType>()?;
            Ok(MiniSibOpts::new(nexter, 1, None))
        } else if self.args.len() == 2 {
            let nexter = self.args.first().unwrap().parse::<NexterType>()?;
            if let Ok(n) = self.args[1].parse::<i32>() {
                let step = n;
                Ok(MiniSibOpts::new(nexter, step, None))
            } else {
                let file = self.args[1].as_str().to_string();
                Ok(MiniSibOpts::new(nexter, 1, Some(file)))
            }
        } else {
            let nexter = self.args.first().unwrap().parse::<NexterType>()?;
            if let Ok(n) = self.args[1].parse::<i32>() {
                let step = n;
                let file = Some(self.args[2].as_str().to_string());
                Ok(MiniSibOpts::new(nexter, step, file))
            } else if let Ok(n) = self.args[2].parse::<i32>() {
                let step = n;
                let file = Some(self.args[1].as_str().to_string());
                Ok(MiniSibOpts::new(nexter, step, file))
            } else {
                Err(Error::Fatal("Invalid step argument".into()))
            }
        }
    }
}

pub(crate) struct MiniSibOpts {
    pub(crate) nexter_type: NexterType,
    pub(crate) step: i32,
    pub(crate) file: Option<String>,
}

impl MiniSibOpts {
    pub(crate) fn new(nexter_type: NexterType, step: i32, file: Option<String>) -> Self {
        MiniSibOpts {
            nexter_type,
            step,
            file,
        }
    }

    pub(crate) fn perform(&self) -> Result<String> {
        let nexter = sibling::NexterFactory::create(self.nexter_type.clone());
        let target_dirs = if let Some(file) = &self.file {
            Dirs::new_from_file(file)?
        } else {
            let cwd = std::env::current_dir().map_err(Error::Io)?;
            Dirs::new(cwd)?
        };
        if let Some(dir) = target_dirs.next_with(nexter.as_ref(), self.step) {
            Ok(format!(
                "{}\n{}\n{}\n{}",
                crate::printer::pathbuf_to_string(Some(dir.path()), false, target_dirs.on_dirs()),
                target_dirs.len(),
                dir.index() + 1,
                dir.is_last_item()
            ))
        } else {
            Ok(format!(
                "{}\n{}\n{}\n{}",
                crate::printer::pathbuf_to_string(
                    Some(target_dirs.parent()),
                    false,
                    target_dirs.on_dirs()
                ),
                target_dirs.len(),
                -1,
                true
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::minisib::MiniSibCommand;

    #[test]
    fn test_args_1() {
        let args = vec!["sibling", "minisib", "previous", "-2", "dirs.txt"];
        let minisib = crate::cli::CliOpts::parse_from(&args).minisib;
        assert!(minisib.is_some());
        let opts = if let Some(MiniSibCommand::MiniSib(opts)) = minisib {
            opts.parse().unwrap()
        } else {
            panic!("Expected MiniSibCommand::MiniSib");
        };

        assert_eq!(opts.step, -2);
        assert_eq!(opts.nexter_type, super::NexterType::Previous);
        assert_eq!(opts.file, Some("dirs.txt".into()));
    }

    #[test]
    fn test_args_2() {
        let args = vec!["sibling", "minisib", "previous", "dirs.txt"];
        let minisib = crate::cli::CliOpts::parse_from(&args).minisib;
        assert!(minisib.is_some());
        let opts = if let Some(MiniSibCommand::MiniSib(opts)) = minisib {
            opts.parse().unwrap()
        } else {
            panic!("Expected MiniSibCommand::MiniSib");
        };
        assert_eq!(opts.step, 1);
        assert_eq!(opts.nexter_type, super::NexterType::Previous);
        assert_eq!(opts.file, Some("dirs.txt".into()));
    }

    #[test]
    fn test_args_3() {
        let args = vec!["sibling", "minisib", "keep"];
        let minisib = crate::cli::CliOpts::parse_from(&args).minisib;
        assert!(minisib.is_some());
        let opts = if let Some(MiniSibCommand::MiniSib(opts)) = minisib {
            opts.parse().unwrap()
        } else {
            panic!("Expected MiniSibCommand::MiniSib");
        };
        assert_eq!(opts.step, 1);
        assert_eq!(opts.nexter_type, super::NexterType::Keep);
        assert!(opts.file.is_none());
    }
}
