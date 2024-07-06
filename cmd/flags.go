package cmd

import "github.com/spf13/pflag"

const VERSION string = "2.0.0-beta-1"

func BuildFlags(flags *pflag.FlagSet) error {
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
	return nil
}
