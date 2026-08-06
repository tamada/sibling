# The shell functions for traversing the sibling directories.
# Install them by putting the following line into your config.fish,
# which is usually ~/.config/fish/config.fish.
#
#     sibling --init fish | source
#
# Each function receives the optional count of the traversing, such as
# "cdnext 3". A negative count traverses in the opposite direction.
# The count is ignored by cdfirst, cdlast, cdrand, lsfirst, lslast, and lsrand.

# Print the found sibling directory of the working directory.
# The exit status is the one of the sibling command; 0 means the directory was
# found, 1 means no more sibling directory, and the others mean an error.
function __sibling_find --argument-names type count
    sibling --type $type --step $count -- $PWD
end

# Print the working directory with its position, such as "/path/to/c (3/26)".
function __sibling_position
    sibling --progress --type keep -- $PWD
end

# Tell the user why no directory was found, and return the given status.
function __sibling_report --argument-names code
    if test $code -eq 1
        echo "sibling: no more sibling directory" >&2
    end
    return $code
end

# Change the working directory to the found sibling directory.
function __sibling_cd --argument-names type count
    test -z "$count"; and set count 1
    set -l next (__sibling_find $type $count)
    set -l code $status
    if test -z "$next"
        # The sibling command prints nothing but the found directory; the empty
        # output means it found none, even if the status was not propagated.
        test $code -eq 0; and set code 1
        __sibling_report $code
        return $code
    end
    cd $next; or return $status
    __sibling_position
end

# List the entries of the found sibling directory, without changing the
# working directory.
function __sibling_ls --argument-names type count
    test -z "$count"; and set count 1
    set -l next (__sibling_find $type $count)
    set -l code $status
    if test -z "$next"
        test $code -eq 0; and set code 1
        __sibling_report $code
        return $code
    end
    echo $next
    ls -- $next
end

# Choose a sibling directory with the filter command, such as peco and fzf,
# and change the working directory to it.
function __sibling_cd_with_filter --argument-names filter
    set -l selected (sibling --format list --type keep -- $PWD | $filter)
    set -l code $status
    if test $code -ne 0 -o -z "$selected"
        return $code
    end
    # Each line of the list format consists of the index, the marker of the
    # current and the next directories, and the path; drop all but the path.
    set selected (string replace -r '^ *[0-9]+ (\* |> |  )' '' -- $selected)
    cd $selected; or return $status
    __sibling_position
end

function cdnext --description "Change to the next sibling directory"
    __sibling_cd next $argv
end

function cdprev --description "Change to the previous sibling directory"
    __sibling_cd previous $argv
end

function cdfirst --description "Change to the first sibling directory"
    __sibling_cd first $argv
end

function cdlast --description "Change to the last sibling directory"
    __sibling_cd last $argv
end

function cdrand --description "Change to a random sibling directory"
    __sibling_cd random $argv
end

function lsnext --description "List the entries of the next sibling directory"
    __sibling_ls next $argv
end

function lsprev --description "List the entries of the previous sibling directory"
    __sibling_ls previous $argv
end

function lsfirst --description "List the entries of the first sibling directory"
    __sibling_ls first $argv
end

function lslast --description "List the entries of the last sibling directory"
    __sibling_ls last $argv
end

function lsrand --description "List the entries of a random sibling directory"
    __sibling_ls random $argv
end

function sibling_peco --description "Choose a sibling directory with peco"
    __sibling_cd_with_filter peco
end

function sibling_fzf --description "Choose a sibling directory with fzf"
    __sibling_cd_with_filter fzf
end
