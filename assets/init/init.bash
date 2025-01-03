__change_directory_to_sibling() {
    traversing_type="$1"
    if [[ "$traversing_type" == "" ]]; then
        traversing_type="next"
    fi
    step=1
    if [[ $# -eq 2 ]]; then
        step=$2
    fi
    next=$(sibling --type $traversing_type --csv --step $step)
    sibling_status=$?
    result=($(echo $next | tr -s ',' ' '))
    if [[ $sibling_status -eq 0 ]] ; then
        cd "$(echo ${result[2]} | xargs)"
        echo "$PWD (${result[4]}/${result[5]})"
    else
        echo "Done (${result[4]}/${result[5]})"
        cd ..
    fi
    return $sibling_status
}

__cd_sibling_filtering() {
    result="$(./sibling --list | $1)"
    if [[ $(echo $result | wc -l) -ne 1 ]]; then
        echo "Error: multiple paths are given"
        return 1
    fi
    cd ${result:2}
    pwd
}

__ls_sibling() {
    traversing_type="$1"
    if [[ "$1" == "" ]]; then
        traversing_type="next"
    fi
    step=1
    if [[ $# -eq 2 ]]; then
        step=$2
    fi
    next=$(sibling --absolute --type $traversing_type --csv --step $step)
    sibling_status=$?
    result=($(echo $next | tr -s ',' ' '))
    if [[ $sibling_status -eq 0 ]]; then
        r=$(echo ${result[2]} | xargs)
        echo "$r (${result[4]}/${result[5]})"
        ls "$r"
    else
        echo "no more siblings"
    fi
}

lsnext() {
    __ls_sibling next $@
}

lsprev() {
    __ls_sibling previous $@
}

lsrand() {
    __ls_sibling random
}

lsfirst() {
    __ls_sibling first
}

lslast() {
    __ls_sibling last
}

sibling_peco() {
    __cd_sibling_filtering peco
}

sibling_fzf() {
    __cd_sibling_filtering fzf
}

cdfirst() {
    __change_directory_to_sibling first
}

cdlast() {
    __change_directory_to_sibling last
}

cdnext() {
    __change_directory_to_sibling next $@
}

cdprev() {
    __change_directory_to_sibling previous $@
}

cdrand() {
    __change_directory_to_sibling random
}

