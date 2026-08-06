use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::{Dirs, Error, Result};

/// Configuration for creating `Dirs` instances.
/// the `base_dir` becoms the parent directory, and
/// the child directories of `base_dir` are listed as sibling directories.
/// If `all_target` is true, non-existent directories in the list are also included as target directories.
/// This flag is used when creating `Dirs` from a file or `Reader``.
pub struct Config {
    pub base_dir: std::path::PathBuf,
    pub all_target: bool,
    pub current: Option<PathBuf>,
}

impl Config {
    pub fn new<P: AsRef<Path>>(base_dir: P, all_target: bool) -> Self {
        Config {
            base_dir: base_dir.as_ref().to_path_buf(),
            all_target,
            current: None,
        }
    }

    pub fn new_with_wd<P1: AsRef<Path>, P2: AsRef<Path>>(base_dir: P1, all_target: bool, current: P2) -> Self {
        Config {
            base_dir: base_dir.as_ref().to_path_buf(),
            all_target,
            current: Some(current.as_ref().to_path_buf()),
        }
    }
}

pub struct DirsFactory {
}

impl DirsFactory {
    /// Creates a new [`Dirs`] instance based on the given base directory.
    /// the `base_dir` becoms the parent directory, and
    /// the child directories of `base_dir` are listed as sibling directories.
    /// 
    /// The resultant `Dirs` instance has no current directory, since no directory
    /// is given as the current one. See [`Nextable::current_index`](crate::Nextable::current_index)
    /// for the traversing from such a position.
    pub fn create<P: AsRef<Path>>(base_dir: P) -> Result<Dirs> {
        Self::create_with(&Config {
            base_dir: base_dir.as_ref().to_path_buf(),
            all_target: false,
            current: None,
        })
    }

    /// Create a new [`Dirs`] instance from the given configuration.
    /// If the current directory is ".", it uses the current working directory.
    ///
    /// # Returns
    ///
    /// A [`Result`]<[`Dirs`], [`Error`]> instance.
    ///
    /// # Errors
    ///
    /// - Returns [`Error::Io`]
    ///   - if `config.base_dir` is not found, or an I/O error occurs while reading the directory.
    /// - Returns [`Error::NotDir`] if the given path is not a directory.
    ///   - if `config.base_dir` is not a directory.
    /// - Returns [`Error::NotFound`] if the given path does not exist.
    ///   - if `config.current` is some, and the directory specified by `config.current` is not found in the siblings.
    pub fn create_with(config: &Config) -> Result<Dirs> {
        let base_dir = &config.base_dir;
        if base_dir == Path::new(".") {
            log::info!("DirsFactory::create_with: Using current directory as base_dir");
            let cwd = std::env::current_dir().map_err(Error::Io)?;
            create_dirs_from_base_path(&cwd, config)
        } else if base_dir.exists() && base_dir.is_dir() {
            create_dirs_from_base_path(base_dir, config)
        } else if base_dir.exists() && !base_dir.is_dir() {
            log::error!("{}:  Not a directory.", base_dir.display());
            Err(Error::NotDir(base_dir.to_path_buf()))
        } else {
            log::error!("{}: directory not found", base_dir.display());
            Err(Error::NotFound(base_dir.to_path_buf()))
        }
    }

    pub fn create_from_file<P: AsRef<Path>>(file: P, config: &Config) -> Result<Dirs> {
        let file = file.as_ref();
        if file == Path::new("-") {
            log::info!("Reading directories from stdin");
            return Self::create_from_reader(
                Box::new(std::io::stdin().lock()),
                config
            );
        }
        if !file.exists() {
            log::error!(
                "Dirs::new_from_file: Not found: {}, pwd: {}",
                file.display(),
                std::env::current_dir().unwrap().display()
            );
            Err(Error::NotFound(file.to_path_buf()))
        } else if file.is_dir() {
            log::error!("Dirs::new_from_file: Not a file: {}", file.display());
            Err(Error::NotFile(file.to_path_buf()))
        } else {
            build_from_list_file(file, config)
        }
    }

