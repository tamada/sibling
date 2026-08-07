# The shell functions for traversing the sibling directories.
# Install them by putting the following line into your shell profile,
# such as .bash_profile and .zshrc.
#
#     eval "$(sibling --init bash)"
#
# Each function receives the optional count of the traversing, such as
# "cdnext 3". A negative count traverses in the opposite direction.
# The count is ignored by cdfirst, cdlast, cdrand, lsfirst, lslast, and lsrand.
#
# They also receive "-f FILE", which traverses the directories listed in the
# file, instead of the siblings of the working directory, such as
# "cdnext -f ~/projects.txt". Give the file in an absolute path, since the
# working directory changes.

# __sibling_parse [COUNT] [-f FILE]
# Set the count and the file from the given arguments; the caller declares them
# as its local variables, and the empty file means the working directory.
__sibling_parse() {
    count=1
    file=
    while [ $# -gt 0 ]; do
        case "$1" in
            -f | --file)
                if [ -z "${2:-}" ]; then
                    echo "sibling: $1: no file is given" >&2
                    return 2
                fi
                file=$2
                shift 2
                ;;
            *)
                count=$1
                shift
                ;;
        esac
    done
}

# __sibling_find <TYPE> <COUNT> [FILE]
# Print the found sibling directory of the working directory, or of the given
# list file. The exit status is the one of the sibling command; 0 means the
# directory was found, 1 means no more sibling directory, and the others mean
# an error.
__sibling_find() {
    sibling --type "$1" --step "$2" -- "${3:-$PWD}"
}

# __sibling_position [FILE]
# Print the working directory with its position, such as "/path/to/c (3/26)".
__sibling_position() {
    sibling --progress --type keep -- "${1:-$PWD}"
}

# __sibling_report <CODE>
# Tell the user why no directory was found, and return the given status.
__sibling_report() {
    if [ "$1" -eq 1 ]; then
        echo "sibling: no more sibling directory" >&2
    fi
    return "$1"
}

# __sibling_cd <TYPE> [COUNT]
# Change the working directory to the found sibling directory.
__sibling_cd() {
    local type=$1 count file next code
    shift
    __sibling_parse "$@" || return $?
    next=$(__sibling_find "$type" "$count" "$file")
    code=$?
    if [ $code -ne 0 ]; then
        __sibling_report $code
        return $code
    fi
    cd -- "$next" || return $?
    __sibling_position "$file"
}

# __sibling_ls <TYPE> [COUNT]
# List the entries of the found sibling directory, without changing the
# working directory.
__sibling_ls() {
    local type=$1 count file next code
    shift
    __sibling_parse "$@" || return $?
    next=$(__sibling_find "$type" "$count" "$file")
    code=$?
    if [ $code -ne 0 ]; then
        __sibling_report $code
        return $code
    fi
    echo "$next"
    ls -- "$next"
}

# __sibling_cd_with_filter <FILTER>
# Choose a sibling directory with the filter command, such as peco and fzf,
# and change the working directory to it.
__sibling_cd_with_filter() {
    local filter=$1 count file selected code
    shift
    __sibling_parse "$@" || return $?
    selected=$(sibling --format list --type keep -- "${file:-$PWD}" | "$filter")
    code=$?
    if [ $code -ne 0 ] || [ -z "$selected" ]; then
        return $code
    fi
    # Each line of the list format consists of the index, the marker of the
    # current and the next directories, and the path; drop all but the path.
    selected=$(printf '%s\n' "$selected" | sed -E 's/^ *[0-9]+ (\* |> |  )//')
    cd -- "$selected" || return $?
    __sibling_position "$file"
}

# __sibling_print <TYPE> [COUNT] [-f FILE]
# Print the found sibling directory, without changing the working directory.
# It tells the result by the exit status only, since the caller usually reads
# it by the command substitution, such as `cp file "$(nextdir)"`.
__sibling_print() {
    local type=$1 count file next code
    shift
    __sibling_parse "$@" || return $?
    next=$(__sibling_find "$type" "$count" "$file")
    code=$?
    if [ $code -ne 0 ]; then
        return $code
    fi
    echo "$next"
}

# __sibling_hook
# Set NEXTDIR and PREVDIR to the siblings of the working directory; they become
# empty when no such directory is found.
__sibling_hook() {
    NEXTDIR=$(__sibling_find next 1 "" 2> /dev/null) || NEXTDIR=
    PREVDIR=$(__sibling_find previous 1 "" 2> /dev/null) || PREVDIR=
    export NEXTDIR PREVDIR
}

# Run the hook on every change of the working directory. It is not registered
# by default, since it runs the sibling command twice on every change; reading
# a directory of ten thousand entries costs about 30 milliseconds.
sibling_hook_enable() {
    if [ -n "${ZSH_VERSION:-}" ]; then
        typeset -ga chpwd_functions
        case " ${chpwd_functions[*]} " in
            *" __sibling_hook "*) ;;
            *) chpwd_functions+=(__sibling_hook) ;;
        esac
    else
        case "${PROMPT_COMMAND:-}" in
            *__sibling_hook*) ;;
            *) PROMPT_COMMAND="__sibling_hook${PROMPT_COMMAND:+;$PROMPT_COMMAND}" ;;
        esac
    fi
    __sibling_hook
}

sibling_hook_disable() {
    local kept f
    if [ -n "${ZSH_VERSION:-}" ]; then
        kept=()
        for f in "${chpwd_functions[@]}"; do
            [ "$f" = "__sibling_hook" ] || kept+=("$f")
        done
        chpwd_functions=("${kept[@]}")
    else
        PROMPT_COMMAND=$(printf '%s' "${PROMPT_COMMAND:-}" | sed -e 's/__sibling_hook;\{0,1\}//')
    fi
    unset NEXTDIR PREVDIR
}

nextdir() {
    __sibling_print next "$@"
}

prevdir() {
    __sibling_print previous "$@"
}

cdnext() {
    __sibling_cd next "$@"
}

cdprev() {
    __sibling_cd previous "$@"
}

cdfirst() {
    __sibling_cd first "$@"
}

cdlast() {
    __sibling_cd last "$@"
}

cdrand() {
    __sibling_cd random "$@"
}

lsnext() {
    __sibling_ls next "$@"
}

lsprev() {
    __sibling_ls previous "$@"
}

lsfirst() {
    __sibling_ls first "$@"
}

lslast() {
    __sibling_ls last "$@"
}

lsrand() {
    __sibling_ls random "$@"
}

sibling_peco() {
    __sibling_cd_with_filter peco "$@"
}

sibling_fzf() {
    __sibling_cd_with_filter fzf "$@"
}
