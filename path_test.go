package sibling

import "testing"

func TestPath(t *testing.T) {
	testdata := []struct {
		giveString string
		wontBase   string
		wontString string
		wontParent string
	}{
		{"testdata", "testdata", "testdata", "."},
	}
	for _, td := range testdata {
		path := NewPath(td.giveString)
		if path.Base() != td.wontBase {
			t.Errorf("Base did not match, wont %s, got %s", td.wontBase, path.Base())
		}
		if path.String() != td.wontString {
			t.Errorf("String did not match, wont %s, got %s", td.wontString, path.String())
		}
		if path.Parent() != td.wontParent {
			t.Errorf("Parent did not match, wont %s, got %s", td.wontParent, path.Parent())
		}
	}
}

func TestIsSame(t *testing.T) {
	testdata := []struct {
		giveString1  string
		giveString2  string
		wontSameFlag bool
	}{
		{"./sibling", "sibling", true},
	}
	for _, td := range testdata {
		path1 := NewPath(td.giveString1)
		path2 := NewPath(td.giveString2)
		if path1.IsSame(path2) != td.wontSameFlag {
			t.Errorf(`"%s".IsSame("%s") wont %v, but %v`, td.giveString1, td.giveString2, td.wontSameFlag, !td.wontSameFlag)
		}
	}
}
