# Tests for the initialize scripts

The utility commands (`cdnext`, `cdprev`, and the others) are defined by the
shell scripts which `sibling --init` generates, and no unit test covers them;
they run only on their own shells. These tests run them on every supported
shell in a container, and check that all of the shells behave identically.

```bash
tests/init/run.sh              # test every shell
tests/init/run.sh fish elvish  # test the given shells only
just test_init                 # the same as the first one
```

Requirements: docker. The first run takes a while, since it builds the image
and the command; both of them are cached afterwards.

## How it works

1. [`Dockerfile`](Dockerfile) builds an image which has bash, zsh, fish,
   Elvish, and PowerShell. It also has the stand-ins of the filter commands,
   `peco` (chooses the 5th line) and `fzf` (chooses nothing), which let the
   tests call `sibling_peco` and `sibling_fzf` without a terminal.
2. [`run.sh`](run.sh) builds the `sibling` command for the container with the
   `rust` image, and runs the cases of each shell.
3. Each shell loads the initialize script **as the documents tell**; `eval`
   for bash and zsh, `| source` for fish, `Invoke-Expression` for PowerShell,
   and the module for Elvish. Therefore, the documented installation is tested,
   too.
4. The stdout and the stderr of the run are compared with [`expected.out`](expected.out)
   and [`expected.err`](expected.err).

The two streams are compared separately on purpose; the order of writing to
them differs among the shells, hence, merging them makes the result flaky.

## The cases

The cases are written for each shell, and they must be kept in the same order
so that every shell prints the identical output.

| File | Shell |
|---|---|
| [`cases.sh`](cases.sh) | bash and zsh (`TEST_SHELL` tells which one) |
| [`cases.fish`](cases.fish) | fish |
| [`cases.elv`](cases.elv) | Elvish |
| [`cases.ps1`](cases.ps1) | PowerShell |

They traverse `testdata/basic` and `testdata/worried`, and cover the count of
the traversing, the negative count, the directory names containing a space and
a multibyte character, the filter commands, the list file of `-f`, and the
messages of the exhausted list (exit status 1) and the error (3).

`cdrand` is called, but its result is not compared, since it is random; the
cases run `cdfirst` after it to get back to a known directory.

## Updating the expected files

Run the cases on bash, and take its result as the expected one.

```bash
tests/init/run.sh --update
```

Review the difference before committing it; the expected files are the
specification of the utility commands, hence, an unintended change of them
means a regression.
