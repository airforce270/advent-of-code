package part1_test

import (
	"testing"

	"github.com/airforce270/advent-of-code/2024/go/day02/part1"
)

func TestProcess(t *testing.T) {
	const in = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`
	const want = "2"

	got, err := part1.Process(in)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != want {
		t.Errorf("Process() = %q, want %q", got, want)
	}
}
