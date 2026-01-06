//! The library for traversing the sibling directories.
//! This library provides the interface for obtaining the next/previous sibling directories of a given directory by sorting with alphabetical order.
//! 
//! It supports various strategies for selecting the next directory,
//! such as [first](NexterType::First), [last](NexterType::Last), [next](NexterType::Next),
//! [previous](NexterType::Previous), [random](NexterType::Random), and [keep](NexterType::Keep) current.
//! 
//! ## Example
//! 
//! ```rust
//! let dirs = sibling::Dirs::new("/path/to/current/dir").unwrap();
//! let nexter = sibling::NexterFactory::build(sibling::NexterType::Next);
//! let next_dir = nexter.next(&dirs); // Get the next sibling directory
//! ```
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use clap::ValueEnum;

/// The result type for sibling operations.
pub type Result<T> = std::result::Result<T, Error>;

/// The type of the nexter.
#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
pub enum NexterType {
    /// the first sibling directory.
    First,
    /// the last sibling directory.
    Last,
    /// the previous sibling directory (step parameter specifies how many to go back).
    Previous,
    /// the next sibling directory (step parameter specifies how many to go forward).
    Next,
    /// a random sibling directory (step parameter is ignored).
    Random,
    /// keep the current directory (step parameter is ignored).
    Keep,
}

/// The error type for sibling.
#[derive(Debug)]
pub enum Error {
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
    /// A fatal error with a custom message.
    Fatal(String),
    /// Multiple errors occurred (array of errors).
    Array(Vec<Error>),
}

/// Stores a list of sibling directories, the parent directory path, and the index of the current directory.
#[derive(Debug, Clone)]
pub struct Dirs {
    /// The list of sibling directories, sorted alphabetically.
    dirs: Vec<PathBuf>,
    /// The parent directory that contains all the sibling directories.
    parent: PathBuf,
    /// The index of the current directory in the `dirs` vector.
    current: usize,
}

/// The struct represents a directory in the traversal target set.
#[derive(Debug, Clone)]
pub struct Dir<'a> {
    /// Reference to the parent `Dirs` instance.
    dirs: &'a Dirs,
    /// The index of this directory in the `Dirs.dirs` vector.
    index: usize,
    /// Flag indicating if this is the first or last directory in the sorted list.
    last_item: bool,
}

impl Dir<'_> {
    /// Create a new [`Dir`] instance.
    pub fn new(dirs: &Dirs, index: usize) -> Dir<'_> {
        log::trace!("Dir::new(index={})", index);
        Dir {
            dirs,
            index,
            last_item: false,
        }
    }

    /// Create a new [`Dir`] instance with the last item flag.
    pub fn new_of_last_item(dirs: &Dirs, index: usize) -> Dir<'_> {
        log::trace!("Dir::new_of_last_item(index={})", index);
        Dir {
            dirs,
            index,
            last_item: true,
        }
    }

    /// Get the path of the directory.
    pub fn path(&self) -> &Path {
        &self.dirs.dirs[self.index]
    }

    /// Get the index of the directory.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Check if this directory is the last item.
    pub fn is_last_item(&self) -> bool {
        self.last_item
    }
}

impl Dirs {
    /// Create a new [`Dirs`] instance from the given directory and its sibling directories.
    /// If the current directory is ".", it uses the current working directory.
    /// 
    /// # Returns
    /// 
    /// A [`Result`]<[`Dirs`]> instance.
    /// 
    /// # Errors
    /// 
    /// - Returns [`Error::Io`] if an I/O error occurs.
    /// - Returns [`Error::NotDir`] if the given path is not a directory.
    /// - Returns [`Error::NotFound`] if the given path does not exist.
    pub fn new<P: AsRef<Path>>(current_dir: P) -> Result<Self> {
        let current_dir = current_dir.as_ref();
        log::debug!("Dirs::new(current_dir={})", current_dir.display());
        if current_dir == PathBuf::from(".") {
            match std::env::current_dir() {
                Ok(dir) => build_dirs(dir.clone().parent(), dir),
                Err(e) => {
                    log::error!("Dirs::new: I/O error: {}", e);
                    Err(Error::Io(e))
                }
            }
        } else if current_dir.exists() {
            if current_dir.is_dir() {
                let current = std::fs::canonicalize(current_dir).unwrap();
                build_dirs(current.clone().parent(), current)
            } else {
                log::error!("Dirs::new: Not a directory: {}", current_dir.display());
                Err(Error::NotDir(current_dir.to_path_buf()))
            }
        } else {
            log::error!("Dirs::new: Not found: {}", current_dir.display());
            Err(Error::NotFound(current_dir.to_path_buf()))
        }
    }

