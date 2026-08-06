# sibling CLI

Command-line tool for traversing sibling directories (directories under the same parent) in alphabetical order.

## Overview

When a directory has many subdirectories, moving between them can be tedious. Instead of typing full directory names like `cd ../next_directory`, you can use `sibling` commands like `cdnext` or `cdprev` to quickly switch to the next or previous sibling directory.

This CLI is built on top of the `sibling` library in `../src`.

## Features

- Move to sibling directories with simple commands (`cdnext`, `cdprev`, `cdfirst`, `cdlast`, `cdrand`)
- List sibling directories (`lsnext`, `lsprev`, `lsfirst`, `lslast`, `lsrand`)
- Supports multiple traversal strategies: next, previous, first, last, random, keep
- Works with absolute or relative paths
- Optional progress indicator and parent fallback

## Installation

### Homebrew

```bash
brew tap tamada/brew
brew install sibling
```

Then add the initialization script to your shell profile (`.bash_profile`, `.zshrc`, etc.):

```bash
eval "$(sibling --init bash)"
```

### From Source

```bash
git clone https://github.com/tamada/sibling.git
cd sibling
cargo build --release
```

The binary will be at `target/release/sibling`.

## Usage

```shell
get next/previous sibling directory name.

Usage: sibling [OPTIONS] [DIR]...

Arguments:
  [DIR]...  the target directory

Options:
  -a, --absolute      print the directory name in the absolute path
  -l, --list          list the sibling directories
  -p, --progress      print the progress of traversing directories
  -P, --parent        print parent directory, when no more sibling directories are found
  -s, --step <COUNT>  specify the number of times to execute sibling [default: 1]
      --log <LEVEL>   set the log level [default: warn]
                      [possible values: error, warn, info, debug, trace]
  -t, --type <TYPE>   specify the nexter type [default: next]
                      [possible values: first, last, previous, next, random, keep]
  -i, --input <FILE>  directory list from file, if FILE is "-", reads from stdin.
  -h, --help          Print help (see more with '--help')
  -V, --version       Print version
```

### Examples

```bash
# Move to the next sibling directory
cdnext

# Move 3 directories forward
cdnext -s 3

# Move to the previous sibling
cdprev

# Jump to the first sibling
cdfirst

# Jump to the last sibling
cdlast

# Random sibling
cdrand

# List the next sibling (without changing directory)
lsnext

# List all siblings with progress indicator
sibling -l -p /path/to/dir
```

## Utility Commands

Once initialized with `sibling --init bash`, the following shell functions are available:

- **`cdnext`** / **`cdprev`**: Change to the next/previous sibling directory
- **`cdfirst`** / **`cdlast`**: Change to the first/last sibling directory
- **`cdrand`**: Change to a random sibling directory
- **`lsnext`** / **`lsprev`**: List the next/previous sibling without changing directory
- **`lsfirst`** / **`lslast`**: List the first/last sibling
- **`lsrand`**: List a random sibling

## Input from File or stdin

You can provide a directory list from a file:

```bash
sibling -i dirlist.txt -t next
```

Or from stdin:

```bash
echo -e "parent:/projects\na\nb\nc" | sibling -i - -t next
```

Format:
```
parent:/path/to/parent
dir1
dir2
dir3
```

The optional `parent:` line sets the base directory.

## Build & Test

```bash
# Build the CLI
cargo build --release

# Run tests
cargo test

# Generate shell completions (debug mode only)
cargo run -- --generate-completions bash > completions/sibling.bash
```

## License

See the workspace `LICENSE`.
