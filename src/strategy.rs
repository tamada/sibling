use crate::{Dir, Nextable};

/// The type of the nexter.
#[derive(Debug)]
pub(super) enum Strategy {
    /// The first sibling directory.
    First(First),
    /// The last sibling directory.
    Last(Last),
    /// The previous sibling directory (step parameter specifies how many to go back).
    Previous(Previous),
    /// The next sibling directory (step parameter specifies how many to go forward).
    Next(Next),
    /// A random sibling directory (step parameter is ignored).
    Random(Random),
    /// Keep the current directory (step parameter is ignored).
    Keep(Keep),
}

/// The trait for nexter.
/// This trait defines the interface for obtaining the next directory.
pub(super) trait Nexter {
    /// Find the next directory with the given step.
    /// If the nexter type is `first`, `last`, `keep`, or `random`, the step parameter is ignored.
    fn next_with<'a>(&self, dirs: &'a impl Nextable, step: i32) -> Option<Dir<'a>>;
}

#[derive(Debug)]
pub(super) struct First {}
#[derive(Debug)]
pub(super) struct Last {}
#[derive(Debug)]
pub(super) struct Previous {}
#[derive(Debug)]
pub(super) struct Next {}
#[derive(Debug)]
pub(super) struct Random {}
#[derive(Debug)]
pub(super) struct Keep {}

impl Nexter for First {
    fn next_with<'a>(&self, dirs: &'a impl Nextable, _step: i32) -> Option<Dir<'a>> {
        if dirs.dirs().is_empty() {
            None
        } else {
            Some(Dir::new(dirs.dirs(), 0))
        }
    }
}

impl Nexter for Last {
    fn next_with<'a>(&self, dirs: &'a impl Nextable, _step: i32) -> Option<Dir<'a>> {
        if dirs.dirs().is_empty() {
            None
        } else {
            let next = dirs.dirs().len() - 1;
            Some(Dir::new(dirs.dirs(), next))
        }
    }
}

impl Nexter for Previous {
    fn next_with<'a>(&self, dirs: &'a impl Nextable, step: i32) -> Option<Dir<'a>> {
        next_impl(dirs, -step)
    }
}

impl Nexter for Next {
    fn next_with<'a>(&self, dirs: &'a impl Nextable, step: i32) -> Option<Dir<'a>> {
        next_impl(dirs, step)
    }
}

impl Nexter for Random {
    fn next_with<'a>(&self, dirs: &'a impl Nextable, _step: i32) -> Option<Dir<'a>> {
        if dirs.dirs().is_empty() {
            log::warn!("Random::next_with: no directory to choose");
            return None;
        }
        let next = rand::random_range(0..dirs.dirs().len());
        log::trace!("Random::next_with -> index {next}");
        Some(Dir::new(dirs.dirs(), next))
    }
}

impl Nexter for Keep {
    fn next_with<'a>(&self, dirs: &'a impl Nextable, _step: i32) -> Option<Dir<'a>> {
        if dirs.dirs().is_empty() {
            log::warn!("Keep::next_with: no directory to keep");
            return None;
        }
        Some(Dir::new(dirs.dirs(), dirs.index()))
    }
}

fn next_impl<'a>(dirs: &'a impl Nextable, step: i32) -> Option<Dir<'a>> {
    let next = i32::try_from(dirs.index()).unwrap() + step;
    let length = i32::try_from(dirs.dirs().len()).unwrap();
    log::trace!(
        "next_impl(step={step}, current={}, next={next})",
        dirs.index()
    );
    if next < 0 || next >= length {
        log::warn!("next_impl: out of range (next={next}, len={length})",);
        None
    } else if next == 0 {
        Some(Dir::new(dirs.dirs(), 0))
    } else if next == length - 1 {
        Some(Dir::new(dirs.dirs(), usize::try_from(length - 1).unwrap()))
    } else {
        Some(Dir::new(dirs.dirs(), usize::try_from(next).unwrap()))
    }
}
