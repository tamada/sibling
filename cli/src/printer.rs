use std::path::PathBuf;

use crate::cli::PrintingOpts;
use sibling::{Dir, Dirs, Result};

pub(crate) fn result_string(
    dirs: &Dirs,
    next: Option<Dir<'_>>,
    opts: &PrintingOpts,
) -> Result<String> {
    if opts.csv {
        csv_string(dirs, next, opts.absolute)
    } else if next.is_some() && next.clone().unwrap().is_last_item() {
        no_more_dir_string(dirs, opts)
    } else if opts.list {
        list_string(dirs, next, opts)
    } else {
        result_string_impl(dirs, next, opts)
    }
}

fn csv_string(dirs: &Dirs, next: Option<Dir<'_>>, absolute: bool) -> Result<String> {
    let current = dirs.current();
    Ok(format!(
        r##""{}","{}",{},{},{}"##,
        pathbuf_to_string(Some(dirs.current().path()), absolute),
        pathbuf_to_string(next.clone().map(|p| p.path()), absolute),
        current.index() + 1,
        next.map(|n| n.index() as i32 + 1).unwrap_or(-1),
        dirs.len()
    ))
}

fn no_more_dir_string(dirs: &Dirs, opts: &PrintingOpts) -> Result<String> {
    if opts.parent {
        Ok(pathbuf_to_string(Some(dirs.parent()), opts.absolute))
    } else {
        Ok(String::from("no more sibling directory"))
    }
}

fn list_string(dirs: &Dirs, next: Option<Dir<'_>>, opts: &PrintingOpts) -> Result<String> {
    let mut result = vec![];
    let current = dirs.current();
    let next_index = next.clone().map(|n| n.index() as i32).unwrap_or(-1);
    for (i, dir) in dirs.directories().enumerate() {
        let prefix = if i as i32 == next_index {
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
            pathbuf_to_string(Some(dir.to_path_buf()), opts.absolute)
        ));
    }
    Ok(result.join("\n"))
}

fn result_string_impl(dirs: &Dirs, next: Option<Dir<'_>>, opts: &PrintingOpts) -> Result<String> {
    let r = if opts.progress {
        format!(
            "{} ({}/{})",
            pathbuf_to_string(next.clone().map(|n| n.path()), opts.absolute),
            next.map(|n| n.index() as i32).unwrap_or(-1) + 1,
            dirs.len()
        )
    } else {
        pathbuf_to_string(next.map(|n| n.path()), opts.absolute).to_string()
    };
    Ok(r)
}

fn pathbuf_to_string(path: Option<PathBuf>, absolute: bool) -> String {
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
        None => "".to_string(),
    }
}
