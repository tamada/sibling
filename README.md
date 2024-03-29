# sibling

[![build](https://github.com/tamada/sibling/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/sibling/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/sibling/badge.svg?branch=main)](https://coveralls.io/github/tamada/sibling?branch=main)

[![codebeat badge](https://codebeat.co/badges/aef821a8-27ef-45ec-af37-9bf67a427837)](https://codebeat.co/projects/github-com-tamada-sibling-main)
[![Go Report Card](https://goreportcard.com/badge/github.com/tamada/sibling)](https://goreportcard.com/report/github.com/tamada/sibling)

[![License](https://img.shields.io/badge/License-WTFPL-green.svg)](https://github.com/tamada/sibling/blob/master/LICENSE)
[![Version](https://img.shields.io/badge/Version-1.3.0-green.svg)](https://github.com/tamada/sibling/releases/tag/v1.3.0)

get next/previous sibling directory name.

## :speaking_head: Description

When a directory has too may sub directories, we are tiresome to traverse whole of sub directories.
Because, sometimes we lose where we are.
Ideally, we move directory by specifying ‘next’ or ‘previous,' not directory name.

The command like following makes us tired :-1:.

    cd ../next_directory_name

We should type command like below :+1:.

    cdnext

For this, I implemented `sibling`.

## :runner: Usage

```sh
get next/previous sibling directory name

Usage: sibling [FLAGs] [DIRs...]
FLAGS
    -a, --absolute      print the directory name in the absolute path
    -l, --list          list the sibling directories
    -p, --progress      print the progress traversing directories
    -P, --parent        print parent directory, when no more sibling directories
                        (available on no-console mode)
    -t, --type <TYPE>   specify the traversing type of siblings (default: next, 
                        available: next, previous, first, last and random)

    -h, --help          print this message
    -v, --version       print version
ARGUMENTS
    DIR                 specify the directory. If not specified, the current directory is used
```

`sibling` prints the next directory name with 0 status code.
The next directory is decided by the traversing type. Available values are: `next`, `previous`, `first`, `last` and `random`, default is `next`.

After visiting the final directory, `sibling` prints nothing and exits with non-zero status code.

### :cool: Utilities

The following utility functions are generated by executing `sibling --init bash`.
Therefore, write the snippet (`eval "$(sibling --init bash)"`) into your `.bash_profile`, and restart bash session.

#### :abcd: `change_directory_to_sibling`

```sh
function __change_directory_to_sibling() {
    traversing_type="$1"
    if [ "$1" == "" ]; then
        traversing_type="next"
    fi
    next=$(sibling -t $traversing_type)
    status=$?
    if [ $status -ne 0 ] ; then
        echo "done ($(sibling -p -t $traversing_type))"
        cd ..
    else
        cd $next
        echo "$PWD ($(sibling -p -t $traversing_type))"
    fi
    return $status
}
```

#### :fast_forward: `cdnext`

```sh
function cdnext() {
    __change_directory_to_sibling next
}
```

#### :rewind: `cdprev`

```sh
function cdprev() {
    __change_directory_to_sibling previous
}
```

#### :repeat: `cdrand`

```sh
function cdrand() {
    __change_directory_to_sibling random
}
```

## :anchor: Installation

### :beer: Homebrew

Install `sibling` via [Homebrew](https://brew.sh), simply run:

```sh
brew tap tamada/brew
brew install sibling
```

### Go lang

Install by CLI, run:

```sh
go get github.com/tamada/sibling
```

### :muscle: Compiling yourself

Get source codes by `git clone` or download from [GitHub](https://github.com/tamada/sibling),
then run `make` to build `sibling`

```sh
$ git clone https://github.com/tamada/sibling.git # or download from https://github.com/tamada/sibling
$ cd sibling
$ make
```

### :briefcase: Requirements

#### Development

- Go lang 1.17 and after
- Dependencies (See `go.mod`)
  - github.com/spf13/cobra v1.2.1

## :smile: About the project

### :scroll: License

- [WTFPL](https://github.com/tamada/sibling/blob/master/LICNESE)
  - :+1: Commercial use
  - :+1: Modification
  - :+1: Distribution
  - :+1: Private use

### :man_office_worker: Developers :woman_office_worker:

- [Haruaki Tamada](https://tamada.github.io)
