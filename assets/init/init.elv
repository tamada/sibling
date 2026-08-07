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
#
# They also receive "-f FILE", which traverses the directories listed in the
# file, instead of the siblings of the working directory, such as
# "cdnext -f ~/projects.txt". Give the file in an absolute path, since the
# working directory changes.

use re

# Put the count of the traversing and the file of the given arguments;
# the empty file means the working directory.
fn -parse {|@rest|
    var count = 1
    var file = ''
    while (> (count $rest) 0) {
        if (or (eq $rest[0] -f) (eq $rest[0] --file)) {
            if (< (count $rest) 2) {
                echo "sibling: "$rest[0]": no file is given" >&2
                fail 'no file is given'
            }
            set file = $rest[1]
            set rest = $rest[2..]
        } else {
            set count = $rest[0]
            set rest = $rest[1..]
        }
    }
    put $count $file
}

# Put the target of the traversing; the given file, or the working directory.
fn -target {|file|
    if (eq $file '') { put $pwd } else { put $file }
}

# Print the working directory with its position, such as "/path/to/c (3/26)".
fn -position {|file|
    sibling --progress --type keep -- (-target $file)
}

# Tell the user why no directory was found. The sibling command itself prints
# the reason of an error, hence, this reports the exhausted list only.
fn -report {|code|
    if (== $code 1) {
        echo "sibling: no more sibling directory" >&2
    }
}

# Change the working directory to the found sibling directory.
fn -cd {|type @rest|
    var count file = (-parse $@rest)
    var found = []
    # A non-zero exit status of an external command raises an exception in
    # Elvish; capture it by ?(...) to tell the result from its exit status.
    var err = ?(set found = [(sibling --type $type --step $count -- (-target $file))])
    if (not (is $err $ok)) {
        -report $err[reason][exit-status]
        return
    }
    cd $found[0]
    -position $file
}

# List the entries of the found sibling directory, without changing the
# working directory.
fn -ls {|type @rest|
    var count file = (-parse $@rest)
    var found = []
    var err = ?(set found = [(sibling --type $type --step $count -- (-target $file))])
    if (not (is $err $ok)) {
        -report $err[reason][exit-status]
        return
    }
    echo $found[0]
    ls -- $found[0]
}

# Choose a sibling directory with the filter command, such as peco and fzf,
# and change the working directory to it.
fn -cd-with-filter {|filter @rest|
    var _ file = (-parse $@rest)
    var selected = []
    var err = ?(set selected = [(sibling --format list --type keep -- (-target $file) | (external $filter))])
    if (not (is $err $ok)) {
        return
    }
    if (== (count $selected) 0) {
        return
    }
    # Each line of the list format consists of the index, the marker of the
    # current and the next directories, and the path; drop all but the path.
    cd (re:replace '^ *[0-9]+ (\* |> |  )' '' $selected[0])
    -position $file
}

# Print the found sibling directory, without changing the working directory.
# It tells the result by the exception only, since the caller usually reads it
# by the output capture, such as `cp file (nextdir)`.
fn -print {|type @rest|
    var count file = (-parse $@rest)
    var found = []
    var err = ?(set found = [(sibling --type $type --step $count -- (-target $file))])
    if (not (is $err $ok)) {
        return
    }
    echo $found[0]
}

# Whether the hook is registered; the callback of $after-chdir cannot be
# removed once it is added, hence, it asks this variable every time.
var hook-enabled = $false

# Set NEXTDIR and PREVDIR to the siblings of the working directory; they become
# empty when no such directory is found.
fn -hook {
    if (not $hook-enabled) {
        return
    }
    var next = []
    var err = ?(set next = [(sibling --type next -- $pwd 2>/dev/null)])
    var prev = []
    set err = ?(set prev = [(sibling --type previous -- $pwd 2>/dev/null)])
    set-env NEXTDIR (if (> (count $next) 0) { put $next[0] } else { put '' })
    set-env PREVDIR (if (> (count $prev) 0) { put $prev[0] } else { put '' })
}

# Run the hook on every change of the working directory. It is not registered
# by default, since it runs the sibling command twice on every change; reading
# a directory of ten thousand entries costs about 30 milliseconds.
fn sibling_hook_enable {
    if (not $hook-enabled) {
        set after-chdir = [$@after-chdir {|_| -hook }]
    }
    set hook-enabled = $true
    -hook
}

fn sibling_hook_disable {
    set hook-enabled = $false
    unset-env NEXTDIR
    unset-env PREVDIR
}

fn nextdir {|@rest| -print next $@rest }

fn prevdir {|@rest| -print previous $@rest }

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

fn sibling_peco {|@rest| -cd-with-filter peco $@rest }

fn sibling_fzf {|@rest| -cd-with-filter fzf $@rest }
