package main

import (
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/spf13/cobra"
	"github.com/tamada/sibling"
)

const VERSION = "1.1.3"

type options struct {
	absolute bool
	progress bool
	list     bool
	kind     string
	parent   bool
}

var shellInitializer string

var opts = &options{}

func usage(c *cobra.Command) error {
	c.SilenceUsage = false
	c.Printf(`%sUsage: %s [FLAGs] [DIRs...]
FLAGS
    -a, --absolute      print the directory name in the absolute path
    -l, --list          list the sibling directories
    -p, --progress      print the progress traversing directories
    -P, --parent        print parent directory, when no more sibling directories (available on no-console mode)
    -t, --type <TYPE>   specify the traversing type of siblings (default: next, available: next, previous, random)

    -h, --help          print this message
    -v, --version       print version
ARGUMENTS
    DIR                 specify the directory. If not specified, the current directory is used
`, c.Long, c.Use)

	return nil
}

func newCommand() *cobra.Command {
	cmd := &cobra.Command{
		Use:     "sibling",
		Version: VERSION,
		Args:    validateArgs,
		Short:   "get next/previous sibling directory name",
		RunE:    perform,
	}
	cmd.SetUsageFunc(usage)
	flags := cmd.Flags()
	flags.BoolVarP(&opts.absolute, "absolute", "a", false, "print the directory name in the absolute path")
	flags.BoolVarP(&opts.progress, "progress", "p", false, "print the progress of the traversing directories")
	flags.BoolVarP(&opts.parent, "parent", "P", false, "print parent directory, when no more sibling directories (available on no-console mode)")
	flags.BoolVarP(&opts.list, "list", "l", false, "list the sibling directories")
	flags.StringVarP(&opts.kind, "type", "t", "next", "specify the traversing type of siblings. (default: next, available: next, previous, and random)")
	flags.StringVarP(&shellInitializer, "init", "", "", "generate shell functions")
	cmd.SetOut(os.Stdout)

	return cmd
}

func (opts *options) formatter() sibling.Formatter {
	return sibling.NewFormatter(opts.absolute)
}

func (opts *options) nexter() sibling.Nexter {
	traversingType := sibling.NexterType(opts.kind)
	return sibling.NewNexter(traversingType)
}

func (opts *options) Parent() bool {
	return opts.parent
}

func (opts *options) parentPrinter(c *cobra.Command, formatter sibling.Formatter) resulter {
	if opts.parent {
		return &parentPrinter{out: c, formatter: formatter}
	}
	return &nullPrinter{}
}

func (opts *options) buildParams(c *cobra.Command) *params {
	params := &params{}
	if opts.progress {
		params.printer = &progressPrinter{out: c}
	} else if opts.list {
		params.printer = &listPrinter{out: c, formatter: opts.formatter(), nexter: opts.nexter()}
	} else {
		formatter := opts.formatter()
		params.printer = &defaultPrinter{out: c, formatter: formatter, nexter: opts.nexter(), parent: opts.parentPrinter(c, formatter)}
	}
	return params
}

func validateArgs(c *cobra.Command, args []string) error {
	if err := validateKind(opts.kind); err != nil {
		return err
	}
	return nil
}

func validateKind(kind string) error {
	switch strings.ToLower(kind) {
	case "next", "previous", "random":
		return nil
	default:
		return fmt.Errorf("%s: invalid type", kind)
	}
}

func performEach(arg string, opts *options, params *params) (*sibling.Siblings, error) {
	path := sibling.NewPath(arg)
	sib, err := sibling.NewSiblings(path)
	if err != nil {
		return sib, err
	}
	return params.printer.Print(sib)
}

func perform(c *cobra.Command, args []string) error {
	if shellInitializer != "" {
		return printGenerator(shellInitializer, c)
	}
	params := opts.buildParams(c)
	c.SilenceUsage = true
	for _, arg := range constructArgs(args) {
		if len(args) > 1 {
			params.printer.PrintHeader(fmt.Sprintf("===== %s =====", arg))
		}
		siblings, err := performEach(arg, opts, params)
		if err != nil && !errors.Is(err, &sibling.Finish{}) {
			return err
		}
		if siblings.Status == sibling.FINISH {
			c.SilenceErrors = true
			return errors.New("no more siblings")
		}
	}
	return nil
}

func constructArgs(args []string) []string {
	if len(args) == 0 {
		return []string{"."}
	}
	return args
}

func main() {
	err := newCommand().Execute()
	if err != nil {
		os.Exit(1)
	}
	os.Exit(0)
}
