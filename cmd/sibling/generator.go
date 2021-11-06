package main

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/spf13/cobra"
)

//go:embed data/sibling.bash
var bashFunctions []byte

func printGenerator(shellName string, c *cobra.Command) error {
	switch strings.ToLower(shellName) {
	case "bash":
		bashFunctionGenerator(c)
	default:
		return fmt.Errorf("sorry, %s is unsupported shell", shellName)
	}
	return nil
}

func bashFunctionGenerator(c *cobra.Command) error {
	c.Println(string(bashFunctions))
	return nil
}
