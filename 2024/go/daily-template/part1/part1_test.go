package part1_test

import (
	"testing"

	"github.com/airforce270/advent-of-code/2024/go/day02/part1"
)

func TestProcess(t *testing.T) {
	const in = ``
	const want = ""

	got, err := part1.Process(in)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != want {
		t.Errorf("Process() = %q, want %q", got, want)
	}
}
