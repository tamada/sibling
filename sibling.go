package sibling

import (
	"fmt"
	"io/ioutil"
	"path/filepath"
	"sort"
)

type Status int

const (
	TRAVERSING Status = iota + 1
	FINISH
	INVALID
)

type Siblings struct {
	current  int
	siblings []*Path
	Status   Status
}

func (sib *Siblings) Current() *Path {
	if sib.current < 0 {
		return nil
	}
	return sib.siblings[sib.current]
}

func (sib *Siblings) CurrentIndex() int {
	return sib.current
}

func (sib *Siblings) TotalCount() int {
	return len(sib.siblings)
}

func findCurrentIndex(path *Path, paths []*Path) int {
	for i, p := range paths {
		if p.IsSame(path) {
			return i
		}
	}
	return -1
}

func NewSiblings(path *Path) (*Siblings, error) {
	paths, err := sibling(path)
	if err != nil {
		return nil, err
	}
	index := findCurrentIndex(path, paths)
	if index < 0 {
		return nil, &NotFound{path: path}
	}
	return &Siblings{current: index, siblings: paths, Status: TRAVERSING}, nil
}

type Finish struct {
}

func (f *Finish) Error() string {
	return "done"
}

type NotFound struct {
	path *Path
}

func (nf *NotFound) Error() string {
	return fmt.Sprintf("%s: not found", nf.path.String())
}

func sibling(path *Path) ([]*Path, error) {
	parent := path.Parent()
	infos, err := ioutil.ReadDir(parent)
	if err != nil {
		return nil, err
	}
	results := []*Path{}
	for _, info := range infos {
		if info.IsDir() {
			results = append(results, NewPath(filepath.Join(parent, info.Name())))
		}
	}
	sort.Slice(results, func(i, j int) bool { return results[i].path < results[j].path })
	return results, nil
}
