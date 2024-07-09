# sibling

[![build](https://github.com/tamada/sibling/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/sibling/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/sibling/badge.svg?branch=main)](https://coveralls.io/github/tamada/sibling?branch=main)

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/tamada/sibling)](https://rust-reportcard.xuri.me/report/github.com/tamada/sibling)

[![License](https://img.shields.io/badge/License-WTFPL-green.svg)](https://github.com/tamada/sibling/blob/master/LICENSE)
[![Version](https://img.shields.io/badge/Version-2.0.0--beta--1-green.svg)](https://github.com/tamada/sibling/releases/tag/v2.0.0-beta-1)

get the next/previous sibling directory name.

## :speaking_head: Description

When a directory has too many sub-directories, we are tiresome to traverse the whole of sub-directories.
Because, sometimes we lose where we are.
Ideally, we move the directory by specifying ‘next’ or ‘previous,' not the directory name.

The command like the following makes us tired :-1:.

    cd ../next_directory_name

We should type the command like below :+1:.

    cdnext

For this, I implemented `sibling`.

## Utility commands

The `sibling` introduces the following utility commands.

- change the working directory to the sibling directory:
  - `cdnext`,
  - `cdprev`,
  - `cdfirst`,
  - `cdlast`, and
  - `cdrand`
- list the sibling directory:
  - `lsnext`,
  - `lsprev`,
  - `lsfirst`,
  - `lslast`, and
  - `lsrand`

The `cdnext` and the `cdprev` allow the integer argument to repeat the traversing.

## :runner: Usage

```sh
get next/previous sibling directory name.

Usage: sibling [OPTIONS] [DIR]

Arguments:
  [DIR]  the directory for listing the siblings [default: .]

Options:
  -a, --absolute      print the directory name in the absolute path
  -l, --list          list the sibling directories
  -p, --progress      print the progress of traversing directories
  -P, --parent        print parent directory, when no more sibling directories
  -s, --step <COUNT>  specify the number of times to execute sibling [default: 1]
  -t, --type <TYPE>   specify the nexter type [default: next]
                      [possible values: first, last, previous, next, random, keep]
  -h, --help          Print help
  -V, --version       Print version
```

`sibling` prints the next directory name with 0 status code.
The next directory is decided by the traversing type. Available values are: `next`, `previous`, `first`, `last`, `keep` and `random`, default is `next`.

After visiting the final directory, the `sibling` prints nothing and exits with a non-zero status code.

## :anchor: Installation

### :beer: Homebrew

Install `sibling` via [Homebrew](https://brew.sh), simply run:

```sh
brew tap tamada/brew
brew install sibling
```

And put the following snipeets into your shell profile (e.g., `.bash_profile`, or `.zshrc`)
The `--init` option accepts only `bash`.
Other shell scripts are not supported, yet.

```shell
$(sibling --init bash)
```

### :muscle: Compiling yourself

Get source codes by `git clone` or download from [GitHub](https://github.com/tamada/sibling),
then run `cargo build` to build `sibling`.

```sh
$ git clone https://github.com/tamada/sibling.git # or download from https://github.com/tamada/sibling
$ cd sibling
$ cargo build --release # the binary file is put on target/release/sibling.
```

### :briefcase: Requirements

#### Development

- Rust 1.78 or later
- Dependencies (See `Cargo.toml`)
  - clap 4.5.5
  - rand 0.8.5
  - rust-embed 8.4.0

## :smile: About the project

### :scroll: License

- [WTFPL](https://github.com/tamada/sibling/blob/master/LICNESE)
  - :+1: Commercial use
  - :+1: Modification
  - :+1: Distribution
  - :+1: Private use

### :man_office_worker: Developers :woman_office_worker:

- [Haruaki Tamada](https://tamada.github.io)

### :link: Useful links for directory traversing

- [cdargs](https://github.com/cbxbiker61/cdargs)
  - Directory bookmarking system - Enhanced cd utilities
- [Is there a directory history for bash?](https://superuser.com/questions/299694/is-there-a-directory-history-for-bash)
