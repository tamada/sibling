---
title: ":anchor: Installation"
---

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
eval "$(sibling --init bash)"
```

### :muscle: Compiling yourself

Get source codes by `git clone` or download from [GitHub](https://github.com/tamada/sibling),
then run `cargo` to build `sibling`

```sh
$ git clone https://github.com/tamada/sibling.git # or download from https://github.com/tamada/sibling
$ cd sibling
$ cargo build
```

### :briefcase: Requirements

#### Development

- Rust 1.78 or later
- Dependencies (See `Cargo.toml`)
  - clap 4.5.5
  - rand 0.8.5
  - rust-embed 8.4.0
