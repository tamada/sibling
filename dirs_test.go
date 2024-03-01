package sibling

import (
	"os"
	"path/filepath"
	"testing"
)

func TestNewDirsError(t *testing.T) {
	testdata := []struct {
		givenPath string
	}{
		{"testdata/not_exit_dir"},
		{"testdata/textdata.txt"},
	}
	for _, td := range testdata {
		_, err := NewDirs(td.givenPath)
		if err == nil {
			t.Errorf("NewDirs(%s) wont error, but got no error", td.givenPath)
		}
	}
}

func TestNewDirsWithWorkingDir(t *testing.T) {
	os.Chdir("testdata/numbers/3")
	dirs, _ := NewDirs(".")
	if dirs.CurrentPath() != "../3" {
		t.Errorf("current path did not match, wont \"../3\", got %s", dirs.CurrentPath())
	}
}

func TestNewDirs(t *testing.T) {
	testdata := []struct {
		givenPath     string
		wontParent    string
		wontEntrySize int
		wontCurrent   int
	}{
		{"testdata/numbers/9", "testdata/numbers", 10, 9},
		{"testdata/alphabets/c", "testdata/alphabets", 26, 2},
	}

	for _, td := range testdata {
		dirs, _ := NewDirs(td.givenPath)
		if dirs.CurrentName() != filepath.Base(td.givenPath) {
			t.Errorf("%s: current name did not match, got %s, wont %s", td.givenPath, dirs.CurrentName(), filepath.Base(td.givenPath))
		}
		if dirs.Parent != td.wontParent {
			t.Errorf("%s: parent did not match, got %s, wont %s", td.givenPath, dirs.Parent, td.wontParent)
		}
		if dirs.Current != td.wontCurrent {
			t.Errorf("%s: current did not match, got %d, wont %d", td.givenPath, dirs.Current, td.wontCurrent)
		}
		if len(dirs.Entries) != td.wontEntrySize {
			t.Errorf("%s: entry size did not match, got %d, wont %d", td.givenPath, len(dirs.Entries), td.wontEntrySize)
		}
	}
}
