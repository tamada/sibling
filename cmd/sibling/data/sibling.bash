__change_directory_to_sibling() {
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
    next=$(sibling --absolute --type $traversing_type)
    sibling_status=$?
    if [[ $sibling_status -ne 0 ]]; then
        echo "no more siblings"
    else
        echo $next
        ls $next
    fi
}

lsnext() {
    __ls_sibling next
}

lsprev() {
    __ls_sibling previous
}

lsrand() {
    __ls_sibling rand
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
    __change_directory_to_sibling next
}

cdprev() {
    __change_directory_to_sibling previous
}

cdrand() {
    __change_directory_to_sibling random
}

