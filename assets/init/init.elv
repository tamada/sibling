# The functions for traversing the sibling directories.
# Elvish loads them as a module; save this script into the lib directory,
# which is usually ~/.config/elvish/lib.
#
#     sibling --init elvish > ~/.config/elvish/lib/sibling.elv
#
# Then, put the following line into your rc.elv.
#
#     use sibling
#
# The commands are available with the module name, such as "sibling:cdnext".
# To call them by the bare names, bind their function values in your rc.elv.
#
#     var cdnext~ = $sibling:cdnext~
#     var cdprev~ = $sibling:cdprev~
#
# Note that "eval (sibling --init elvish | slurp)" defines nothing; Elvish
# evaluates the code in a temporary namespace, and discards it.
#
# Each function receives the optional count of the traversing, such as
# "cdnext 3". A negative count traverses in the opposite direction.
# The count is ignored by cdfirst, cdlast, cdrand, lsfirst, lslast, and lsrand.

use re

# Print the working directory with its position, such as "/path/to/c (3/26)".
fn -position {
    sibling --progress --type keep -- $pwd
}

# Tell the user why no directory was found. The sibling command itself prints
# the reason of an error, hence, this reports the exhausted list only.
fn -report {|code|
    if (== $code 1) {
        echo "sibling: no more sibling directory" >&2
    }
}

# Return the count of the traversing in the given arguments; 1 by default.
fn -count {|@rest|
    if (> (count $rest) 0) {
        put $rest[0]
    } else {
        put 1
    }
}

# Change the working directory to the found sibling directory.
fn -cd {|type @rest|
    var found = []
    # A non-zero exit status of an external command raises an exception in
    # Elvish; capture it by ?(...) to tell the result from its exit status.
    var err = ?(set found = [(sibling --type $type --step (-count $@rest) -- $pwd)])
    if (not (is $err $ok)) {
        -report $err[reason][exit-status]
        return
    }
    cd $found[0]
    -position
}

# List the entries of the found sibling directory, without changing the
# working directory.
fn -ls {|type @rest|
    var found = []
    var err = ?(set found = [(sibling --type $type --step (-count $@rest) -- $pwd)])
    if (not (is $err $ok)) {
        -report $err[reason][exit-status]
        return
    }
    echo $found[0]
    ls -- $found[0]
}

# Choose a sibling directory with the filter command, such as peco and fzf,
# and change the working directory to it.
fn -cd-with-filter {|filter|
    var selected = []
    var err = ?(set selected = [(sibling --format list --type keep -- $pwd | (external $filter))])
    if (not (is $err $ok)) {
        return
    }
    if (== (count $selected) 0) {
        return
    }
    # Each line of the list format consists of the index, the marker of the
    # current and the next directories, and the path; drop all but the path.
    cd (re:replace '^ *[0-9]+ (\* |> |  )' '' $selected[0])
    -position
}

fn cdnext {|@rest| -cd next $@rest }

fn cdprev {|@rest| -cd previous $@rest }

fn cdfirst {|@rest| -cd first $@rest }

fn cdlast {|@rest| -cd last $@rest }

fn cdrand {|@rest| -cd random $@rest }

fn lsnext {|@rest| -ls next $@rest }

fn lsprev {|@rest| -ls previous $@rest }

fn lsfirst {|@rest| -ls first $@rest }

fn lslast {|@rest| -ls last $@rest }

fn lsrand {|@rest| -ls random $@rest }

fn sibling_peco { -cd-with-filter peco }

fn sibling_fzf { -cd-with-filter fzf }
