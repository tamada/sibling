# sibling (library)

A small Rust library for traversing sibling directories (directories that share the same parent) in alphabetical order.
From a directory, you can get the “next” one using several strategies: first, last, next, previous, random, and keep.

This library and the `sibling` command are in the same crate: the library sources are in `src`, and the command sources are in `cli`.
See the [GitHub repository](https://github.com/tamada/sibling) for the command line usage.

## Features

- Simple API to enumerate and navigate sibling directories
- Switchable strategies: `First`, `Last`, `Next`, `Previous`, `Random`, `Keep`
- Build the target list from a directory, or from a file/stdin list
- The current position is optional; the traversing is defined even if it is unknown
- Lightweight logging via the `log` crate (initialize a logger in your app)

## How to use

To use this crate, add it as a dependency in your Cargo.toml:

```toml
[dependencies]
sibling = "3"
```

## Concepts

- [`Dirs`] holds the list of the target directories, their parent directory, and
  the index of the current directory in the list.
- The siblings of a directory are the child directories of its **parent** directory,
  and the directory itself is included in them. Therefore, give the parent directory
  as the base directory, and the target directory as the current one.
- The current directory is optional; [`Dirs::current`] returns `None` when it is
  unknown. Such a position means the one **before the first** directory.
- [`Dir`] points an entry of [`Dirs`]. It is also [`Nextable`], hence, you can
  traverse continuously from the found directory.

## Quick Start

```rust
use sibling::factory::{Config, DirsFactory};
use sibling::{NexterType, Nextable};

fn main() -> sibling::Result<()> {
    // the siblings of "testdata/basic/c", that is, the children of "testdata/basic".
    let config = Config::new_with_wd("testdata/basic", false, "testdata/basic/c");
    let dirs = DirsFactory::create_with(&config)?;

    // the next sibling: Some("testdata/basic/d")
    let next = dirs.next(NexterType::Next).map(|d| d.path().to_path_buf());

    // two steps backward: Some("testdata/basic/a")
    let prev2 = dirs
        .next_with(NexterType::Previous, 2)
        .map(|d| d.path().to_path_buf());

    // traverse continuously; the next of the next: Some("testdata/basic/e")
    let next2 = dirs
        .next(NexterType::Next)
        .and_then(|d| d.next(NexterType::Next).map(|d| d.path().to_path_buf()));

    println!("next={next:?}, prev2={prev2:?}, next2={next2:?}");
    Ok(())
}
```

Note that the [`Nextable`] trait must be in the scope to call `next` and `next_with`.

## Inputs

### 1) From a directory

[`DirsFactory::create_with`] lists the child directories of `config.base_dir`, and
finds `config.current` among them.

```rust
use sibling::factory::{Config, DirsFactory};

// with the current directory.
let dirs = DirsFactory::create_with(&Config::new_with_wd("testdata", false, "testdata/basic"))?;
assert_eq!(dirs.current().map(|d| d.index()), Some(0));

// without the current directory; DirsFactory::create is the shorthand of it.
let dirs = DirsFactory::create("testdata")?;
assert!(dirs.current().is_none());
```

`.` as the base directory means the working directory.
[`DirsFactory::create_with`] returns `Error::NotFound` if the given current directory
is not in the listed ones.

### 2) From a file (or stdin)

[`DirsFactory::create_from_file`] builds [`Dirs`] from a list of the directories
(one per line). Give `-` as the file name to read the list from stdin;
[`DirsFactory::create_from_reader`] receives any [`std::io::Read`] instead.

Example: `dirlist.txt`

```txt
# lines starting with '#', and empty lines, are ignored.
parent: /projects
a
current: /projects/b
c
```

| Line | Meaning |
|---|---|
| `parent:` / `base_dir:` | the parent directory; the following entries are resolved on it. It affects only the entries after this line. |
| `current:` | the current directory. Unlike the other entries, the path is used as is, and it is also added to the list. |
| `#`, empty | ignored. |
| otherwise | an entry of the list. |

```rust
use sibling::factory::{Config, DirsFactory};

// Config::new(base_dir, all_target); the entries of the list are resolved on the
// base directory, until the list gives its own one by the "parent:" line.
let config = Config::new("testdata/basic", false);
let dirs = DirsFactory::create_from_file("testdata/basic/dirlist.txt", &config)?;
assert_eq!(dirs.len(), 3); // testdata/basic/{a,b,c}

// Read from stdin by passing "-".
let dirs = DirsFactory::create_from_file("-", &config)?;
```

The entries which do not exist are skipped by default; `all_target` of [`Config`]
makes them the targets, too. Therefore, giving the wrong base directory results
in the empty [`Dirs`], not in an error.

## The current directory

Which directory is the current one is decided by the following order.

1. `config.current`, or the `current:` line of the list,
2. the working directory, if it is in the list (the list input only; the paths are
   compared as the canonicalized ones), or
3. unknown ([`Dirs::current`] returns `None`).

The first one is the directory **given** by the caller; `Error::NotFound` tells
that it is not in the target directories. `Config::not_on_dirs` takes
`NotOnDirs::BeforeFirst` to make it the position before the first one, instead of
the error.

The unknown position means the one before the first directory, hence:

```rust
use sibling::{NexterType, Nextable};
use sibling::factory::DirsFactory;

let dirs = DirsFactory::create("testdata/basic")?; // no current directory

assert!(dirs.current().is_none());
assert!(dirs.next(NexterType::Next).is_some());     // the first directory
assert!(dirs.next(NexterType::Previous).is_none()); // nothing is before it
assert!(dirs.next(NexterType::Keep).is_none());     // nothing to keep
```

## Strategies (NexterType)

| Type | The found directory |
|---|---|
| `First` | the first directory of the list |
| `Last` | the last directory of the list |
| `Next` | the directory of the `step` distance forward |
| `Previous` | the directory of the `step` distance backward |
| `Random` | a directory chosen at random |
| `Keep` | the current directory |

`step` is ignored by all but `Next` and `Previous`. The negative `step` traverses
in the opposite direction (`next` with `-1` is the same as `previous` with `1`),
and 0 points the current directory. Every strategy returns `None` if the resultant
position is out of the list.

## Errors

The library exposes `type Result<T> = std::result::Result<T, Error>`.
Main error variants include:

- `Error::Io(std::io::Error)`: file system I/O error
- `Error::NotFound(PathBuf)`: path does not exist, or the current directory is not in the list
- `Error::NotDir(PathBuf)`: not a directory
- `Error::NotFile(PathBuf)`: not a file
- `Error::NoParent(PathBuf)`: no parent directory
- `Error::UnknownNexterType(String)`: the string is not a name of [`NexterType`]
- `Error::Fatal(String)`: fatal error with message
- `Error::Array(Vec<Error>)`: aggregation of multiple errors

## License

See the workspace `LICENSE`.

[`Config`]: https://docs.rs/sibling/latest/sibling/factory/struct.Config.html
[`Dir`]: https://docs.rs/sibling/latest/sibling/struct.Dir.html
[`Dirs`]: https://docs.rs/sibling/latest/sibling/struct.Dirs.html
[`Dirs::current`]: https://docs.rs/sibling/latest/sibling/struct.Dirs.html#method.current
[`DirsFactory::create_from_file`]: https://docs.rs/sibling/latest/sibling/factory/struct.DirsFactory.html#method.create_from_file
[`DirsFactory::create_from_reader`]: https://docs.rs/sibling/latest/sibling/factory/struct.DirsFactory.html#method.create_from_reader
[`DirsFactory::create_with`]: https://docs.rs/sibling/latest/sibling/factory/struct.DirsFactory.html#method.create_with
[`Nextable`]: https://docs.rs/sibling/latest/sibling/trait.Nextable.html
[`NexterType`]: https://docs.rs/sibling/latest/sibling/enum.NexterType.html
[`std::io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
