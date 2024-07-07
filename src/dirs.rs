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
                    build_dirs(current_dir.parent(), dir),
                Err(e) => Err(SiblingError::Io(e)),
            }
        } else {
            if current_dir.exists() {
                if current_dir.is_dir() {
                    build_dirs(current_dir.clone().parent(), current_dir)
                } else {
                    Err(SiblingError::NotDir(current_dir))
                }
            } else {
                Err(SiblingError::NotFound(current_dir))
            }
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
    let dirs = collect_dirs(parent, &mut errs);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirs_new() {
        let dirs = Dirs::new(PathBuf::from("testdata/b"));
        assert!(dirs.is_ok());
        let dirs = dirs.unwrap();
        assert_eq!(dirs.dirs.len(), 4);
        assert_eq!(dirs.current, 1);
    }
}
