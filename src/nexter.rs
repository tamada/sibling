use std::path::PathBuf;

use crate::dirs::Dirs;
use crate::cli::NexterType;

pub trait Nexter {
    fn next(&self, dirs: &mut Dirs, step: i32) -> Option<PathBuf>;
}

pub fn build_nexter(nexter_type: NexterType) -> Box<dyn Nexter> {
    match nexter_type {
        NexterType::First => Box::new(First {}),
        NexterType::Last => Box::new(Last {}),
        NexterType::Previous => Box::new(Previous {}),
        NexterType::Next => Box::new(Next {}),
        NexterType::Random => Box::new(Random {}),
        NexterType::Keep => Box::new(Keep {}),
    }
}

struct First {}
struct Last {}
struct Previous {}
struct Next {}
struct Random {}
struct Keep {}

impl Nexter for First {
    fn next(&self, dirs: &mut Dirs, _step: i32) -> Option<PathBuf> {
        dirs.next = 0;
        dirs.dirs.first().map(|p| p.to_path_buf())
    }
}

impl Nexter for Last {
    fn next(&self, dirs: &mut Dirs, _step: i32) -> Option<PathBuf> {
        dirs.next = (dirs.dirs.len() - 1) as i32;
        dirs.dirs.last().map(|p| p.to_path_buf())
    }
}

impl Nexter for Previous {
    fn next(&self, dirs: &mut Dirs, step: i32) -> Option<PathBuf> {
        next_impl(dirs, step * -1)
    }
}

impl Nexter for Next {
    fn next(&self, dirs: &mut Dirs, step: i32) -> Option<PathBuf> {
        next_impl(dirs, step)
    }
}

impl Nexter for Random {
    fn next(&self, dirs: &mut Dirs, _step: i32) -> Option<PathBuf> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        dirs.next = rng.gen_range(0..dirs.dirs.len()) as i32;
        dirs.dirs.get(dirs.next as usize).map(|p| p.to_path_buf())
    }
}

impl Nexter for Keep {
    fn next(&self, dirs: &mut Dirs, _step: i32) -> Option<PathBuf> {
        dirs.next = dirs.current as i32;
        dirs.dirs.get(dirs.current).map(|p| p.to_path_buf())
    }
}

fn next_impl(dirs: &mut Dirs, step: i32) -> Option<PathBuf> {
    dirs.next = dirs.current as i32 + step;
    let r = if dirs.next < 0 {
        dirs.next = 0;
        dirs.no_more_dir = true;
        dirs.dirs.first()
    } else if dirs.next >= dirs.dirs.len() as i32 {
        dirs.next = (dirs.dirs.len() - 1) as i32;
        dirs.no_more_dir = true;
        dirs.dirs.last()
    } else {
        dirs.dirs.get(dirs.next as usize)
    };
    r.map(|f| f.to_path_buf())
}