    /// Create a new [`Dirs`] instance from the list of the directories.
    ///
    /// Each line of the list is an entry, which is resolved on the base directory.
    /// The `parent:` (or `base_dir:`) line gives the base directory of the following
    /// entries, and the `current:` line gives the current directory; the path of it
    /// is used as is, and it is also added to the list. The empty lines and the lines
    /// starting with `#` are ignored.
    ///
    /// The current directory is decided by the following order.
    ///
    /// 1. the `current:` line, or `config.current`, if it is given,
    /// 2. the working directory, if it is in the list, or
    /// 3. unknown; see [`Nextable::current_index`](crate::Nextable::current_index).
    ///
    /// # Errors
    ///
    /// - Returns [`Error::Io`] if reading the list failed.
    /// - Returns [`Error::NotFound`] if the given current directory is not in the list.
    pub fn create_from_reader(reader: Box<dyn std::io::Read>, config: &Config) -> Result<Dirs> {
        let mut base_dir = config.base_dir.clone();
        let mut dirs = vec![];
        let mut current = config.current.clone();
        let buf_reader = BufReader::new(reader);
        for line in buf_reader.lines() {
            let line = line.map_err(Error::Io)?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            } else if line.starts_with("base_dir:") || line.starts_with("parent:") {
                base_dir = std::path::PathBuf::from(
                    line.split_once(':').unwrap().1.trim(),
                );
            } else if line.starts_with("current:") {
                let child = PathBuf::from(line.split_once(':').unwrap().1.trim());
                append_dirs(&mut dirs, child.clone(), config);
                current = Some(child);

            } else {
                let path = base_dir.join(line);
                append_dirs(&mut dirs, path, config);
            }
        }
        if let Some(current) = current {
            Dirs::new_with_wd(base_dir, dirs, current)
        } else {
            // No current directory was given by the `current:` line nor the config.
            // Then, the working directory becomes the current one if it is in the
            // list; otherwise the current directory is left unknown.
            let current = find_cwd(&dirs);
            Ok(Dirs {
                entries: dirs,
                parent: base_dir,
                current,
            })
        }
    }
}

fn append_dirs(dirs: &mut Vec<PathBuf>, path: PathBuf, config: &Config) {
    if path.exists() {
        dirs.push(path);
    } else if config.all_target {
        log::info!("append_dirs: Non-existent directory: {}", path.display());
        dirs.push(path);
    } else {
        log::info!("append_dirs: Skipping non-existent directory: {}", path.display());
    }
}

fn build_from_list_file<P: AsRef<Path>>(file: P, config: &Config) -> Result<Dirs> {
    let file = file.as_ref();
    if let Ok(f) = std::fs::File::open(file) {
        let reader = BufReader::new(f);
        DirsFactory::create_from_reader(Box::new(reader), config)
    } else {
        log::error!("build_from_list_file: I/O error: {}", file.display());
        Err(Error::Io(std::io::Error::last_os_error()))
    }
}

fn create_dirs_from_base_path(parent: &Path, config: &Config) -> Result<Dirs> {
    log::trace!("build_dirs_from_base_path(parent={parent:?})");
    let mut errs = vec![];
    let entries = collect_dirs(parent, &mut errs);
    if errs.is_empty() {
        let current = find_current(parent, &entries, &config.current);
        log::info!("build_dirs: siblings={}, current={current:?}", entries.len());
        Ok(Dirs {
            entries,
            parent: parent.to_path_buf(),
            current,
        })
    } else {
        Err(Error::Array(errs))
    }
}

/// Return the index of `current` in `dirs`, or [`None`] if it is not in them.
pub(super) fn find_current(base_dir: &Path, dirs: &[PathBuf], current: &Option<PathBuf>) -> Option<usize> {
    let Some(current) = current else {
        log::debug!("find_current: no current directory was given");
        return None;
    };
    let wd = current.strip_prefix(base_dir)
        .unwrap_or(current);
    let index = dirs.iter().position(|dir|{
        let td = dir.strip_prefix(base_dir);
        td.map(|d| wd == d).unwrap_or(false)
    });
    if index.is_none() {
        log::warn!("find_current: current directory not found in siblings");
    }
    index
}

/// Return the index of the current working directory in `dirs`, or [`None`] if
/// it is not in them.
///
/// Unlike [`find_current`], the paths are compared as the canonicalized ones,
/// since the entries of the list are given by the user, and they may be relative
/// to the working directory.
fn find_cwd(dirs: &[PathBuf]) -> Option<usize> {
    let cwd = std::env::current_dir()
        .and_then(std::fs::canonicalize)
        .ok()?;
    let index = dirs
        .iter()
        .position(|dir| std::fs::canonicalize(dir).is_ok_and(|d| d == cwd));
    match index {
        Some(index) => log::debug!("find_cwd: the current directory is at {index}"),
        None => log::debug!("find_cwd: the current directory is not in the list"),
    }
    index
}

