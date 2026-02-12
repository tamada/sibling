use std::path::Path;

use crate::cli::{Format, PrintingOpts};
use sibling::{Dir, Dirs};

pub(crate) fn result_string(dirs: &Dirs, next: Option<Dir<'_>>, opts: &PrintingOpts) -> String {
    match opts.format {
        Format::Json => json_string(dirs, next, opts.absolute),
        Format::Csv => csv_string(dirs, next, opts.absolute),
        Format::List => list_string(dirs, next.as_ref(), opts),
        Format::Default => {
            if next.is_none() {
                no_more_dir_string(dirs, opts)
            } else {
                result_string_impl(dirs, next, opts)
            }
        }
    }
}

fn json_string(dirs: &Dirs, next: Option<Dir<'_>>, absolute: bool) -> String {
    let current = dirs.current();
    let next_path = next
        .as_ref()
        .map(|n| path_to_string(Some(n.path()), absolute, dirs.on_dirs()));
    format!(
        r#"{{"current":{{"path":"{}","index":{}}},"next":{{"path":"{}","index":{}}},"total":{}}}"#,
        path_to_string(Some(dirs.current().path()), absolute, dirs.on_dirs()),
        current.index() + 1,
        next_path.unwrap_or_default(),
        next.map_or(-1, |n| i32::try_from(n.index()).unwrap() + 1),
        dirs.len()
    )
}

fn csv_string(dirs: &Dirs, next: Option<Dir<'_>>, absolute: bool) -> String {
    let current = dirs.current();
    format!(
        r#""{}","{}",{},{},{}"#,
        path_to_string(Some(dirs.current().path()), absolute, dirs.on_dirs()),
        path_to_string(next.as_ref().map(Dir::path), absolute, dirs.on_dirs()),
        current.index() + 1,
        next.map_or(-1, |n| i32::try_from(n.index()).unwrap() + 1),
        dirs.len()
    )
}

fn no_more_dir_string(dirs: &Dirs, opts: &PrintingOpts) -> String {
    if opts.parent {
        path_to_string(Some(dirs.parent()), opts.absolute, dirs.on_dirs())
    } else {
        String::from("no more sibling directory")
    }
}

fn list_string(dirs: &Dirs, next: Option<&Dir<'_>>, opts: &PrintingOpts) -> String {
    let mut result = vec![];
    let current = dirs.current();
    let next_index = next.map_or(-1, |n| i32::try_from(n.index()).unwrap());
    for (i, dir) in dirs.directories().enumerate() {
        let prefix = if i32::try_from(i) == Ok(next_index) {
            "> "
        } else if i == current.index() {
            "* "
        } else {
            "  "
        };
        result.push(format!(
            "{:>4} {}{}",
            i + 1,
            prefix,
            path_to_string(Some(dir), opts.absolute, dirs.on_dirs())
        ));
    }
    result.join("\n")
}

fn result_string_impl(dirs: &Dirs, next: Option<Dir<'_>>, opts: &PrintingOpts) -> String {
    if opts.progress {
        format!(
            "{} ({}/{})",
            path_to_string(next.as_ref().map(Dir::path), opts.absolute, dirs.on_dirs()),
            next.map_or(-1, |n| i32::try_from(n.index()).unwrap()) + 1,
            dirs.len()
        )
    } else {
        path_to_string(next.as_ref().map(Dir::path), opts.absolute, dirs.on_dirs()).to_string()
    }
}

pub(crate) fn path_to_string(path: Option<&Path>, absolute: bool, on_dirs: bool) -> String {
    match path {
        Some(p) => {
            if absolute {
                match std::fs::canonicalize(p) {
                    Ok(path) => path.to_string_lossy().to_string(),
                    Err(e) => {
                        log::error!("Failed to get canonical path for {p:?}: {e}");
                        p.to_string_lossy().to_string()
                    },
                }
            } else if on_dirs && !p.is_absolute() {
                Path::new("..").join(p).to_string_lossy().to_string()
            } else {
                p.to_string_lossy().to_string()
            }
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::path_to_string;
    use std::path::Path;

    #[test]
    fn test_pathbuf_to_string_1() {
        let p = Path::new("../testdata");
        let s = path_to_string(Some(p), false, true);
        assert_eq!(s, "../testdata");
    }

    #[test]
    fn test_pathbuf_to_string_2() {
        let p = Path::new("../testdata");
        let s = path_to_string(Some(p), false, false);
        assert_eq!(s, "testdata");
    }

    #[test]
    fn test_pathbuf_to_string_3() {
        let p = Path::new("../testdata");
        let s = path_to_string(Some(p), true, false);
        let abs = std::fs::canonicalize(p).unwrap();
        assert_eq!(s, abs.to_string_lossy());
    }

    #[test]
    fn test_pathbuf_to_string_4() {
        let p = Path::new("../testdata");
        let s = path_to_string(Some(p), true, true);
        let abs = std::fs::canonicalize(p).unwrap();
        assert_eq!(s, abs.to_string_lossy());
    }
}