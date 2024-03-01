package main

func Example_Main() {
	goMain([]string{"sibling", "--type", "next", "../../testdata/alphabets/c"})
	// Output:
	// ../../testdata/alphabets/d
}

func Example_Main_withGreatStep() {
	goMain([]string{"sibling", "--type", "previous", "--quiet", "--parent", "--step", "100", "--progress", "../../testdata/alphabets/c"})
	// Output:
	// ../../testdata/alphabets
}

func Example_Main_withStepAndProgress() {
	goMain([]string{"sibling", "--type", "previous", "--step", "2", "--progress", "../../testdata/alphabets/c"})
	// Output:
	// ../../testdata/alphabets/a (1/26)
}

func Example_Main_withProgress() {
	goMain([]string{"sibling", "--type", "next", "--progress", "../../testdata/alphabets/c"})
	// Output:
	// ../../testdata/alphabets/d (4/26)
}
