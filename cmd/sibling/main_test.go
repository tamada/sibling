package main

import (
	"os"
)

func Example_Help() {
	cmd := newCommand()
	cmd.SetArgs([]string{"--help"})
	cmd.SetOut(os.Stdout)
	cmd.Execute()
	// Output:
	// get next/previous sibling directory name
	//
	// Usage: sibling [FLAGs] [DIRs...]
	// FLAGS
	//     -a, --absolute      print the directory name in the absolute path
	//     -l, --list          list the sibling directories
	//     -p, --progress      print the progress traversing directories
	//     -P, --parent        print parent directory, when no more sibling directories (available on no-console mode)
	//     -t, --type <TYPE>   specify the traversing type of siblings (default: next, available: next, previous, random)
	//
	//     -h, --help          print this message
	//     -v, --version       print version
	// ARGUMENTS
	//     DIR                 specify the directory. If not specified, the current directory is used
}

func Example() {
	cmd := newCommand()
	cmd.SetArgs([]string{"../../testdata/3", "../../testdata/1", "../../testdata/9"})
	cmd.SetOut(os.Stdout)
	cmd.Execute()
	// Output:
	// ===== ../../testdata/3 =====
	// ../../testdata/4
	// ===== ../../testdata/1 =====
	// ../../testdata/2
	// ===== ../../testdata/9 =====
}

func Example_ShellFunctionGenerator() {
	cmd := newCommand()
	cmd.SetArgs([]string{"--init", "bash"})
	cmd.Execute()
	// Output:
	// function __change_directory_to_sibling() {
	//     traversing_type="$1"
	//     if [[ "$1" == "" ]]; then
	//         traversing_type="next"
	//     fi
	//     next=$(sibling -t $traversing_type)
	//     sibling_status=$?
	//     if [[ $sibling_status -ne 0 ]] ; then
	//         echo "Done ($(sibling -p -t $traversing_type))"
	//         cd ..
	//     else
	//         cd $next
	//         echo "$PWD ($(sibling -p -t $traversing_type))"
	//     fi
	//     return $sibling_status
	// }
	//
	// function __cd_sibling_filtering() {
	//     result="$(./sibling --list | $1)"
	//     if [[ $(echo $result | wc -l) -ne 1 ]]; then
	//         echo "Error: multiple paths are given"
	//         return 1
	//     fi
	//     cd ${result:2}
	//     pwd
	// }
	//
	// sibling_peco() {
	//     __cd_sibling_filtering peco
	// }
	//
	// sibling_fzf() {
	//     __cd_sibling_filtering fzf
	// }
	//
	// cdnext(){
	//     __change_directory_to_sibling next
	// }
	//
	// cdprev(){
	//     __change_directory_to_sibling previous
	// }
	//
	// cdrand(){
	//     __change_directory_to_sibling random
	// }
	//
	// alias nextdir="sibling -t next"
	// alias prevdir="sibling -t previous"
}

func Example_parent() {
	cmd := newCommand()
	cmd.SetArgs([]string{"--parent", "../../testdata/3", "../../testdata/9"})
	cmd.SetOut(os.Stdout)
	cmd.Execute()
	// Output:
	// ===== ../../testdata/3 =====
	// ../../testdata/4
	// ===== ../../testdata/9 =====
	// ../../testdata
}

func Example_progress() {
	cmd := newCommand()
	cmd.SetArgs([]string{"../../testdata/4", "--progress"})
	cmd.SetOut(os.Stdout)
	cmd.Execute()
	// Output:
	//  5/ 10
}
