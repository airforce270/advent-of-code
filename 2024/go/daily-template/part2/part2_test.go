package part2_test

import (
	"testing"

	"github.com/airforce270/advent-of-code/2024/go/day02/part2"
)

func TestProcess(t *testing.T) {
	const in = ``
	const want = ""

	got, err := part2.Process(in)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != want {
		t.Errorf("Process() = %q, want %q", got, want)
	}
}
