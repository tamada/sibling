__sibling() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    case "${prev}" in
        --type | -t)
            COMPREPLY=($(compgen -W "next previous random" -- "${cur}"))
            return 0
            ;;
    esac
    opts=" -a -p -P -h -V -t  --absolute --progress --parent --help --version --type"
    if [[ "$cur" =~ ^\- ]]; then
        COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
        return 0
    else
        compopt -o filenames
        COMPREPLY=($(compgen -d -- "$cur"))
    fi
}

complete -F __sibling -o bashdefault -o default sibling
