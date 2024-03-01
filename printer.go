package sibling

import (
	"fmt"
	"path/filepath"
)

type Printer struct {
	Absolute bool
	List     bool
	Progress bool
	Parent   bool
	Quiet    bool
}

func (p *Printer) Print(dirs *Dirs, noMoreFlag bool) error {
	if noMoreFlag {
		return p.printNoMoreDir(dirs)
	}
	if p.List {
		return p.printList(dirs)
	}
	return p.printImpl(dirs)
}

func (p *Printer) printImpl(dirs *Dirs) error {
	fmt.Print(p.path(dirs.NextPath()))
	if p.Progress {
		fmt.Printf(" (%d/%d)", dirs.Next, len(dirs.Entries))
	}
	fmt.Println()
	return nil
}

func (p *Printer) printList(dirs *Dirs) error {
	for index, entry := range dirs.Entries {
		if index == dirs.Next {
			fmt.Print("> ")
		} else if index == dirs.Current {
			fmt.Print("* ")
		} else {
			fmt.Print("  ")
		}
		fmt.Print(p.path(filepath.Join(dirs.Parent, entry)))
		if p.Progress {
			fmt.Printf(" (%d/%d)", index, len(dirs.Entries))
		}
		fmt.Println()
	}
	return nil
}

func (p *Printer) printNoMoreDir(dirs *Dirs) error {
	if !p.Quiet {
		fmt.Printf("%s: no more sibling directory\n", p.path(dirs.CurrentName()))
	}
	if p.Parent {
		fmt.Println(p.path(dirs.Parent))
	}
	return nil
}

func (p *Printer) path(path string) string {
	if p.Absolute {
		r, err := filepath.Abs(path)
		if err != nil {
			panic(err)
		}
		return r
	}
	return path
}
