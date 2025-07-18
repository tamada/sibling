//! The sibling library.
//!
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use clap::ValueEnum;

pub type Result<T> = std::result::Result<T, SiblingError>;

/// The type of the nexter.
#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
pub enum NexterType {
    First,
    Last,
    Previous,
    Next,
    Random,
    Keep,
}

/// The error type for sibling.
#[derive(Debug)]
pub enum SiblingError {
    Io(std::io::Error),
    NotFound(PathBuf),
    NoParent(PathBuf),
    NotDir(PathBuf),
    NotFile(PathBuf),
    Fatal(String),
    Array(Vec<SiblingError>),
}

/// The struct of directory list.
#[derive(Debug, Clone)]
pub struct Dirs {
    dirs: Vec<PathBuf>,
    parent: PathBuf,
    current: usize,
}

/// The struct of directory.
#[derive(Debug, Clone)]
pub struct Dir<'a> {
    dirs: &'a Dirs,
    index: usize,
    last_item: bool,
}

impl Dir<'_> {
    /// Create a new `Dir` instance.
    pub fn new(dirs: &Dirs, index: usize) -> Dir<'_> {
        Dir {
            dirs,
            index,
            last_item: false,
        }
    }

    /// Create a new `Dir` instance with the last item flag.
    pub fn new_of_last_item(dirs: &Dirs, index: usize) -> Dir<'_> {
        Dir {
            dirs,
            index,
            last_item: true,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.dirs.dirs[self.index].clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn is_last_item(&self) -> bool {
        self.last_item
    }
}

impl Dirs {
    pub fn new<P: AsRef<Path>>(current_dir: P) -> Result<Self> {
        let current_dir = current_dir.as_ref();
        if current_dir == PathBuf::from(".") {
            match std::env::current_dir() {
                Ok(dir) => build_dirs(dir.clone().parent(), dir),
                Err(e) => Err(SiblingError::Io(e)),
            }
        } else if current_dir.exists() {
            if current_dir.is_dir() {
                let current = std::fs::canonicalize(current_dir).unwrap();
                build_dirs(current.clone().parent(), current)
            } else {
                Err(SiblingError::NotDir(current_dir.to_path_buf()))
            }
        } else {
            Err(SiblingError::NotFound(current_dir.to_path_buf()))
        }
    }

    pub fn new_from_file<S: AsRef<str>>(file: S) -> Result<Self> {
        let file = file.as_ref();
        if file == "-" {
            return build_from_reader(Box::new(std::io::stdin().lock()));
        }
        let path = PathBuf::from(file);
        if !path.exists() {
            Err(SiblingError::NotFound(path))
        } else if path.is_dir() {
            Err(SiblingError::NotFile(path))
        } else {
            build_from_list(path)
        }
    }

    pub fn parent(&self) -> PathBuf {
        self.parent.clone()
    }

    pub fn current(&self) -> Dir<'_> {
        Dir::new(self, self.current)
    }

    pub fn is_empty(&self) -> bool {
        self.dirs.is_empty()
    }

    pub fn len(&self) -> usize {
        self.dirs.len()
    }

    pub fn next(&self, nexter: &dyn Nexter) -> Option<Dir<'_>> {
        self.next_with(nexter, 1)
    }

    pub fn next_with(&self, nexter: &dyn Nexter, step: usize) -> Option<Dir<'_>> {
        nexter.next(self, step as i32)
    }

    pub fn directories(&self) -> impl Iterator<Item = &PathBuf> {
        self.dirs.iter()
    }
}

fn build_dirs(parent: Option<&Path>, current: PathBuf) -> Result<Dirs> {
    let parent = match parent {
        Some(p) => p,
        None => return Err(SiblingError::NoParent(current)),
    };
    let mut errs = vec![];
    let dirs = collect_dirs(parent, &mut errs);
    if !errs.is_empty() {
        Err(SiblingError::Array(errs))
    } else {
        let current_index = find_current(&dirs, &current);
        Ok(Dirs {
            dirs,
            parent: parent.to_path_buf(),
            current: current_index,
        })
    }
}

fn collect_dirs(parent: &Path, errs: &mut Vec<SiblingError>) -> Vec<PathBuf> {
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
                Err(e) => errs.push(SiblingError::Io(e)),
            };
        }
    }
    dirs.sort();
    dirs
}

