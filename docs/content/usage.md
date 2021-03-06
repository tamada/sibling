---
title: ":runner: Usage"
---

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

### :cool: Utilities (bash)

write the below functions into your `.bash_profile`, and restart bash.

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
