__sibling() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    case "${prev}" in
        --type | -t)
            COMPREPLY=($(compgen -W "next previous random last first" -- "${cur}"))
            return 0
            ;;
    esac
    opts="-a -h -l -P -p -q -s  -t --absolute --help --list --parent --progress --quiet --step --type"
    if [[ "$cur" =~ ^\- ]]; then
        COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
        return 0
    else
        compopt -o filenames
        COMPREPLY=($(compgen -d -- "$cur"))
    fi
}

complete -F __sibling -o bashdefault -o default sibling
