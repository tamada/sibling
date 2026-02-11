use std::path::Path;

use crate::cli::{Format, PrintingOpts};
use sibling::{Dir, Dirs};

pub(crate) fn result_string(
    dirs: &Dirs,
    next: Option<Dir<'_>>,
    opts: &PrintingOpts,
) -> String {
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
        },
    }
}

fn json_string(dirs: &Dirs, next: Option<Dir<'_>>, absolute: bool) -> String {
    let current = dirs.current();
    let next_path = next.as_ref().map(|n| pathbuf_to_string(Some(n.path()), absolute));
    format!(
        r#"{{"current":{{"path":"{}","index":{}}},"next":{{"path":"{}","index":{}}},"total":{}}}"#,
        pathbuf_to_string(Some(dirs.current().path()), absolute),
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
        pathbuf_to_string(Some(dirs.current().path()), absolute),
        pathbuf_to_string(next.as_ref().map(Dir::path), absolute),
        current.index() + 1,
        next.map_or(-1, |n| i32::try_from(n.index()).unwrap() + 1),
        dirs.len()
    )
}

fn no_more_dir_string(dirs: &Dirs, opts: &PrintingOpts) -> String {
    if opts.parent {
        pathbuf_to_string(Some(dirs.parent()), opts.absolute)
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
            pathbuf_to_string(Some(dir), opts.absolute)
        ));
    }
    result.join("\n")
}

fn result_string_impl(dirs: &Dirs, next: Option<Dir<'_>>, opts: &PrintingOpts) -> String {
    if opts.progress {
        format!(
            "{} ({}/{})",
            pathbuf_to_string(next.as_ref().map(Dir::path), opts.absolute),
            next.map_or(-1, |n| i32::try_from(n.index()).unwrap()) + 1,
            dirs.len()
        )
    } else {
        pathbuf_to_string(next.as_ref().map(Dir::path), opts.absolute).to_string()
    }
}

fn pathbuf_to_string(path: Option<&Path>, absolute: bool) -> String {
    match path {
        Some(p) => {
            if absolute {
                std::fs::canonicalize(p)
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            } else {
                p.to_string_lossy().to_string()
            }
        }
        None => String::new(),
    }
}