    /// Create a new [`Dirs`] instance from the given file.
    /// The file should contain a list of directories, one per line.
    /// The first line can optionally specify the parent directory in the format `parent:/path/to`.
    ///
    /// # Returns
    /// A [`Result`]<[`Dirs`]> instance.
    /// 
    /// # Errors
    /// 
    /// - Returns [`Error::Io`] if an I/O error occurs.
    /// - Returns [`Error::NotFile`] if the given path is not a file.
    /// - Returns [`Error::NotFound`] if the given path does not exist.
    pub fn new_from_file<S: AsRef<str>>(file: S) -> Result<Self> {
        log::debug!("Dirs::new_from_file(file={})", file.as_ref());
        let file = file.as_ref();
        if file == "-" {
            log::info!("Reading directories from stdin");
            return build_from_reader(Box::new(std::io::stdin().lock()));
        }
        let path = PathBuf::from(file);
        if !path.exists() {
            log::error!("Dirs::new_from_file: Not found: {}", path.display());
            Err(Error::NotFound(path))
        } else if path.is_dir() {
            log::error!("Dirs::new_from_file: Not a file: {}", path.display());
            Err(Error::NotFile(path))
        } else {
            build_from_list(path)
        }
    }

    /// Get the parent directory path.
    pub fn parent(&self) -> &Path {
        self.parent.as_path()
    }

    /// Get the current directory as a [`Dir`] instance.
    pub fn current(&self) -> Dir<'_> {
        Dir::new(self, self.current)
    }

    /// Check if the directory list is empty.
    pub fn is_empty(&self) -> bool {
        self.dirs.is_empty()
    }

    /// Get the number of directories in the list.
    pub fn len(&self) -> usize {
        self.dirs.len()
    }

    /// Get the next directory using the given [`Nexter`].
    pub fn next(&self, nexter: &dyn Nexter) -> Option<Dir<'_>> {
        nexter.next_with(self, 1)
    }

    /// Get the next directory using the given [`Nexter`] and step.
    pub fn next_with(&self, nexter: &dyn Nexter, step: usize) -> Option<Dir<'_>> {
        nexter.next_with(self, step as i32)
    }

    /// Get an iterator over the directories.
    pub fn directories(&self) -> impl Iterator<Item = &PathBuf> {
        self.dirs.iter()
    }
}

/// Collect sibling directories under parent and compute the index of current.
fn build_dirs(parent: Option<&Path>, current: PathBuf) -> Result<Dirs> {
    log::trace!("build_dirs(parent={parent:?}, current={current:?})");
    let parent = match parent {
        Some(p) => p,
        None => {
            log::error!("build_dirs: No parent for current={}", current.display());
            return Err(Error::NoParent(current))
        }
    };
    let mut errs = vec![];
    let dirs = collect_dirs(parent, &mut errs);
    if !errs.is_empty() {
        Err(Error::Array(errs))
    } else {
        let current_index = find_current(&dirs, &current);
        if current_index == 0 {
            log::warn!(
                "build_dirs: current directory not found in siblings: {}",
                current.display()
            );
        }
        log::info!(
            "build_dirs: siblings={}, current_index={}",
            dirs.len(),
            current_index
        );
        Ok(Dirs {
            dirs,
            parent: parent.to_path_buf(),
            current: current_index,
        })
    }
}

/// Read child directories under parent, push IO errors to errs, and return a sorted list.
fn collect_dirs(parent: &Path, errs: &mut Vec<Error>) -> Vec<PathBuf> {
    log::trace!("collect_dirs(parent={})", parent.display());
    let mut dirs = vec![];
    if let Ok(entries) = parent.read_dir() {
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs.push(path);
                    }
                }
                Err(e) => {
                    log::error!("collect_dirs: I/O error: {}", e);
                    errs.push(Error::Io(e));
                }
            };
        }
    }
    if log::log_enabled!(log::Level::Warn) && dirs.is_empty() {
        log::warn!("collect_dirs: no directories under {}", parent.display());
    }
    dirs.sort();
    dirs
}

