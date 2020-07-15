package sibling

import (
	"os"
	"path/filepath"
)

type Path struct {
	path string
}

func NewPath(path string) *Path {
	path = filepath.Clean(path)
	if path == "." {
		wd, err := os.Getwd()
		if err != nil {
			panic(err)
		}
		path = wd
	}
	return &Path{path: path}
}

func (path *Path) Abs() (*Path, error) {
	abs, err := filepath.Abs(path.path)
	if err != nil {
		return nil, err
	}
	return NewPath(abs), nil
}

func (path *Path) IsSame(other *Path) bool {
	return other != nil && path.Base() == other.Base()
}

func (path *Path) Base() string {
	return filepath.Base(path.path)
}

func (path *Path) ParentPath() *Path {
	return NewPath(path.Parent())
}

func (path *Path) Parent() string {
	return filepath.Clean(filepath.Join(path.path, ".."))
}

func (path *Path) String() string {
	return path.path
}
