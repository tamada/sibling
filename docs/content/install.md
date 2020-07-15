---
title: ":anchor: Installation"
---

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
