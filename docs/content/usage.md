---
title: ":runner: Usage"
date: 2025-12-03
---

## :cool: Utility commands

The `sibling` introduce the following utility commands.

- change working directory the sibling directory.
  - `cdnext`
  - `cdprev`
  - `cdlast`
  - `cdfirst`
  - `cdrand`
- list the entries of the sibling directory.
  - `lsnext`
  - `lsprev`
  - `lsfirst`
  - `lslast`
  - `lsrand`
- choose the sibling directory with the filter command.
  - `sibling_peco`
  - `sibling_fzf`
- print the sibling directory without moving.
  - `nextdir`
  - `prevdir`
- set `NEXTDIR` and `PREVDIR` on every change of the working directory.
  - `sibling_hook_enable`
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

To install the above utility commands into your environment, write the snippet of your shell into your shell profile, and restart the session.
See the [installation](../install) page for the snippets; `bash`, `zsh`, `fish`, `powershell`, and `elvish` are available as the argument of the `--init` option.

## :apple: Finder

The AppleScripts move the front Finder window to the next/previous sibling folder,
as `cdnext` and `cdprev` do in the shell.
Get them from [`assets/applescripts`](https://github.com/tamada/sibling/tree/main/assets/applescripts)
in the repository, and see its README for the installation.

## :runner: Usage

```sh
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
      --not-on-dirs <ACTION>
                         specify the action when the current directory is not in
                         the target directories [default: error]
                         [possible values: error, before-first]
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

`sibling` receives the target directory, and prints the name of its sibling directory with 0 status code.
The siblings are the child directories of the parent directory of the given one, and the given directory itself is included in them.
Which sibling is printed is decided by the traversing type. Available values are: `next`, `previous`, `first`, `last`, `keep` and `random`, default is `next`.

After visiting the final directory, the `sibling` prints nothing and exits with 1.
The `--step` option repeats the traversing; `--step 3` finds the third directory
from the current one. The negative count traverses in the opposite direction
(`--type next --step -1` is the same as `--type previous`), and 0 points the
current directory itself. The step is ignored by the `first`, `last`, `random`,
and `keep` types.

Note that the `json`, `csv`, and `list` formats print their result even in that case, since the list of the siblings and the total count are still meaningful.
