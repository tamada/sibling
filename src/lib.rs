//! The library for traversing the sibling directories.
//! This library provides the interface for obtaining the next/previous sibling directories of a given directory by sorting with alphabetical order.
//!
//! It supports various strategies for selecting the next directory,
//! such as [first](NexterType::First), [last](NexterType::Last), [next](NexterType::Next),
//! [previous](NexterType::Previous), [random](NexterType::Random), and [keep](NexterType::Keep) current.
//!
//! ## Example
//!
//! The siblings of a directory are the child directories of its parent directory.
//! Therefore, give the parent directory as the base directory, and the target
//! directory as the current one.
//!
//! ```rust
//! use sibling::{NexterType, Nextable};
//! use sibling::factory::{Config, DirsFactory};
//!
//! // the siblings of "testdata/basic", that is, the children of "testdata".
//! let config = Config::new_with_wd("testdata", false, "testdata/basic");
//! let dirs = DirsFactory::create_with(&config)
//!     .expect("Failed to create Dirs");
//!
//! let next_dir = dirs.next(NexterType::Next); // Get the next sibling directory
//! assert_eq!(next_dir.map(|d| d.path().to_path_buf()),
//!            Some(std::path::PathBuf::from("testdata/demo")));
//! ```
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::ValueEnum;

use crate::strategy::Nexter;

pub mod factory;
mod strategy;

/// The result type for sibling operations.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
pub enum NexterType {
    First,
    Last,
    Previous,
    Next,
    Random,
    Keep
}

impl NexterType {
    fn build(&self) -> strategy::Strategy {
        match self {
            NexterType::First => strategy::Strategy::First(strategy::First{}),
            NexterType::Last => strategy::Strategy::Last(strategy::Last{}),
            NexterType::Previous => strategy::Strategy::Previous(strategy::Previous{}),
            NexterType::Next => strategy::Strategy::Next(strategy::Next{}),
            NexterType::Random => strategy::Strategy::Random(strategy::Random{}),
            NexterType::Keep => strategy::Strategy::Keep(strategy::Keep{}),
        }
    }
}

impl FromStr for NexterType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "first" => Ok(NexterType::First),
            "last" => Ok(NexterType::Last),
            "previous" => Ok(NexterType::Previous),
            "next" => Ok(NexterType::Next),
            "random" => Ok(NexterType::Random),
            "keep" => Ok(NexterType::Keep),
            _ => Err(Error::UnknownNexterType(s.to_string())),
        }
    }
}

/// The error type for sibling.
#[derive(Debug)]
pub enum Error {
    /// Multiple errors occurred (array of errors).
    Array(Vec<Error>),
    /// A fatal error with a custom message.
    Fatal(String),
    /// I/O error occurred while accessing the file system.
    Io(std::io::Error),
    /// The specified path was not found.
    NotFound(PathBuf),
    /// The path has no parent directory (e.g., root directory).
    NoParent(PathBuf),
    /// The specified path is not a directory.
    NotDir(PathBuf),
    /// The specified path is not a file.
    NotFile(PathBuf),
    /// The specified nexter type is unknown.
    UnknownNexterType(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::NotDir(path) => write!(f, "{}: Not a directory", path.display()),
            Error::NoParent(path) => write!(f, "{}: No parent directory", path.display()),
            Error::Array(array) => array
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
                .fmt(f),
            Error::NotFile(path) => write!(f, "{}: Not a file", path.display()),
            Error::NotFound(path) => write!(f, "{}: Not found", path.display()),
            Error::Fatal(message) => write!(f, "Fatal error: {message}"),
            Error::UnknownNexterType(s) => write!(f, "Unknown nexter type: {s}"),
        }
    }
}

pub trait Nextable {
    /// The index of the current directory in the list.
    ///
    /// [`None`] means that the current directory is unknown; it is not in the
    /// list. Such a position is treated as the one before the first directory,
    /// hence, [`NexterType::Next`] finds the first directory from it, and
    /// [`NexterType::Previous`] and [`NexterType::Keep`] find nothing.
    fn current_index(&self) -> Option<usize>;

    fn dirs(&self) -> &Dirs;

    /// Get the next directory using the given [`NexterType`].
    fn next(&self, nexter: NexterType) -> Option<Dir<'_>> {
        self.next_with(nexter, 1)
    }

    /// Get the next directory using the given [`NexterType`] and step.
    fn next_with(&self, nexter: NexterType, step: i32) -> Option<Dir<'_>>;
}

/// Stores a list of sibling directories, the parent directory path, and the index of the current directory.
/// This struct manages the entries of sibling directories, parent directory, and the current directory index.
#[derive(Debug, Clone)]
pub struct Dirs {
    /// The list of sibling directories, sorted alphabetically.
    entries: Vec<PathBuf>,
    /// The parent directory that contains all the sibling directories.
    parent: PathBuf,
    /// The index of the current directory in the `entries` vector, or [`None`]
    /// if the current directory is not in them. See [`Nextable::current_index`].
    current: Option<usize>,
}

