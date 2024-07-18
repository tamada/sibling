use std::path::PathBuf;

use crate::cli::{PrintingOpts, Result};
use crate::dirs::Dirs;

pub(crate) fn result_string(dirs: &Dirs, opts: &PrintingOpts) -> Result<String> {
    if opts.csv {
        csv_string(dirs, opts.absolute)
    } else if dirs.no_more_dir {
        no_more_dir_string(dirs, opts)
    } else if opts.list {
        list_string(dirs, opts)
    } else {
        result_string_impl(dirs, opts)
    }
}

fn csv_string(dirs: &Dirs, absolute: bool) -> Result<String> {
    Ok(format!(r##""{}","{}",{},{},{}"##, 
            pathbuf_to_string(Some(dirs.current_path()), absolute), 
            pathbuf_to_string(dirs.next_path(), absolute),
            dirs.current + 1, dirs.next + 1, dirs.dirs.len()))
}

fn no_more_dir_string(dirs: &Dirs, opts: &PrintingOpts) -> Result<String> {
    if opts.parent {
        Ok(pathbuf_to_string(Some(dirs.parent.clone()), opts.absolute))
    } else {
        Ok(String::from("no more sibling directory"))
    }
}

fn list_string(dirs: &Dirs, opts: &PrintingOpts) -> Result<String> {
    let mut result = vec![];
    for (i, dir) in dirs.dirs.iter().enumerate() {
        let prefix = if i == dirs.next as usize { "> " }
        else if i == dirs.current {
            "* " 
        } else {
            "  "
        };
        result.push(format!("{:>4} {}{}", i + 1, prefix, pathbuf_to_string(Some(dir.to_path_buf()), opts.absolute)));
    }
    Ok(result.join("\n"))
}

fn result_string_impl(dirs: &Dirs, opts: &PrintingOpts) -> Result<String> {
    let r = if opts.progress {
        format!("{} ({}/{})", pathbuf_to_string(dirs.next_path(), opts.absolute), dirs.next + 1, dirs.dirs.len())
    } else {
        pathbuf_to_string(dirs.next_path(), opts.absolute).to_string()
    };
    Ok(r)
}

fn pathbuf_to_string(path: Option<PathBuf>, absolute: bool) -> String {
    match path {
        Some(p) => {
            if absolute {
                std::fs::canonicalize(p).unwrap().to_string_lossy().to_string()
            } else {
                p.to_string_lossy().to_string()
            }
        },
        None => "".to_string(),
    }
}