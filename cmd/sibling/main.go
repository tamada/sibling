package main

import (
	"embed"
	"errors"
	"fmt"
	"os"
	"path/filepath"

	flag "github.com/spf13/pflag"
	"github.com/tamada/sibling/v2"
)

const VERSION string = "2.0.0-beta-1"

func helpMessage(flags *flag.FlagSet, progName string) string {
	return fmt.Sprintf(`%s version %s
%s [OPTIONS] [DIRS...]
OPTIONS
%sDIRS
    the target directory of sibling`, filepath.Base(progName), VERSION, filepath.Base(progName), flags.FlagUsages())
}

func parseFlags(args []string) (*flag.FlagSet, error) {
	flags := flag.NewFlagSet("sibling", flag.ContinueOnError)
	flags.Usage = func() { fmt.Println(helpMessage(flags, args[0])) }
	flags.StringP("type", "t", "next", "specify nexter type (available: next, previous, first, last and random)")
	flags.IntP("step", "s", 1, "specify the number of times to execute sibling.")
	flags.BoolP("absolute", "a", false, "print the directory name in the absolute path")
	flags.BoolP("list", "l", false, "list the sibling directories")
	flags.BoolP("progress", "p", false, "print the progress traversing directories")
	flags.BoolP("parent", "P", false, "print parent directory, when no more sibling directories")
	flags.BoolP("quiet", "q", false, "quiet mode")
	flags.BoolP("help", "h", false, "print this message")
	flags.StringP("init", "", "", "generate shell initializer")
	flags.BoolP("csv", "", false, "print the result in csv format")
	flags.MarkHidden("csv")
	flags.MarkHidden("init")
	if err := flags.Parse(args); err != nil {
		return nil, err
	}
	if flag, _ := flags.GetBool("help"); flag {
		flags.Usage()
	}
	return flags, nil
}

func getBool(flags *flag.FlagSet, name string) bool {
	value, err := flags.GetBool(name)
	if err != nil {
		return false
	}
	return value
}

func constructPrinter(flags *flag.FlagSet) *sibling.Printer {
	p := &sibling.Printer{}
	p.Absolute = getBool(flags, "absolute")
	p.Parent = getBool(flags, "parent")
	p.Progress = getBool(flags, "progress")
	p.List = getBool(flags, "list")
	p.Quiet = getBool(flags, "quiet")
	p.Csv = getBool(flags, "csv")
	return p
}

func constructNexter(flags *flag.FlagSet) (sibling.Nexter, error) {
	kind, err := flags.GetString("type")
	if err != nil {
		return nil, err
	}
	nexterType, err := sibling.FindNexterType(kind)
	if err != nil {
		return nil, err
	}
	step, err := flags.GetInt("step")
	if err != nil {
		return nil, err
	}
	return sibling.NewNexter(nexterType, step)
}

func performEach(arg string, nexter sibling.Nexter) (*sibling.Dirs, bool, error) {
	dirs, err := sibling.NewDirs(arg)
	if err != nil {
		return nil, true, err
	}
	return dirs, nexter.Next(dirs), nil
}

func perform(nexter sibling.Nexter, printer *sibling.Printer, args []string) int {
	var errs []error
	returnFlag := 0
	for _, arg := range args {
		dirs, noMoreFlag, err := performEach(arg, nexter)
		errs = appendErrors(errs, err)
		printer.Print(dirs, noMoreFlag)
		if noMoreFlag {
			returnFlag = 1
		}
	}
	if len(errs) > 0 {
		return printError(errors.Join(errs...), -3)
	}
	return returnFlag
}

func appendErrors(errs []error, err error) []error {
	if err != nil {
		return append(errs, err)
	}
	return errs
}

func constructArgs(args []string) []string {
	if len(args) == 0 {
		return []string{"."}
	}
	return args
}

//go:embed data
var fs embed.FS

func initSiblingImpl(shellName string) error {
	data, err := fs.ReadFile("data/init." + shellName)
	if err != nil {
		return err
	}
	fmt.Print(string(data))
	return nil
}

func initSibling(shellName string) int {
	if err := initSiblingImpl(shellName); err != nil {
		fmt.Println(err.Error())
		return -5
	}
	return 0
}

func goMain(args []string) int {
	flags, err := parseFlags(args)
	if err != nil {
		return printError(err, -1)
	}
	if flag, _ := flags.GetBool("help"); flag {
		return 0
	}
	if value, err := flags.GetString("init"); err == nil && value != "" {
		return initSibling(value)
	}
	nexter, err := constructNexter(flags)
	if err != nil {
		return printError(err, -2)
	}
	printer := constructPrinter(flags)
	return perform(nexter, printer, constructArgs(flags.Args()[1:]))
}

func printError(err error, statusCode int) int {
	if err != nil {
		fmt.Println(err.Error())
		return statusCode
	}
	return 0
}

func main() {
	status := goMain(os.Args)
	os.Exit(status)
}
