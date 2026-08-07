# sibling

[![crates.io](https://img.shields.io/badge/crates.io-v3.0.0-orange.svg?logo=rust)](https://crates.io/crates/sibling)
[![License](https://img.shields.io/badge/License-WTFPL-information.svg)](https://github.com/tamada/sibling/blob/main/LICENSE)
[![Version](https://img.shields.io/badge/Version-v3.0.0-information.svg)](https://github.com/tamada/sibling/releases/tag/v3.0.0)

[![build](https://github.com/tamada/sibling/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/sibling/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/sibling/badge.svg?branch=main)](https://coveralls.io/github/tamada/sibling?branch=main)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/tamada/sibling)](https://rust-reportcard.xuri.me/report/github.com/tamada/sibling)

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
- list the entries of the sibling directory:
  - `lsnext`,
  - `lsprev`,
  - `lsfirst`,
  - `lslast`, and
  - `lsrand`
- choose the sibling directory with the filter command:
  - `sibling_peco`, and
  - `sibling_fzf`
- print the sibling directory without moving:
  - `nextdir`, and
  - `prevdir`
- set `NEXTDIR` and `PREVDIR` on every change of the working directory:
  - `sibling_hook_enable`, and
  - `sibling_hook_disable`

Every command allows the integer argument to repeat the traversing, such as `cdnext 3`.
A negative count traverses in the opposite direction.
The count is ignored by the `first`, `last`, and `random` ones.

They also accept `-f FILE`, which traverses the directories listed in the file,
instead of the siblings of the working directory, such as `cdnext -f ~/projects.txt`.
Give the file in an absolute path, since the working directory changes.
The entry of the list where you are becomes the current position, hence, calling
it again finds the next entry of the list.

`nextdir` and `prevdir` print the sibling directory without moving, such as
`cp report.txt "$(nextdir)"`.
`sibling_hook_enable` sets `NEXTDIR` and `PREVDIR` on every change of the working
directory through the hook of your shell; it is not registered by default, since
it runs the command twice on every change.

### :apple: Finder

The AppleScripts in [`assets/applescripts`](assets/applescripts) move the front
Finder window to the next/previous sibling folder, as `cdnext` and `cdprev` do
in the shell. See [its README](assets/applescripts/README.md) for the installation.

## :runner: Usage

`sibling` receives the target directory, and prints the name of its sibling directory with 0 status code.
The siblings are the child directories of the parent directory of the given one, and the given directory itself is included in them.
Which sibling is printed is decided by the traversing type. Available values are: `next`, `previous`, `first`, `last`, `keep` and `random`, default is `next`.

After visiting the final directory, the `sibling` prints nothing and exits with 1.
The `--step` option repeats the traversing; `--step 3` finds the third directory
from the current one. The negative count traverses in the opposite direction
(`--type next --step -1` is the same as `--type previous`), and 0 points the
current directory itself. The step is ignored by the `first`, `last`, `random`,
and `keep` types.

### :vertical_traffic_light: Exit status

| Status | Description |
|:------:|:------------|
| 0 | the next directory was found, and it was printed to stdout. |
| 1 | no more sibling directory was found. |
| 2 | the given command line arguments were wrong. |
| 3 | the command failed; the reason is printed to stderr. |

Note that the `json`, `csv`, and `list` formats print their result even if no more sibling directory was found, since the list of the siblings and the total count are still meaningful. Only the status code tells it.

## :anchor: Installation

### :beer: Homebrew

Install `sibling` via [Homebrew](https://brew.sh), simply run:

```bash
brew tap tamada/brew
brew install sibling
```

And put the snippet of your shell into your shell profile.
The `--init` option accepts `bash`, `zsh`, `fish`, `powershell`, and `elvish`;
other shells are not supported, yet.

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


### :muscle: Compiling yourself

Get source codes by `git clone` or download from [GitHub](https://github.com/tamada/sibling),
then run `cargo build` to build `sibling`.

```shell
git clone https://github.com/tamada/sibling.git # or download from https://github.com/tamada/sibling
cd sibling
cargo build --release # the binary file is put on target/release/sibling.
```

## :smile: About the project

### :memo: Changelog

- [CHANGELOG.md](CHANGELOG.md)
  - Note that v3.0.0 changed the meaning of the `DIR` argument, the exit status,
    and the utility commands of the shell. See the migration in it.

### :scroll: License

- [WTFPL](https://github.com/tamada/sibling/blob/main/LICENSE)
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