/// Return the index of current in dirs, or 0 if not found.
fn find_current(dirs: &[PathBuf], current: &PathBuf) -> usize {
    let idx = dirs.iter().position(|dir| dir == current).unwrap_or(0);
    log::trace!("find_current: index={} for {}", idx, current.display());
    idx
}

/// Parse lines from a reader; parent: sets base, remaining lines are directory entries.
fn build_from_reader(reader: Box<dyn BufRead>) -> Result<Dirs> {
    let lines = reader
        .lines()
        .filter_map(|line| line.map(|n| n.trim().to_string()).ok())
        .collect::<Vec<String>>();
    let base = if let Some(base) = lines.iter().find(|l| l.starts_with("parent:")) {
        base.chars().skip(7).collect::<String>().trim().to_string()
    } else {
        ".".to_string()
    };
    let dirs = lines
        .iter()
        .filter(|l| !l.starts_with("parent:"))
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();
    log::debug!("build_from_reader: base='{}', entries={}", base, dirs.len());
    let current = find_current_dir_index(&dirs);
    if current == 0 {
        log::warn!("build_from_reader: current directory not found in siblings");
    }
    Ok(Dirs {
        dirs,
        parent: PathBuf::from(base),
        current,
    })
}

fn find_current_dir_index(dirs: &[PathBuf]) -> usize {
    log::trace!("find_current_dir_index(dirs.len={})", dirs.len());
    if let Ok(pwd) = std::env::current_dir() {
        let cwd = PathBuf::from(".");
        if let Some(pos) = dirs
            .iter()
            .position(|dir| dir == &cwd || pwd.ends_with(dir))
        {
            return pos;
        }
    }
    0
}

fn build_from_list(filename: PathBuf) -> Result<Dirs> {
    if let Ok(f) = std::fs::File::open(&filename) {
        let reader = BufReader::new(f);
        build_from_reader(Box::new(reader))
    } else {
        log::error!("build_from_list: I/O error: {}", filename.display());
        Err(Error::Io(std::io::Error::last_os_error()))
    }
}

/// The trait for nexter.
/// This trait defines the interface for obtaining the next directory.
pub trait Nexter {
    /// Find the next directory with the given step.
    /// If the nexter type is `first`, `last`, `keep`, or `random`, the step parameter is ignored.
    fn next_with<'a>(&self, dirs: &'a Dirs, step: i32) -> Option<Dir<'a>>;

    /// Find the next directory (calls `self.next_with(dirs, 1)`).
    fn next<'a>(&self, dirs: &'a Dirs) -> Option<Dir<'a>> {
        self.next_with(dirs, 1)
    }
}

/// Factory pattern for creating [`Nexter`] instances.
pub struct NexterFactory {}

impl NexterFactory {
    /// Build a [`Nexter`] instance based on the given [`NexterType`].
    pub fn build(nexter_type: NexterType) -> Box<dyn Nexter> {
        log::trace!("NexterFactory::build(nexter_type={:?})", nexter_type);
        match nexter_type {
            NexterType::First => Box::new(First {}),
            NexterType::Last => Box::new(Last {}),
            NexterType::Previous => Box::new(Previous {}),
            NexterType::Next => Box::new(Next {}),
            NexterType::Random => Box::new(Random {}),
            NexterType::Keep => Box::new(Keep {}),
        }
    }
}

struct First {}
struct Last {}
struct Previous {}
struct Next {}
struct Random {}
struct Keep {}

impl Nexter for First {
    fn next_with<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        Some(Dir::new_of_last_item(dirs, 0))
    }
}

impl Nexter for Last {
    fn next_with<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        let next = dirs.dirs.len() - 1;
        Some(Dir::new_of_last_item(dirs, next))
    }
}

impl Nexter for Previous {
    fn next_with<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        next_impl(dirs, -_step)
    }
}

impl Nexter for Next {
    fn next_with<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        next_impl(dirs, _step)
    }
}

impl Nexter for Random {
    fn next_with<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        use rand::Rng;
        let mut rng = rand::rng();
        let next = rng.random_range(0..dirs.dirs.len()) as usize;
        log::trace!("Random::next_with -> index {}", next);
        Some(Dir::new(dirs, next))
    }
}

impl Nexter for Keep {
    fn next_with<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        Some(dirs.current())
    }
}

