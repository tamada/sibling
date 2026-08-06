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

/// Return the 1-origin index of the given directory, or -1 if it is [`None`].
fn index_of(dir: Option<&Dir<'_>>) -> i32 {
    dir.map_or(-1, |d| i32::try_from(d.index()).unwrap() + 1)
}

/// Escape the given string as a JSON string value (RFC 8259).
/// A directory name can contain a double quote, a backslash, and even a control
/// character; they break the resultant JSON unless they are escaped.
fn json_escape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\u{08}' => result.push_str("\\b"),
            '\u{0c}' => result.push_str("\\f"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c < '\u{20}' => result.push_str(&format!("\\u{:04x}", c as u32)),
            c => result.push(c),
        }
    }
    result
}

/// Escape the given string as a CSV field (RFC 4180).
/// Every field of the result is quoted, hence, the double quotes in it are doubled.
fn csv_escape(s: &str) -> String {
    s.replace('"', "\"\"")
}

fn json_string(dirs: &Dirs, next: Option<Dir<'_>>, absolute: bool) -> String {
    let current = dirs.current();
    format!(
        r#"{{"current":{{"path":"{}","index":{}}},"next":{{"path":"{}","index":{}}},"total":{}}}"#,
        json_escape(&path_to_string(current.as_ref().map(Dir::path), absolute)),
        index_of(current.as_ref()),
        json_escape(&path_to_string(next.as_ref().map(Dir::path), absolute)),
        index_of(next.as_ref()),
        dirs.len()
    )
}

fn csv_string(dirs: &Dirs, next: Option<Dir<'_>>, absolute: bool) -> String {
    let current = dirs.current();
    format!(
        r#""{}","{}",{},{},{}"#,
        csv_escape(&path_to_string(current.as_ref().map(Dir::path), absolute)),
        csv_escape(&path_to_string(next.as_ref().map(Dir::path), absolute)),
        index_of(current.as_ref()),
        index_of(next.as_ref()),
        dirs.len()
    )
}

fn list_string(dirs: &Dirs, next: Option<&Dir<'_>>, opts: &PrintingOpts) -> String {
    let mut result = vec![];
    let current_index = index_of(dirs.current().as_ref());
    let next_index = index_of(next);
    for (i, dir) in dirs.directories().enumerate() {
        let index = i32::try_from(i).unwrap() + 1;
        let prefix = if index == next_index {
            "> "
        } else if index == current_index {
            "* "
        } else {
            "  "
        };
        result.push(format!(
            "{index:>4} {prefix}{}",
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
            index_of(next.as_ref()),
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
    use super::{path_to_string, result_string};
    use crate::cli::{Format, PrintingOpts};
    use sibling::{Dirs, NexterType, Nextable};
    use std::path::{Path, PathBuf};

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

    #[test]
    fn test_json_escape() {
        use super::json_escape;

        assert_eq!(json_escape("/tmp/plain"), "/tmp/plain");
        assert_eq!(json_escape(r#"/tmp/qu"ote"#), r#"/tmp/qu\"ote"#);
        assert_eq!(json_escape(r"C:\Users\tamada"), r"C:\\Users\\tamada");
        assert_eq!(json_escape("/tmp/tab\tx"), r"/tmp/tab\tx");
        assert_eq!(json_escape("/tmp/lf\nx"), r"/tmp/lf\nx");
        assert_eq!(json_escape("/tmp/bell\u{07}x"), r"/tmp/bell\u0007x");
        assert_eq!(json_escape("/tmp/絵文字_👍"), "/tmp/絵文字_👍");
    }

    #[test]
    fn test_csv_escape() {
        use super::csv_escape;

        assert_eq!(csv_escape("/tmp/plain"), "/tmp/plain");
        assert_eq!(csv_escape(r#"/tmp/qu"ote"#), r#"/tmp/qu""ote"#);
        assert_eq!(csv_escape(r"C:\Users\tamada"), r"C:\Users\tamada");
    }

    /// The directories are built on memory, since a name containing a double
    /// quote is not available on every platform.
    fn dirs_of(paths: &[&str]) -> Dirs {
        Dirs::new(
            PathBuf::from("/tmp"),
            paths.iter().map(PathBuf::from).collect(),
        )
    }

    fn opts_of(format: Format) -> PrintingOpts {
        PrintingOpts {
            format,
            absolute: false,
            progress: false,
        }
    }

    #[test]
    fn test_json_string_with_worried_names() {
        let dirs = dirs_of(&[r#"/tmp/qu"ote"#, r"/tmp/back\slash"]);
        let next = dirs.next(NexterType::Next);

        assert_eq!(
            result_string(&dirs, next, &opts_of(Format::Json)),
            r#"{"current":{"path":"","index":-1},"next":{"path":"/tmp/qu\"ote","index":1},"total":2}"#
        );
    }

    #[test]
    fn test_csv_string_with_worried_names() {
        let dirs = dirs_of(&[r#"/tmp/qu"ote"#, r"/tmp/back\slash"]);
        let next = dirs.next(NexterType::Next);

        assert_eq!(
            result_string(&dirs, next, &opts_of(Format::Csv)),
            r#""","/tmp/qu""ote",-1,1,2"#
        );
    }
}
