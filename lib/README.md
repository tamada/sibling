# sibling (library)

A small Rust library for traversing sibling directories (directories that share the same parent) in alphabetical order.
From a current directory, you can get the “next” directory using several strategies: first, last, next, previous, random, and keep.
The CLI in `cli/` consumes this library.

## Features

- Simple API to enumerate and navigate sibling directories
- Switchable strategies: `First`, `Last`, `Next`, `Previous`, `Random`, `Keep`
- Initialize from `.` (current directory), a directory path, or a file/stdin list
- Lightweight logging via the `log` crate (initialize a logger in your app)

## How to use

To use this crate, add it as a dependency in your Cargo.toml:

```toml
[dependencies]
sibling = "2"
```

## Quick Start

```rust
use sibling::{Dirs, NexterFactory, NexterType};

fn main() -> sibling::Result<()> {
    // Build from the current directory
    let dirs = Dirs::new(".")?;

    // Move to the next sibling
    let next = NexterFactory::build(NexterType::Next)
            .next(&dirs)
            .map(|d| d.path());

    // Move two steps backward
    let prev2 = NexterFactory::build(NexterType::Previous)
            .next_with(&dirs, 2)
            .map(|d| d.path());

    println!("next={:?}, prev2={:?}", next, prev2);
    Ok(())
}
```

## Inputs

### 1) From a directory path

```rust
let dirs = sibling::Dirs::new("/path/to/current")?;
```

### 2) From a file (or stdin)

`Dirs::new_from_file()` builds `Dirs` from a list of directories (one per line).
Optionally, the first line may set the parent as `parent:/path/to/parent`.

Example: `dirlist.txt`

```txt
parent:/projects
a
b
c
```

```rust
let dirs = sibling::Dirs::new_from_file("dirlist.txt")?;

// Read from stdin by passing "-"
let dirs = sibling::Dirs::new_from_file("-")?; // stdin
```

## Strategies (NexterType)

- `First`: the first sibling directory
- `Last`: the last sibling directory
- `Next`: move forward by the given step (default: 1)
- `Previous`: move backward by the given step (default: 1)
- `Random`: pick one at random
- `Keep`: keep the current directory

`step` is ignored for all but `Next` and `Previous`.

## Errors

The library exposes `type Result<T> = std::result::Result<T, Error>`.
Main error variants include:

- `Error::Io(std::io::Error)`: file system I/O error
- `Error::NotFound(PathBuf)`: path does not exist
- `Error::NotDir(PathBuf)`: not a directory
- `Error::NotFile(PathBuf)`: not a file
- `Error::NoParent(PathBuf)`: no parent directory
- `Error::Fatal(String)`: fatal error with message
- `Error::Array(Vec<Error>)`: aggregation of multiple errors

## License

See the workspace `LICENSE`.