fn next_impl(dirs: &Dirs, step: i32) -> Option<Dir<'_>> {
    let next = dirs.current as i32 + step;
    log::trace!("next_impl(step={step}, current={}, next={next})", dirs.current);
    if next < 0 || next >= dirs.dirs.len() as i32 {
        log::warn!("next_impl: out of range (next={next}, len={})", dirs.dirs.len());
        None
    } else if next == 0 {
        Some(Dir::new_of_last_item(dirs, 0))
    } else if next == dirs.dirs.len() as i32 - 1 {
        Some(Dir::new_of_last_item(dirs, dirs.dirs.len() - 1))
    } else {
        Some(Dir::new(dirs, next as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirs_new() {
        let dirs = Dirs::new(PathBuf::from("../testdata/d"));
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(dirs.dirs.len(), 26);
        assert_eq!(dirs.current, 3);
    }

    #[test]
    fn test_dir_dot() {
        let dirs = Dirs::new(PathBuf::from(".."));
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(
            dirs.current().path().file_name().map(|s| s.to_str()),
            Some("sibling".into())
        );
    }

    #[test]
    fn test_dirs() {
        let dirs = Dirs::new(PathBuf::from("../testdata/d")).unwrap();
        let abspath = Path::new("../testdata").canonicalize().unwrap();
        assert_eq!(dirs.parent(), &abspath);
        assert_eq!(dirs.current().index(), 3);
        assert!(!dirs.is_empty());
        assert_eq!(dirs.len(), 26);
    }


    #[test]
    fn test_dir_from_file() {
        let dirs = Dirs::new_from_file("../testdata/dirlist.txt");
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(dirs.dirs.len(), 4);
        assert_eq!(dirs.current, 1);
        assert_eq!(dirs.parent, PathBuf::from("testdata"));
    }

    #[test]
    fn test_nexter_first() {
        let dirs = Dirs::new("../testdata/c").unwrap();
        let nexter = NexterFactory::build(NexterType::First);
        match nexter.next(&dirs) {
            Some(p) => assert!(p.path().ends_with("testdata/a")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_keep() {
        let dirs = Dirs::new("../testdata/c").unwrap();
        let nexter = NexterFactory::build(NexterType::Keep);
        match nexter.next(&dirs) {
            Some(p) => assert!(p.path().ends_with("testdata/c")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_last() {
        let dirs = Dirs::new("../testdata/k").unwrap();
        let nexter = NexterFactory::build(NexterType::Last);
        match nexter.next(&dirs) {
            Some(p) => assert!(p.path().ends_with("testdata/z")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_next() {
        let dirs = Dirs::new("../testdata/c").unwrap();
        let nexter = NexterFactory::build(NexterType::Next);
        match nexter.next(&dirs) {
            Some(p) => assert!(p.path().ends_with("testdata/d")),
            None => panic!("unexpected None"),
        }
        match nexter.next_with(&dirs, 2) {
            Some(p) => assert!(p.path().ends_with("testdata/e"), "{:?}", p.path()),
            None => panic!("unexpected None"),
        }
        match nexter.next_with(&dirs, 23) {
            Some(p) => assert!(p.path().ends_with("testdata/z"), "{:?}", p.path()),
            None => panic!("unexpected None"),
        }
        match nexter.next_with(&dirs, 24) {
            None => {}
            Some(p) => panic!("unexpected {:?}", p.path()),
        }
    }

    #[test]
    fn test_nexter_prev() {
        let dirs = Dirs::new("../testdata/k").unwrap();
        let nexter = NexterFactory::build(NexterType::Previous);
        match nexter.next(&dirs) {
            Some(p) => assert!(p.path().ends_with("testdata/j")),
            None => panic!("unexpected None"),
        }
        match nexter.next(&dirs) {
            Some(p) => assert!(p.path().ends_with("testdata/j")),
            None => panic!("unexpected None"),
        }
        match nexter.next_with(&dirs, 4) {
            Some(p) => assert!(p.path().ends_with("testdata/g")),
            None => panic!("unexpected None"),
        }
        match nexter.next_with(&dirs, 10) {
            Some(p) => assert!(p.path().ends_with("testdata/a")),
            None => panic!("unexpected None"),
        }
        if let Some(p) = nexter.next_with(&dirs, 11) {
            panic!("unexpected {:?}", p.path())
        }
    }
}
