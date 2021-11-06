package main

import (
	"os"
	"path/filepath"
	"runtime"
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
	//     -p, --progress      print the progress traversing directories
	//     -t, --type <TYPE>   specify the traversing type of siblings (default: next, available: next, previous, random)
	//     -P, --parent        print parent directory, when no more sibling directories (available on no-console mode)
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
	//     if [ "$1" == "" ]; then
	//         traversing_type="next"
	//     fi
	//     next=$(sibling -t $traversing_type)
	//     status=$?
	//     if [ $status -ne 0 ] ; then
	//         echo "Done ($(sibling -p -t $traversing_type))"
	//         cd ..
	//     else
	//         cd $next
	//         echo "$PWD ($(sibling -p -t $traversing_type))"
	//     fi
	//     return $status
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

func init() {
	_, filename, _, _ := runtime.Caller(0)
	dir := filepath.Dir(filename)
	err := os.Chdir(dir)
	if err != nil {
		panic(err)
	}
}
