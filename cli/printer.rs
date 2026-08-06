use std::path::Path;

use crate::cli::{Format, PrintingOpts};
use sibling::{Dir, Dirs};

/// Build the string to print for the found directory.
///
/// The `default` format prints nothing when no more sibling directory was found;
/// the caller tells it by the exit status. The other formats always print the
/// result, since the list of the siblings and the total count are still meaningful.
pub(crate) fn result_string(dirs: &Dirs, next: Option<Dir<'_>>, opts: &PrintingOpts) -> String {
    match opts.format {
        Format::Json => json_string(dirs, next, opts.absolute),
        Format::Csv => csv_string(dirs, next, opts.absolute),
        Format::List => list_string(dirs, next.as_ref(), opts),
        Format::Default => {
            if next.is_none() {
                String::new()
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
        .map(|n| path_to_string(Some(n.path()), absolute));
    format!(
        r#"{{"current":{{"path":"{}","index":{}}},"next":{{"path":"{}","index":{}}},"total":{}}}"#,
        path_to_string(Some(dirs.current().path()), absolute),
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
        path_to_string(Some(dirs.current().path()), absolute),
        path_to_string(next.as_ref().map(Dir::path), absolute),
        current.index() + 1,
        next.map_or(-1, |n| i32::try_from(n.index()).unwrap() + 1),
        dirs.len()
    )
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
            path_to_string(Some(dir), opts.absolute)
        ));
    }
    result.join("\n")
}

fn result_string_impl(dirs: &Dirs, next: Option<Dir<'_>>, opts: &PrintingOpts) -> String {
    if opts.progress {
        format!(
            "{} ({}/{})",
            path_to_string(next.as_ref().map(Dir::path), opts.absolute),
            next.map_or(-1, |n| i32::try_from(n.index()).unwrap()) + 1,
            dirs.len()
        )
    } else {
        path_to_string(next.as_ref().map(Dir::path), opts.absolute).to_string()
    }
}

/// Convert the given path to the string for printing.
/// The paths in [`Dirs`] are built from the given `DIR` argument, hence, they are
/// printed as is (relative to the current directory) unless `absolute` is true.
pub(crate) fn path_to_string(path: Option<&Path>, absolute: bool) -> String {
    match path {
        Some(p) => {
            if absolute {
                match std::fs::canonicalize(p) {
                    Ok(path) => path.to_string_lossy().to_string(),
                    Err(e) => {
                        log::error!("Failed to get canonical path for {p:?}: {e}");
                        p.to_string_lossy().to_string()
                    }
                }
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
    fn test_path_to_string_relative() {
        let p = Path::new("testdata/basic/c");
        assert_eq!(path_to_string(Some(p), false), "testdata/basic/c");
    }

    #[test]
    fn test_path_to_string_absolute() {
        let p = Path::new("testdata");
        let abs = std::fs::canonicalize(p).unwrap();
        assert_eq!(path_to_string(Some(p), true), abs.to_string_lossy());
    }

    #[test]
    fn test_path_to_string_none() {
        assert_eq!(path_to_string(None, false), "");
        assert_eq!(path_to_string(None, true), "");
    }
}
