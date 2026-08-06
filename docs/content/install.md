---
title: ":anchor: Installation"
date: 2024-07-09
---

## :beer: Homebrew

Install `sibling` via [Homebrew](https://brew.sh), simply run:

```sh
brew install tamada/tap/sibling
```

And put the snippet of your shell into your shell profile.
The `--init` option accepts `bash`, `zsh`, `fish`, and `powershell`;
other shells are not supported, yet.

| Shell | Profile | Snippet |
|---|---|---|
| bash | `.bash_profile` | `eval "$(sibling --init bash)"` |
| zsh | `.zshrc` | `eval "$(sibling --init zsh)"` |
| fish | `config.fish` | `sibling --init fish \| source` |
| PowerShell | `$PROFILE` | `sibling --init powershell \| Out-String \| Invoke-Expression` |


## :muscle: Compiling yourself

Get source codes by `git clone` or download from [GitHub](https://github.com/tamada/sibling),
then run `cargo` to build `sibling`

```sh
$ git clone https://github.com/tamada/sibling.git # or download from https://github.com/tamada/sibling
$ cd sibling
$ cargo build
```

## :briefcase: Requirements

### Development

- Rust 1.78 or later
- Dependencies (See `Cargo.toml`)
  - clap 4.5.23
  - rand 0.8.5
  - rust-embed 8.5.0
