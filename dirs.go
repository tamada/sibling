package sibling

import (
	"fmt"
	"os"
	"path/filepath"
)

type Dirs struct {
	Entries   []string
	Parent    string
	Current   int
	Next      int
	NoMoreDir bool
}

func (d *Dirs) CurrentName() string {
	return d.Entries[d.Current]
}

func (d *Dirs) CurrentPath() string {
	return filepath.Join(d.Parent, d.CurrentName())
}

func (d *Dirs) NextName() string {
	return d.Entries[d.Next]
}

func (d *Dirs) NextPath() string {
	return filepath.Join(d.Parent, d.NextName())
}

func NewDirs(path string) (*Dirs, error) {
	if path == "." {
		p, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		current := filepath.Base(p)
		return newDirs("..", current)
	}
	if err := existDir(path); err != nil {
		return nil, err
	}
	return newDirs(filepath.Dir(path), filepath.Base(path))
}

func newDirs(path, currentDir string) (*Dirs, error) {
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}
	d := &Dirs{Parent: path}
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			d.Entries = append(d.Entries, name)
		}
		if name == currentDir {
			d.Current = len(d.Entries) - 1
		}
	}
	return d, nil
}

func existDir(path string) error {
	stat, err := os.Stat(path)
	if err != nil {
		return err
	}
	if stat.IsDir() {
		return nil
	}
	return fmt.Errorf("%s: not directory", path)
}