/// Read child directories under parent, push IO errors to errs, and return a sorted list.
fn collect_dirs(parent: &Path, errs: &mut Vec<Error>) -> Vec<PathBuf> {
    log::trace!("collect_dirs(parent={})", parent.display());
    let mut dirs = vec![];
    match parent.read_dir() {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            dirs.push(path);
                        }
                    }
                    Err(e) => {
                        log::error!("collect_dirs: I/O error: {e}");
                        errs.push(Error::Io(e));
                    }
                }
            }
        }
        Err(e) => {
            log::error!("collect_dirs: {}: {e}", parent.display());
            errs.push(Error::Io(e));
        }
    }
    if log::log_enabled!(log::Level::Warn) && dirs.is_empty() {
        log::warn!("collect_dirs: no directories under {}", parent.display());
    }
    dirs.sort();
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_dirs() {
        let dirs = DirsFactory::create("testdata/basic")
            .expect("Failed to create Dirs from testdata/basic");
        assert_eq!(dirs.len(), 26);
        assert_eq!(dirs.parent, PathBuf::from("testdata/basic"));
        assert!(dirs.current().is_none());
    }

    #[test]
    fn test_create_dirs_with() {
        let config = Config::new_with_wd(PathBuf::from("testdata/basic"), false, "c");
        let dirs = DirsFactory::create_with(&config)
            .expect("Failed to create Dirs from testdata/basic");
        assert_eq!(dirs.len(), 26);
        assert_eq!(dirs.parent, PathBuf::from("testdata/basic"));
        let wd = dirs.current().expect("no current directory");
        assert_eq!(wd.path(), Path::new("testdata/basic/c"));
        assert_eq!(wd.index(), 2);
    }

    #[test]
    fn test_create_dirs_from_file() {
        let config = Config::new(PathBuf::from("testdata/basic"), false);
        let dirs = DirsFactory::create_from_file("testdata/basic/dirlist.txt", &config)
            .expect("Failed to create Dirs from testdata/basic/dirlist.txt");
        assert_eq!(dirs.len(), 3);
        assert_eq!(dirs.parent, PathBuf::from("testdata/basic"));
        assert!(dirs.current().is_none());
    }

    /// The `current:` line in the list gives the current directory.
    #[test]
    fn test_create_dirs_from_reader_with_current() {
        let list = "parent: testdata/basic\na\ncurrent: testdata/basic/b\nc\n";
        let dirs = DirsFactory::create_from_reader(
            Box::new(std::io::Cursor::new(list)),
            &Config::new(".", false),
        )
        .expect("Failed to create Dirs from the reader");

        assert_eq!(dirs.len(), 3);
        let current = dirs.current().expect("no current directory");
        assert_eq!(current.path(), Path::new("testdata/basic/b"));
        assert_eq!(current.index(), 1);
    }

    /// The working directory becomes the current one if it is in the list, even
    /// though the list has no `current:` line.
    #[test]
    fn test_create_dirs_from_reader_on_cwd() {
        use crate::Nextable;

        let cwd = std::env::current_dir().unwrap();
        let name = cwd.file_name().unwrap().to_string_lossy().to_string();
        let list = format!("parent: ..\n{name}\nzzz_no_such_dir\n");
        let dirs = DirsFactory::create_from_reader(
            Box::new(std::io::Cursor::new(list)),
            &Config::new(".", true), // --all, for the non-existent directory
        )
        .expect("Failed to create Dirs from the reader");

        assert_eq!(dirs.len(), 2);
        assert_eq!(dirs.current().map(|d| d.index()), Some(0));
        assert_eq!(
            dirs.next(crate::NexterType::Next).map(|d| d.path().to_path_buf()),
            Some(PathBuf::from("../zzz_no_such_dir"))
        );
    }

    /// The current directory is left unknown if it is not in the list.
    #[test]
    fn test_create_dirs_from_reader_without_current() {
        let list = "parent: testdata/basic\na\nb\n";
        let dirs = DirsFactory::create_from_reader(
            Box::new(std::io::Cursor::new(list)),
            &Config::new(".", false),
        )
        .expect("Failed to create Dirs from the reader");

        assert_eq!(dirs.len(), 2);
        assert!(dirs.current().is_none());
    }

    #[test]
    fn test_fail_to_create_dirs_by_non_existent_dir() {
        let _ = DirsFactory::create("src/not_exist_dir")
            .expect_err("Expected error when creating Dirs from a non-existent directory");
    }

    #[test]
    fn test_fail_to_create_dirs_by_non_existent_file() {
        let _ = DirsFactory::create_from_file("src/not_exist_file.txt", &Config::new(".", false))
            .expect_err("Expected error when creating Dirs from a non-existent file");
    }

    #[test]
    fn test_fail_to_create_dirs_by_gives_file() {
        let _ = DirsFactory::create("src/lib.rs")
            .expect_err("Expected error when creating Dirs from a file");
    }

    #[test]
    fn test_fail_to_create_dirs_from_file_by_gives_dir() {
        let _ = DirsFactory::create_from_file("src", &Config::new(".", false))
            .expect_err("Expected error when creating Dirs from a directory");
    }
}
