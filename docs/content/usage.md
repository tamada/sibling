---
title: ":runner: Usage"
---

## :cool: Utility commands

The `sibling` introduce the following utility commands.

- change working directory the sibling directory.
  - `cdnext`
  - `cdprev`
  - `cdlast`
  - `cdfirst`
  - `cdrand`
- list the sibling directory.
  - `lsnext`
  - `lsprev`
  - `lsfirst`
  - `lslast`
  - `lsrand`

`cdnext` and `cdprev` allow the integer argument to repeat the traversing.

To install the above utility commands into your environment, write the snippet (`eval "$(sibling --init bash)"`) into your `.bash_profile`, and restart the bash session.

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

After visiting the final directory, the `sibling` prints nothing and exits with a non zero status code.
