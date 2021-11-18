package sibling

import (
	"math/rand"
	"time"
)

type Nexter interface {
	Next(siblings *Siblings) *Siblings
	RestCount(siblings *Siblings) int
}

type NexterType string

const (
	NEXT     NexterType = "next"
	PREVIOUS NexterType = "previous"
	RANDOM   NexterType = "random"
)

var AVAILABLE_TRAVERSING_TYPE = []string{string(NEXT), string(PREVIOUS), string(RANDOM)}

func ValidateNexterType(typeString string) bool {
	for _, att := range AVAILABLE_TRAVERSING_TYPE {
		if att == typeString {
			return true
		}
	}
	return false
}

func NewNexter(kind NexterType) Nexter {
	switch kind {
	case NEXT:
		return new(Next)
	case PREVIOUS:
		return new(Previous)
	case RANDOM:
		return new(Random)
	}
	return nil
}

type Random struct {
}

type Next struct {
}

type Previous struct {
}

func (random *Random) RestCount(siblings *Siblings) int {
	if siblings.Status == FINISH {
		return 0
	}
	return len(siblings.SiblingDirs)
}

func (random *Random) Next(siblings *Siblings) *Siblings {
	rand.Seed(time.Now().Unix())
	newCurrent := rand.Int() % siblings.TotalCount()
	return &Siblings{current: newCurrent, SiblingDirs: siblings.SiblingDirs, Status: TRAVERSING}
}

func (next *Next) RestCount(siblings *Siblings) int {
	if siblings.Status == FINISH {
		return 0
	}
	return len(siblings.SiblingDirs) - siblings.current - 1
}

func (next *Next) Next(siblings *Siblings) *Siblings {
	if (siblings.current + 1) == len(siblings.SiblingDirs) {
		return &Siblings{current: -1, SiblingDirs: siblings.SiblingDirs, Status: FINISH}
	}
	return &Siblings{current: siblings.current + 1, SiblingDirs: siblings.SiblingDirs, Status: TRAVERSING}
}

func (prev *Previous) RestCount(siblings *Siblings) int {
	if siblings.Status == FINISH {
		return 0
	}
	return siblings.current + 1
}

func (prev *Previous) Next(siblings *Siblings) *Siblings {
	if siblings.current == 0 {
		return &Siblings{current: -1, SiblingDirs: siblings.SiblingDirs, Status: FINISH}
	}
	return &Siblings{current: siblings.current - 1, SiblingDirs: siblings.SiblingDirs, Status: TRAVERSING}
}
