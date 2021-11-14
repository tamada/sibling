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

cdnext(){
    __change_directory_to_sibling next
}

cdprev(){
    __change_directory_to_sibling previous
}

cdrand(){
    __change_directory_to_sibling random
}