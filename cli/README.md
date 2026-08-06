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

Then add the initialization script of your shell to your shell profile.

| Shell | Profile | Snippet |
|---|---|---|
| bash | `.bash_profile` | `eval "$(sibling --init bash)"` |
| zsh | `.zshrc` | `eval "$(sibling --init zsh)"` |
| fish | `config.fish` | `sibling --init fish \| source` |
| PowerShell | `$PROFILE` | `sibling --init powershell \| Out-String \| Invoke-Expression` |
| Elvish | `rc.elv` | `use sibling` (see the note below) |

Elvish loads the functions as a module, not by evaluating them.
Save the script into the lib directory, which is usually `~/.config/elvish/lib`,
and use it in your `rc.elv`; the commands are namespaced, such as `sibling:cdnext`.

```console
$ sibling --init elvish > ~/.config/elvish/lib/sibling.elv
```

```elvish
use sibling
# to call them by the bare names
var cdnext~ = $sibling:cdnext~
var cdprev~ = $sibling:cdprev~
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
                         The negative count traverses in the opposite direction,
                         and 0 means the current directory.
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
cdnext 3

# Move to the previous sibling
cdprev

# Move 3 directories forward, again; a negative count reverses the direction
cdprev -3

# Jump to the first sibling
cdfirst

# Jump to the last sibling
cdlast

# Random sibling
cdrand

# List the entries of the next sibling (without changing directory)
lsnext

# List all siblings of the given directory, with the current and the next markers
sibling --format list /path/to/dir
```

## Utility Commands

Once initialized with `sibling --init <SHELL>` (`bash`, `zsh`, `fish`,
`powershell`, and `elvish` are available), the following functions are available:

- **`cdnext`** / **`cdprev`**: Change to the next/previous sibling directory
- **`cdfirst`** / **`cdlast`**: Change to the first/last sibling directory
- **`cdrand`**: Change to a random sibling directory
- **`lsnext`** / **`lsprev`**: List the entries of the next/previous sibling, without changing directory
- **`lsfirst`** / **`lslast`**: List the entries of the first/last sibling
- **`lsrand`**: List the entries of a random sibling
- **`sibling_peco`** / **`sibling_fzf`**: Choose a sibling directory with [peco](https://github.com/peco/peco) or [fzf](https://github.com/junegunn/fzf), and change to it

Every function receives the optional count of the traversing, such as `cdnext 3`;
a negative count traverses in the opposite direction. The count is ignored by the
`first`, `last`, and `random` ones.

The `cd` functions print the directory with its position, such as
`/path/to/c (3/26)`. They keep the working directory and return 1 when no more
sibling directory is found; the message is printed to stderr.

## Input from File or stdin

Instead of a directory, you can give a file which lists the directories to traverse.
Give `-` as the file name to read the list from stdin.

```bash
sibling dirlist.txt -t next
printf 'parent: /projects\na\nb\nc\n' | sibling - -t next
```

### Format of the list

```
# lines starting with '#', and empty lines, are ignored.
parent: /path/to/parent    # or "base_dir:"; the following names are resolved on it.
dir1
current: /path/to/parent/dir2   # the current directory (optional).
dir3
```

| Line | Meaning |
|---|---|
| `parent:` / `base_dir:` | the parent directory; the following entries are resolved on it. It affects only the entries after this line. |
| `current:` | the current directory. Unlike the other entries, the path is used as is, and it is also added to the list. |
| `#`, empty | ignored. |
| otherwise | an entry of the list. |

By default, the entries which do not exist are skipped; `--all` makes them the
targets, too.

### The current directory in the list

Which directory is the current one is decided by the following order.

1. the `current:` line in the list, if it is given,
2. the working directory, if it is in the list (the paths are compared as the
   canonicalized ones), or
3. unknown.

The unknown current position means "before the first entry"; `next` finds the
first entry from it (`--step 3` finds the third one), while `previous` and `keep`
find nothing and exit with 1. `first`, `last`, and `random` are not affected.

```console
$ cat dirlist.txt
parent: /projects
a
b
c
$ sibling dirlist.txt          # not in /projects/*, hence, the first entry
/projects/a
$ cd /projects/b && sibling ~/dirlist.txt   # the working directory is in the list
/projects/c
```

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
