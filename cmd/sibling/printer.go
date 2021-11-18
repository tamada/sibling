package main

import (
	"github.com/tamada/sibling"
)

type printer interface {
	Print(i ...interface{})
	Println(i ...interface{})
	Printf(format string, i ...interface{})
}

type resulter interface {
	PrintHeader(header string)
	Print(sib *sibling.Siblings) (*sibling.Siblings, error)
}

type progressResulter struct {
	out printer
}

func (pp *progressResulter) PrintHeader(header string) {
	pp.out.Println(header)
}

func (pp *progressResulter) Print(sib *sibling.Siblings) (*sibling.Siblings, error) {
	pp.out.Printf("%3d/%3d\n", sib.CurrentIndex()+1, sib.TotalCount())
	return sib, nil
}

type nullResulter struct {
}

func (np *nullResulter) PrintHeader(header string) {
}

func (np *nullResulter) Print(sib *sibling.Siblings) (*sibling.Siblings, error) {
	return sib, nil
}

type parentResulter struct {
	out       printer
	formatter sibling.Formatter
}

func (pp *parentResulter) PrintHeader(header string) {
}

func (pp *parentResulter) Print(sib *sibling.Siblings) (*sibling.Siblings, error) {
	pp.out.Println(sib.Current().Parent())
	return sib, nil
}

type defaultResulter struct {
	out       printer
	formatter sibling.Formatter
	parent    resulter
	nexter    sibling.Nexter
}

func (dp *defaultResulter) PrintHeader(header string) {
	dp.out.Println(header)
}

func (dp *defaultResulter) Print(sib *sibling.Siblings) (*sibling.Siblings, error) {
	sib2 := dp.nexter.Next(sib)
	if sib2.Status == sibling.FINISH {
		dp.parent.Print(sib)
		return sib2, new(sibling.Finish)
	}
	current := sib2.Current()
	dp.out.Println(dp.formatter.Format(current))
	return sib2, nil
}

type listResulter struct {
	out       printer
	formatter sibling.Formatter
	nexter    sibling.Nexter
}

func (lp *listResulter) PrintHeader(header string) {
	lp.out.Println(header)
}

func (lp *listResulter) Print(sib *sibling.Siblings) (*sibling.Siblings, error) {
	sib2 := lp.nexter.Next(sib)
	for index, dir := range sib.SiblingDirs {
		mark := findMark(index, sib, sib2)
		lp.out.Printf("%s %s\n", mark, lp.formatter.Format(dir))
	}
	return sib, nil
}

func findMark(index int, sib, sib2 *sibling.Siblings) string {
	if index == sib.CurrentIndex() {
		if sib2.Status == sibling.FINISH {
			return "#"
		} else {
			return "*"
		}
	} else if index == sib2.CurrentIndex() {
		return ">"
	}
	return " "
}
