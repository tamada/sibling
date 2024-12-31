use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::cli::{Result, SiblingError};

pub struct Dirs {
    pub(crate) dirs: Vec<PathBuf>,    
    pub(crate) parent: PathBuf,
    pub(crate) current: usize,
    pub(crate) next: i32,
    pub(crate) no_more_dir: bool,
}

impl Dirs {
    pub fn new(current_dir: PathBuf) -> Result<Self> {
        if current_dir == PathBuf::from(".") {
            match std::env::current_dir() {
                Ok(dir) => 
                    build_dirs(dir.clone().parent(), dir),
                Err(e) => Err(SiblingError::Io(e)),
            }
        } else if current_dir.exists() {
            if current_dir.is_dir() {
                let current = std::fs::canonicalize(&current_dir).unwrap();
                build_dirs(current.clone().parent(), current)
            } else {
                Err(SiblingError::NotDir(current_dir))
            }
        } else {
            Err(SiblingError::NotFound(current_dir))
        }
    }
    pub fn new_from_file(file: String) -> Result<Self> {
        if file == "-" {
            return build_from_reader(Box::new(std::io::stdin().lock()));
        }
        let path = PathBuf::from(file);
        if !path.exists() {
            return Err(SiblingError::NotFound(path));
        } else if path.is_dir() {
            return Err(SiblingError::NotFile(path));
        } else {
            return build_from_list(path);
        }
    }

    pub fn current_path(&self) -> PathBuf {
        self.dirs[self.current].clone()
    }

    pub fn next_path(&self) -> Option<PathBuf> {
        if self.next >= 0 && self.next < self.dirs.len() as i32 {
            Some(self.dirs[self.next as usize].clone())
        } else {
            None
        }
    }
}

fn build_dirs(parent: Option<&Path>, current: PathBuf) -> Result<Dirs> {
    let parent = match parent {
        Some(p) => p,
        None => return Err(SiblingError::NoParent(current)),
    };
    let mut errs = vec![];
    let dirs = collect_dirs(&parent, &mut errs);
    if !errs.is_empty() {
        Err(SiblingError::Array(errs))
    } else {
        let current_index = find_current(&dirs, &current);
        Ok(Dirs {
            dirs,
            parent: parent.to_path_buf(),
            current: current_index,
            next: -1,
            no_more_dir: false,
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
                },
                Err(e) => errs.push(SiblingError::Io(e))
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
    let lines = reader.lines()
        .filter_map(|line| line.map(|n| n.trim().to_string()).ok())
        .collect::<Vec<String>>();
    let base = if let Some(base) = lines.iter().find(|l| l.starts_with("parent:")) {
        base.chars().skip(7).collect::<String>().trim().to_string()
    } else {
        ".".to_string()
    };
    let dirs = lines.iter()
            .filter(|l| !l.starts_with("parent:"))
            .map(|line| PathBuf::from(line))
            .collect::<Vec<PathBuf>>();
    let current = find_current_dir_index(&dirs);
    Ok(Dirs {
        dirs,
        parent: PathBuf::from(base),
        current: current,
        next: -1,
        no_more_dir: false,
    })
}

fn find_current_dir_index(dirs: &Vec<PathBuf>) -> usize {
    if let Ok(pwd) = std::env::current_dir() {
        let cwd = PathBuf::from(".");
        if let Some(pos) = dirs.iter().position(|dir| dir == &cwd || pwd.ends_with(dir)) {
            return pos;
        }
    }
    0 as usize
}

fn build_from_list(filename: PathBuf) -> Result<Dirs> {
    if let Ok(f) = std::fs::File::open(filename) {
        let reader = BufReader::new(f);
        build_from_reader(Box::new(reader))
    } else {
        Err(SiblingError::Io(std::io::Error::last_os_error()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirs_new() {
        let dirs = Dirs::new(PathBuf::from("testdata/d"));
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(dirs.dirs.len(), 26);
        assert_eq!(dirs.current, 3);
    }

    #[test]
    fn test_dir_dot() {
        let dirs = Dirs::new(PathBuf::from("."));
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(dirs.current_path().file_name().map(|s| s.to_str()), Some("sibling".into()));
    }

    #[test]
    fn test_dir_from_file() {
        let dirs = Dirs::new_from_file("testdata/dirlist.txt".into());
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(dirs.dirs.len(), 4);
        assert_eq!(dirs.current, 1);
        assert_eq!(dirs.parent, PathBuf::from("testdata"));
    }
}
