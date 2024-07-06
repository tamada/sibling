package main

import (
	"os"

	"github.com/spf13/cobra"
	"github.com/tamada/sibling/v2/cmd"
)

func newCommand() *cobra.Command {
	command := &cobra.Command{
		Use:     "sibling",
		Short:   "get next/previous sibling directory name",
		Version: cmd.VERSION,
		RunE:    perform,
	}
	flags := command.Flags()
	cmd.BuildFlags(flags)
	return command
}

func setup() {
	os.MkdirAll("completions/bash", 0755)
	os.MkdirAll("completions/zsh", 0755)
	os.MkdirAll("completions/fish", 0755)
	os.MkdirAll("completions/ps", 0755)
}

func perform(cmd *cobra.Command, args []string) error {
	setup()
	cmd.GenBashCompletionFileV2("completions/bash/sibling.bash", true)
	cmd.GenZshCompletionFile("completions/zsh/_sibling")
	cmd.GenFishCompletionFile("completions/fish/sibling.fish", true)
	cmd.GenPowerShellCompletionFileWithDesc("completions/ps/sibling.ps1")
	return nil
}

func main() {
	err := newCommand().Execute()
	if err != nil {
		os.Exit(1)
	}
	os.Exit(0)
}