/// The struct represents a directory in the traversal target set.
#[derive(Debug, Clone)]
pub struct Dir<'a> {
    /// Reference to the parent `Dirs` instance.
    siblings: &'a Dirs,
    /// The index of this directory in the `Dirs.dirs` vector.
    index: usize,
}

impl Dir<'_> {
    /// Create a new [`Dir`] instance.
    fn new(siblings: &Dirs, index: usize) -> Dir<'_> {
        log::trace!("Dir::new(index={index})");
        Dir {
            siblings,
            index,
        }
    }

    /// Get the path of the directory.
    pub fn path(&self) -> &Path {
        &self.siblings.entries[self.index]
    }

    /// Get the index of the directory.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Build the [`Dirs`] instance whose current directory is this one.
    pub fn dirs(self) -> Dirs {
        Dirs {
            entries: self.siblings.entries.clone(),
            parent: self.siblings.parent.clone(),
            current: Some(self.index),
        }
    }
}

impl Nextable for Dir<'_> {
    fn current_index(&self) -> Option<usize> {
        Some(self.index)
    }

    fn dirs(&self) -> &Dirs {
        self.siblings
    }

    /// Get the next directory using the given [`Nexter`] and step.
    fn next_with(&self, nexter: NexterType, step: i32) -> Option<Dir<'_>> {
        match nexter.build() {
            strategy::Strategy::First(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Last(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Previous(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Next(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Random(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Keep(strategy) => strategy.next_with(self, step),
        }
    }
}

impl Nextable for Dirs {
    fn current_index(&self) -> Option<usize> {
        self.current
    }

    fn dirs(&self) -> &Dirs {
        self
    }

    /// Get the next directory using the given [`Nexter`] and step.
    fn next_with(&self, nexter: NexterType, step: i32) -> Option<Dir<'_>> {
        match nexter.build() {
            strategy::Strategy::First(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Last(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Previous(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Next(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Random(strategy) => strategy.next_with(self, step),
            strategy::Strategy::Keep(strategy) => strategy.next_with(self, step),
        }
    }
}

impl Dirs {
    /// Build the [`Dirs`] instance whose current directory is unknown.
    /// See [`Nextable::current_index`] for the traversing from such a position.
    pub fn new(base_dir: PathBuf, entries: Vec<PathBuf>) -> Self {
        Dirs {
            entries,
            parent: base_dir,
            current: None,
        }
    }

    /// Build the [`Dirs`] instance whose current directory is `cwd`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotFound`] if `cwd` is not in `entries`.
    pub fn new_with_wd(base_dir: PathBuf, entries: Vec<PathBuf>, cwd: PathBuf) -> Result<Self> {
        match factory::find_current(&base_dir, &entries, &Some(cwd.clone())) {
            None => {
                log::debug!("Dirs::new_with_wd: current directory not found in siblings");
                Err(Error::NotFound(cwd))
            }
            current => Ok(Dirs {
                entries,
                parent: base_dir,
                current,
            }),
        }
    }

    /// Get the parent directory path.
    pub fn parent(&self) -> &Path {
        self.parent.as_path()
    }

    /// Get the current directory as a [`Dir`] instance.
    /// Returns [`None`] if the current directory is not in the list.
    pub fn current(&self) -> Option<Dir<'_>> {
        self.current.map(|index| Dir::new(self, index))
    }

    /// Check if the directory list is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the number of directories in the list.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get an iterator over the directories.
    pub fn directories(&self) -> impl Iterator<Item = &PathBuf> {
        self.entries.iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::factory::*;

    #[test]
    fn test_error_display_not_dir() {
        let err = Error::NotDir(PathBuf::from("/path/to/file"));
        assert_eq!(
            format!("{}", err),
            "/path/to/file: Not a directory".to_string()
        );
    }

    #[test]
    fn test_error_display_not_file() {
        let err = Error::NotFile(PathBuf::from("/path/to/file"));
        assert_eq!(
            format!("{}", err),
            "/path/to/file: Not a file".to_string()
        );
    }

    #[test]
    fn test_error_display_no_parent() {
        let err = Error::NoParent(PathBuf::from("/path/to/file"));
        assert_eq!(
            format!("{}", err),
            "/path/to/file: No parent directory".to_string()
        );
    }

    #[test]
    fn test_error_display_fatal() {
        let err = Error::Fatal("Some fatal error".into());
        assert_eq!(
            format!("{}", err),
            "Fatal error: Some fatal error".to_string()
        );
    }

    #[test]
    fn test_error_display_unknown_nexter_type() {
        let err = Error::UnknownNexterType("unknown".into());
        assert_eq!(
            format!("{}", err),
            "Unknown nexter type: unknown".to_string()
        );
    }

    /// No directory is found from the empty [`Dirs`], with any [`NexterType`].
    #[test]
    fn test_empty_dirs() {
        let dirs = Dirs::new(PathBuf::from("testdata"), vec![]);
        assert!(dirs.is_empty());
        assert_eq!(dirs.len(), 0);
        assert!(dirs.current().is_none());

        for nexter in [
            NexterType::First,
            NexterType::Last,
            NexterType::Previous,
            NexterType::Next,
            NexterType::Random,
            NexterType::Keep,
        ] {
            assert!(
                dirs.next(nexter.clone()).is_none(),
                "{nexter:?}: should find no directory"
            );
        }
    }

    #[test]
    /// The current directory is unknown, hence, it is the position before the
    /// first one; three steps from it point the third directory.
    fn test_dirs_new() {
        let dirs = DirsFactory::create(PathBuf::from("testdata/basic"))
            .expect("Failed to create Dirs");
        assert!(dirs.current().is_none());
        let dir = dirs.next_with(NexterType::Next, 3)
            .expect("Failed to get next directory");
        assert_eq!(dir.path().file_name().unwrap(), "c");
        assert_eq!(dirs.len(), 26);
        assert_eq!(dir.index(), 2);

        assert!(dirs.next(NexterType::Previous).is_none());
        assert!(dirs.next(NexterType::Keep).is_none());
        assert_eq!(
            dirs.next(NexterType::Next).map(|d| d.index()),
            Some(0),
            "the next of the unknown position is the first directory"
        );
    }

    #[test]
    fn test_worried_dirs() {
        let config = Config::new_with_wd("testdata/worried", false, "dir with spaces");
        let dirs = DirsFactory::create_with(&config);
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(dirs.len(), 2);
        assert_eq!(dirs.current, Some(0));
    }

    /// No current directory is given, hence, the next directory of the unknown
    /// position is the first one.
    #[test]
    fn test_dir_dot() {
        let dirs = DirsFactory::create(PathBuf::from("."))
            .expect("Failed to create Dirs");
        assert!(dirs.current().is_none());
        assert_eq!(
            dirs.next(NexterType::Next)
                .map(|d| d.path().file_name().unwrap().to_string_lossy().to_string()),
            Some(String::from(".bin"))
        );
    }

    #[test]
    fn test_dirs() {
        let config = &Config::new_with_wd("testdata/basic", false, "d");
        let dirs = DirsFactory::create_with(config)
            .expect("Failed to create Dirs");
        assert_eq!(dirs.parent(), Path::new("testdata/basic"));
        assert_eq!(dirs.current().unwrap().index(), 3);
        assert!(!dirs.is_empty());
        assert_eq!(dirs.len(), 26);
    }

    #[test]
    fn test_dir_from_file() {
        let dirs = DirsFactory::create_from_file("testdata/basic/dirlist.txt",
            &Config::new("testdata/basic", true))
            .expect("Failed to create Dirs from file");
        assert_eq!(dirs.len(), 3);
        assert!(dirs.current().is_none());
        assert_eq!(dirs.parent, Path::new("testdata/basic"));
    }

    #[test]
    fn test_nexter_first() {
        let config = &Config::new_with_wd("testdata/basic", false, "c");
        let dirs = DirsFactory::create_with(config)
            .expect("Failed to create Dirs");
        match &dirs.next(NexterType::First) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/a")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_keep() {
        let config = &Config::new_with_wd("testdata/basic", false, "c");
        let dirs = DirsFactory::create_with(config)
            .expect("Failed to create Dirs");
        match dirs.next(NexterType::Keep) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/c")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_last() {
        let config = &Config::new_with_wd("testdata/basic", false, "c");
        let dirs = DirsFactory::create_with(config)
            .expect("Failed to create Dirs");
        match dirs.next(NexterType::Last) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/z")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_next() {
        let config = &Config::new_with_wd("testdata/basic", false, "c");
        let dirs = DirsFactory::create_with(config)
            .expect("Failed to create Dirs");
        match dirs.next_with(NexterType::Next, 1) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/d")),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Next, 2) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/e"), "{:?}", p.path()),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Next, 23) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/z"), "{:?}", p.path()),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Next, 24) {
            None => {}
            Some(p) => panic!("unexpected {:?}", p.path()),
        }
    }

    /// The negative step traverses in the opposite direction of the nexter type,
    /// and the step 0 points the current directory itself.
    #[test]
    fn test_nexter_with_negative_step() {
        let config = &Config::new_with_wd("testdata/basic", false, "k");
        let dirs = DirsFactory::create_with(config)
            .expect("Failed to create Dirs");
        match dirs.next_with(NexterType::Next, -3) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/h")),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Previous, -3) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/n")),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Next, 0) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/k")),
            None => panic!("unexpected None"),
        }
        assert!(dirs.next_with(NexterType::Next, -11).is_none());
    }

    #[test]
    fn test_nexter_prev() {
        let config = &Config::new_with_wd("testdata/basic", false, "k");
        let dirs = DirsFactory::create_with(config)
            .expect("Failed to create Dirs");
        match dirs.next(NexterType::Previous) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/j")),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Previous, 1) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/j")),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Previous, 4) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/g")),
            None => panic!("unexpected None"),
        }
        match dirs.next_with(NexterType::Previous, 10) {
            Some(p) => assert!(p.path().ends_with("testdata/basic/a")),
            None => panic!("unexpected None"),
        }
        if let Some(p) = dirs.next_with(NexterType::Previous, 11) {
            panic!("unexpected {:?}", p.path())
        }
    }
}
