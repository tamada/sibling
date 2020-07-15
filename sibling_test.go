package sibling

import "testing"

func TestSibling(t *testing.T) {
	testdata := []struct {
		path    string
		current int
		status  Status
	}{
		{"testdata/0", 0, TRAVERSING},
		{"testdata/9", 9, TRAVERSING},
	}

	for _, td := range testdata {
		sib, _ := NewSiblings(NewPath(td.path))
		if sib.Status != td.status {
			t.Errorf("Status of %s did not match, wont %v, got %v", td.path, td.status, sib.Status)
		}
		if sib.CurrentIndex() != td.current {
			t.Errorf("current index did not match, wont %d, got %d", td.current, sib.CurrentIndex())
		}
		if sib.TotalCount() != 10 {
			t.Errorf("total count did not match, wont 10, got %d", sib.TotalCount())
		}
	}
}
