# The shell functions for traversing the sibling directories.
# Install them by putting the following line into your config.fish,
# which is usually ~/.config/fish/config.fish.
#
#     sibling --init fish | source
#
# Each function receives the optional count of the traversing, such as
# "cdnext 3". A negative count traverses in the opposite direction.
# The count is ignored by cdfirst, cdlast, cdrand, lsfirst, lslast, and lsrand.
#
# They also receive "-f FILE", which traverses the directories listed in the
# file, instead of the siblings of the working directory, such as
# "cdnext -f ~/projects.txt". Give the file in an absolute path, since the
# working directory changes.

# Print the count of the traversing, and the file if it is given.
function __sibling_parse
    set -l count 1
    set -l file
    while set -q argv[1]
        switch $argv[1]
            case -f --file
                if not set -q argv[2]
                    echo "sibling: $argv[1]: no file is given" >&2
                    return 2
                end
                set file $argv[2]
                set -e argv[1..2]
            case '*'
                set count $argv[1]
                set -e argv[1]
        end
    end
    echo $count
    if test -n "$file"
        echo $file
    end
end

# Print the target of the traversing; the given file, or the working directory.
function __sibling_target --argument-names file
    if test -n "$file"
        echo $file
    else
        echo $PWD
    end
end

# Print the found sibling directory of the working directory, or of the given
# list file. The exit status is the one of the sibling command; 0 means the
# directory was found, 1 means no more sibling directory, and the others mean
# an error.
function __sibling_find --argument-names type count file
    sibling --type $type --step $count -- (__sibling_target $file)
end

# Print the working directory with its position, such as "/path/to/c (3/26)".
function __sibling_position --argument-names file
    sibling --progress --type keep -- (__sibling_target $file)
end

# Tell the user why no directory was found, and return the given status.
function __sibling_report --argument-names code
    if test $code -eq 1
        echo "sibling: no more sibling directory" >&2
    end
    return $code
end

# Change the working directory to the found sibling directory.
function __sibling_cd --argument-names type
    set -e argv[1]
    set -l parsed (__sibling_parse $argv)
    or return $status
    set -l count $parsed[1]
    set -l file ''
    if test (count $parsed) -gt 1
        set file $parsed[2]
    end
    set -l next (__sibling_find $type $count $file)
    set -l code $status
    if test -z "$next"
        # The sibling command prints nothing but the found directory; the empty
        # output means it found none, even if the status was not propagated.
        test $code -eq 0; and set code 1
        __sibling_report $code
        return $code
    end
    cd $next; or return $status
    __sibling_position $file
end

# List the entries of the found sibling directory, without changing the
# working directory.
function __sibling_ls --argument-names type
    set -e argv[1]
    set -l parsed (__sibling_parse $argv)
    or return $status
    set -l count $parsed[1]
    set -l file ''
    if test (count $parsed) -gt 1
        set file $parsed[2]
    end
    set -l next (__sibling_find $type $count $file)
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
    set -e argv[1]
    set -l parsed (__sibling_parse $argv)
    or return $status
    set -l file ''
    if test (count $parsed) -gt 1
        set file $parsed[2]
    end
    set -l selected (sibling --format list --type keep -- (__sibling_target $file) | $filter)
    set -l code $status
    if test $code -ne 0 -o -z "$selected"
        return $code
    end
    # Each line of the list format consists of the index, the marker of the
    # current and the next directories, and the path; drop all but the path.
    set selected (string replace -r '^ *[0-9]+ (\* |> |  )' '' -- $selected)
    cd $selected; or return $status
    __sibling_position $file
end

# Print the found sibling directory, without changing the working directory.
# It tells the result by the exit status only, since the caller usually reads
# it by the command substitution, such as `cp file (nextdir)`.
function __sibling_print --argument-names type
    set -e argv[1]
    set -l parsed (__sibling_parse $argv)
    or return $status
    set -l count $parsed[1]
    set -l file ''
    if test (count $parsed) -gt 1
        set file $parsed[2]
    end
    set -l next (__sibling_find $type $count $file)
    set -l code $status
    if test -z "$next"
        test $code -eq 0; and set code 1
        return $code
    end
    echo $next
end

# Set NEXTDIR and PREVDIR to the siblings of the working directory; they become
# empty when no such directory is found.
function __sibling_hook
    set -l next (__sibling_find next 1 '' 2> /dev/null)
    set -l prev (__sibling_find previous 1 '' 2> /dev/null)
    set -gx NEXTDIR "$next"
    set -gx PREVDIR "$prev"
end

# Run the hook on every change of the working directory. It is not registered
# by default, since it runs the sibling command twice on every change; reading
# a directory of ten thousand entries costs about 30 milliseconds.
function sibling_hook_enable --description "Set NEXTDIR and PREVDIR on every chdir"
    function __sibling_hook_on_pwd --on-variable PWD
        __sibling_hook
    end
    __sibling_hook
end

function sibling_hook_disable --description "Stop setting NEXTDIR and PREVDIR"
    functions --erase __sibling_hook_on_pwd
    set -e NEXTDIR
    set -e PREVDIR
end

function nextdir --description "Print the next sibling directory"
    __sibling_print next $argv
end

function prevdir --description "Print the previous sibling directory"
    __sibling_print previous $argv
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
    __sibling_cd_with_filter peco $argv
end

function sibling_fzf --description "Choose a sibling directory with fzf"
    __sibling_cd_with_filter fzf $argv
end
