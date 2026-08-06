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

Usage: sibling [OPTIONS] [DIR|FILE]

Arguments:
  [DIR|FILE]  the directory to find its siblings, or the file of directory list [default: .]

Options:
  -f, --format <FORMAT>  print the result in the specified format [default: default]
                         [possible values: json, csv, list, default]
  -A, --absolute         print the directory name in the absolute path
  -p, --progress         print the progress of traversing directories
  -s, --step <COUNT>     specify the number of times to execute sibling [default: 1]
  -t, --type <TYPE>      specify the nexter type [default: next]
                         [possible values: first, last, previous, next, random, keep]
  -a, --all              Set the targets to all directories from the given list.
                         By default, the sibling skips non-existent directories.
  -b, --base-path <DIR>  specify the parent directory of DIR
                         (default: the parent directory of DIR)
      --log <LEVEL>      set the log level [default: warn]
                         [possible values: error, warn, info, debug, trace]
  -h, --help             Print help
  -V, --version          Print version

Exit status:
  0  the next directory was found (printed to stdout),
  1  no more sibling directory was found,
  2  the given command line arguments were wrong, and
  3  the command failed (the reason is printed to stderr).
```

The `DIR` argument is the target directory itself; its siblings are the child directories
of its parent directory, and `DIR` itself is included in them.

### Exit status

The command is designed to be used from a shell function, hence, the caller can tell
the result from the status code.

```bash
if next=$(sibling "$PWD"); then
    cd "$next"
else
    case $? in
        1) echo "no more sibling directory" ;;
        *) echo "sibling: failed" ;;
    esac
fi
```

Note that the `json`, `csv`, and `list` formats print their result even if no more sibling
directory was found; only the status code tells it.

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
