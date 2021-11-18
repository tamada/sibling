function __change_directory_to_sibling() {
    traversing_type="$1"
    if [[ "$1" == "" ]]; then
        traversing_type="next"
    fi
    next=$(sibling -t $traversing_type)
    sibling_status=$?
    if [[ $sibling_status -ne 0 ]] ; then
        echo "Done ($(sibling -p -t $traversing_type))"
        cd ..
    else
        cd $next
        echo "$PWD ($(sibling -p -t $traversing_type))"
    fi
    return $sibling_status
}

function __cd_sibling_filtering() {
    result="$(./sibling --list | $1)"
    if [[ $(echo $result | wc -l) -ne 1 ]]; then
        echo "Error: multiple paths are given"
        return 1
    fi
    cd ${result:2}
    pwd
}

sibling_peco() {
    __cd_sibling_filtering peco
}

sibling_fzf() {
    __cd_sibling_filtering fzf
}

cdnext(){
    __change_directory_to_sibling next
}

cdprev(){
    __change_directory_to_sibling previous
}

cdrand(){
    __change_directory_to_sibling random
}

alias nextdir="sibling -t next"
alias prevdir="sibling -t previous"