fn find_current(dirs: &[PathBuf], current: &PathBuf) -> usize {
    dirs.iter().position(|dir| dir == current).unwrap_or(0)
}

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
    let current = find_current_dir_index(&dirs);
    Ok(Dirs {
        dirs,
        parent: PathBuf::from(base),
        current,
    })
}

fn find_current_dir_index(dirs: &[PathBuf]) -> usize {
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
    if let Ok(f) = std::fs::File::open(filename) {
        let reader = BufReader::new(f);
        build_from_reader(Box::new(reader))
    } else {
        Err(SiblingError::Io(std::io::Error::last_os_error()))
    }
}

pub trait Nexter {
    fn next<'a>(&self, dirs: &'a Dirs, step: i32) -> Option<Dir<'a>>;
}

pub struct NexterFactory {}

impl NexterFactory {
    pub fn build(nexter_type: NexterType) -> Box<dyn Nexter> {
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
    fn next<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        Some(Dir::new_of_last_item(dirs, 0))
    }
}

impl Nexter for Last {
    fn next<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        let next = dirs.dirs.len() - 1;
        Some(Dir::new_of_last_item(dirs, next))
    }
}

impl Nexter for Previous {
    fn next<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        next_impl(dirs, -_step)
    }
}

impl Nexter for Next {
    fn next<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        next_impl(dirs, _step)
    }
}

impl Nexter for Random {
    fn next<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        use rand::Rng;
        let mut rng = rand::rng();
        let next = rng.random_range(0..dirs.dirs.len()) as usize;
        Some(Dir::new(dirs, next))
    }
}

impl Nexter for Keep {
    fn next<'a>(&self, dirs: &'a Dirs, _step: i32) -> Option<Dir<'a>> {
        Some(dirs.current())
    }
}

fn next_impl(dirs: &Dirs, step: i32) -> Option<Dir<'_>> {
    let next = dirs.current as i32 + step;
    if next < 0 || next >= dirs.dirs.len() as i32 {
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
        match nexter.next(&dirs, 1) {
            Some(p) => assert!(p.path().ends_with("testdata/a")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_last() {
        let dirs = Dirs::new("../testdata/k").unwrap();
        let nexter = NexterFactory::build(NexterType::Last);
        match nexter.next(&dirs, 1) {
            Some(p) => assert!(p.path().ends_with("testdata/z")),
            None => panic!("unexpected None"),
        }
    }

    #[test]
    fn test_nexter_next() {
        let dirs = Dirs::new("../testdata/c").unwrap();
        let nexter = NexterFactory::build(NexterType::Next);
        match nexter.next(&dirs, 1) {
            Some(p) => assert!(p.path().ends_with("testdata/d")),
            None => panic!("unexpected None"),
        }
        match nexter.next(&dirs, 2) {
            Some(p) => assert!(p.path().ends_with("testdata/e"), "{:?}", p.path()),
            None => panic!("unexpected None"),
        }
        match nexter.next(&dirs, 23) {
            Some(p) => assert!(p.path().ends_with("testdata/z"), "{:?}", p.path()),
            None => panic!("unexpected None"),
        }
        match nexter.next(&dirs, 24) {
            None => {}
            Some(p) => panic!("unexpected {:?}", p.path()),
        }
    }

    #[test]
    fn test_nexter_prev() {
        let dirs = Dirs::new("../testdata/k").unwrap();
        let nexter = NexterFactory::build(NexterType::Previous);
        match nexter.next(&dirs, 1) {
            Some(p) => assert!(p.path().ends_with("testdata/j")),
            None => panic!("unexpected None"),
        }
        match nexter.next(&dirs, 1) {
            Some(p) => assert!(p.path().ends_with("testdata/j")),
            None => panic!("unexpected None"),
        }
        match nexter.next(&dirs, 4) {
            Some(p) => assert!(p.path().ends_with("testdata/g")),
            None => panic!("unexpected None"),
        }
        match nexter.next(&dirs, 10) {
            Some(p) => assert!(p.path().ends_with("testdata/a")),
            None => panic!("unexpected None"),
        }
        if let Some(p) = nexter.next(&dirs, 11) {
            panic!("unexpected {:?}", p.path())
        }
    }
}
