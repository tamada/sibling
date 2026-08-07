# Changelog

The notable changes of `sibling` are documented in this file.
The releases before 3.0.0 are on the [releases](https://github.com/tamada/sibling/releases) page.

## [3.0.0] - 2026-08-07

The release which defines the semantics of the command; what the argument means,
what the exit status tells, and where the traversing starts. The previous
releases left them undefined, and the answers were the ones the implementation
happened to give.

Note that v2.0.5 was never released; the changes below are the ones from
[v2.0.4](https://github.com/tamada/sibling/releases/tag/v2.0.4).

### Breaking changes

#### The `DIR` argument means the target directory itself

`sibling DIR` finds the siblings of `DIR`, that is, the child directories of the
parent directory of `DIR`, including `DIR` itself. It listed the child
directories of `DIR` before, hence, `sibling .` never told the sibling of the
working directory.

```console
$ cd photos/2024-05
$ sibling .
photos/2024-06     # 2.x printed a child of 2024-05, or nothing
```

The `--base-path` (`-b`) option overrides the parent directory. It was declared,
but nothing used its value before.

#### The exit status tells the result

| Status | Meaning |
|:---:|---|
| 0 | the next directory was found, and it was printed to stdout |
| 1 | no more sibling directory was found |
| 2 | the given command line arguments were wrong |
| 3 | the command failed; the reason is printed to stderr |

The command always exited with 0 before, even when it printed nothing, or when
it failed. Scripts which ignored the status keep working; scripts which relied
on "always 0" do not.

#### The `default` format prints nothing when no more sibling directory is found

It printed the message `no more sibling directory` to stdout, which `cd "$(sibling)"`
received as a directory name. The message is gone; the status 1 tells it.
The `json`, `csv`, and `list` formats keep printing their result, since the list
of the siblings and the total count are still meaningful.

#### The command is installed as `sibling`

The library and the CLI were merged into a single crate `sibling`, hence, the
binary is `sibling`, not `sibling-cli`, and `cargo install sibling` installs the
command. The library sources are in `src`, and the command ones are in `cli`.

#### The options

| Before | Now |
|---|---|
| `-a`, `--absolute` | `-A`, `--absolute`; `-a` is `--all` now |
| `-i`, `--input <FILE>` | the positional argument takes the file, and `-` means stdin |
| `-P`, `--parent` | removed; check the status 1, and use the parent yourself |
| `[DIR]...` (multiple) | `[DIR\|FILE]` (single) |
| — | `-a`, `--all`: make the non-existent entries of the list the targets |
| — | `-b`, `--base-path <DIR>`: the parent directory of `DIR` |

Note that `--parent` is the alias of `--base-path` now, and its meaning
completely differs from the old `-P`, `--parent`.

#### The utility commands of the shell

The functions which `--init` generates were rewritten; they called the commands
which no longer existed, hence, `cdnext` and every other one had not worked.

- every command receives the count of the traversing, such as `cdnext 3`;
  it was `cdnext -s 3` in the document, which never worked,
- they keep the working directory when no more sibling directory is found,
  instead of moving to the parent directory silently, and
- `sibling_peco` and `sibling_fzf` are documented at last.

#### The library API

- `Dirs::current` returns `Option<Dir>`; the current directory is optional now.
- `Nextable::index` was renamed to `Nextable::current_index`, and it returns
  `Option<usize>`.
- `Dirs::on_dirs` was removed; use `Dirs::current` instead.
- The `NexterFactory`, `Dirs::new_from_file`, and `Dirs::new(path)` of the
  documents had been gone before this release; use `sibling::factory` instead.
  See [LIBRARY.md](LIBRARY.md).

### Added

- The initialize scripts for **fish**, **PowerShell**, and **Elvish**;
  `--init` accepted bash and zsh only.
- The AppleScripts for Finder in [`assets/applescripts`](assets/applescripts),
  which move the front window to the next/previous sibling folder.
- The negative count of `--step`, which traverses in the opposite direction;
  `-s -3` was rejected by the argument parser, while `--step=-3` was accepted.
- The current directory of the list file, which is decided by the `current:`
  line, or the working directory if it is in the list. The first entry was
  treated as the current one before, hence, `next` skipped it.

### Fixed

- The command no longer panics when the list of the target directories is empty;
  `--type random`, `--format json`, and `--format csv` did.
- The paths are escaped in the `json` and `csv` formats. A directory name
  containing a quote or a backslash broke them; a Windows path was enough.
- The I/O error of reading the parent directory is reported, instead of being
  ignored as the empty list.
- The debug build works; it did nothing but generating the completion files.
- Printing to a closed pipe is not an error, such as `sibling --format list | head`.

### Documents

- [LIBRARY.md](LIBRARY.md) was rewritten along with the current API; none of its
  examples compiled. The main examples are the doc tests now.
- The option tables in README.md, the usage page, and cli/README.md were
  regenerated; they had listed the options which did not exist.

### Internal

- [`tests/init`](tests/init) tests the initialize scripts on every supported
  shell in a container, and requires them to behave identically.
