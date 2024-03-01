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
	Csv      bool
}

func (p *Printer) printCsv(dirs *Dirs, noMoreFlag bool) error {
	fmt.Printf("%s,%s,%d,%d,%d,%v\n", dirs.CurrentPath(), dirs.NextPath(), dirs.Current+1, dirs.Next+1, len(dirs.Entries), noMoreFlag)
	return nil
}

func (p *Printer) Print(dirs *Dirs, noMoreFlag bool) error {
	if p.Csv {
		return p.printCsv(dirs, noMoreFlag)
	}
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
		fmt.Printf(" (%d/%d)", dirs.Next+1, len(dirs.Entries))
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
			fmt.Printf(" (%d/%d)", index+1, len(dirs.Entries))
		}
		fmt.Println()
	}
	return nil
}

func (p *Printer) printNoMoreDir(dirs *Dirs) error {
	if !p.Quiet {
		fmt.Printf("no more sibling directory\n")
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
