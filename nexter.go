package sibling

import (
	"fmt"
	"math/rand"
	"strconv"
	"strings"
	"time"
)

type Nexter interface {
	// Next calculates the next directory of dirs.
	// If no more next directory, this function returns true.
	Next(dirs *Dirs) bool
}

type NexterType int

const (
	First NexterType = iota + 1
	Last
	Next
	Previous
	Random
	Current
)

func (kind NexterType) String() string {
	switch kind {
	case First:
		return "First"
	case Last:
		return "Last"
	case Next:
		return "Next"
	case Previous:
		return "Previous"
	case Current:
		return "Current"
	case Random:
		return "Random"
	default:
		return strconv.Itoa(int(kind))
	}
}

func FindNexterType(kind string) (NexterType, error) {
	smallKind := strings.ToLower(kind)
	switch smallKind {
	case "first":
		return First, nil
	case "last":
		return Last, nil
	case "next":
		return Next, nil
	case "previous", "prev":
		return Previous, nil
	case "current":
		return Current, nil
	case "random", "rand":
		return Random, nil
	}
	return -1, fmt.Errorf("%s: unknown NexterType", kind)
}

func NewNexter(kind NexterType, step int) (Nexter, error) {
	switch kind {
	case First:
		return &first{}, nil
	case Last:
		return &last{}, nil
	case Next:
		return &next{step: step}, nil
	case Previous:
		return &prev{step: step}, nil
	case Current:
		return &current{}, nil
	case Random:
		return &random{r: rand.New(rand.NewSource(time.Now().UnixNano()))}, nil
	default:
		return nil, fmt.Errorf("%s: unknown NexterT", kind.String())
	}
}

type first struct {
}
type last struct {
}
type next struct {
	step int
}
type prev struct {
	step int
}
type current struct {
}
type random struct {
	r *rand.Rand
}

func (n *first) Next(dirs *Dirs) bool {
	dirs.Next = 0
	return false
}

func (n *last) Next(dirs *Dirs) bool {
	dirs.Next = len(dirs.Entries) - 1
	return false
}

func (n *current) Next(dirs *Dirs) bool {
	dirs.Next = dirs.Current
	return false
}

func (n *random) Next(dirs *Dirs) bool {
	dirs.Next = n.r.Intn(len(dirs.Entries))
	return false
}

func (n *prev) Next(dirs *Dirs) bool {
	return nextImpl(dirs, n.step*-1)
}

func (n *next) Next(dirs *Dirs) bool {
	return nextImpl(dirs, n.step)
}

func nextImpl(dirs *Dirs, step int) bool {
	dirs.Next = dirs.Current + step
	if dirs.Next >= len(dirs.Entries) {
		dirs.Next = len(dirs.Entries) - 1
		return true
	}
	if dirs.Next < 0 {
		dirs.Next = 0
		return true
	}
	return false
}
