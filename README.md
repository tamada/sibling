# sibling

get next/previous sibling directory name.

## :speaking_head: Description

When the directory has too may sub directories, we are tiresome to traverse whole of them.
Because, we lose where we are.
Ideally, we move directory by specifying the next, not directory name.

The command like following makes us tired :-1:.

    cd ../next_directory_name

We should type command like below :+1:.

    cdnext

For this, I implemented `sibling`.

## :runner: Usage

```sh
sibling [OPTIONS] [DIRs...]
OPTIONS
    -a, --absolute      print the directory name in the absolute path.
    -p, --progress      print the progress traversing directories.
    -t, --type <TYPE>   specifies the traversing type of siblings. Default is 'next'.
                        Available values are: 'next', 'previous', and 'random'.
    -P, --parent        print parent directory, when no more sibling directories.

    -h, --help          print this message.
    -v, --version       print version.
ARGUMENTS
    DIR                 specifies directory. If not specified, the current directory is used.
```

`sibling` prints the next directory name with 0 status code.
The next directory is decided by the traversing type. Available values are: `next`, `previous`, and `random`, default is `next`.

After visiting the final directory, `sibling` prints nothing and exits with non-zero status code.

### :cool: Utilities

#### :abcd: `change_directory_to_sibling`

```
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
brew install tamada/brew/sibling
```

### Go lang

Install by CLI, run:

```sh
go get github.com/tamada/sibling
```

### :hammer_and_wrench: Install from source

Get source codes by `git clone` or download from [GitHub](https://github.com/tamada/sibling),
then run `make` to build `sibling`

```sh
$ git clone https://github.com/tamada/sibling.git # or download from https://github.com/tamada/sibling
$ cd sibling
$ make
```

### :briefcase: Requirements

#### Development

* Go lang 1.14.3
* Dependencies (See `go.mod`)
    * github.com/mattn/go-isatty
    * github.com/spf13/pflag

## :smile: About the project

### :scroll: License

* [WTFPL](https://github.com/tamada/sibling/blob/master/LICNESE)
    * :+1: Commercial use
    * :+1: Modification
    * :+1: Distribution
    * :+1: Private use

### :man_office_worker: Developers :woman_office_worker:

* [Haruaki Tamada](https://tamada.github.io)
