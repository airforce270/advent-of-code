package part2_test

import (
	"testing"

	"github.com/airforce270/advent-of-code/2024/go/day03/part2"
)

func TestProcess(t *testing.T) {
	const in = `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`
	const want = "48"

	got, err := part2.Process(in)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != want {
		t.Errorf("Process() = %q, want %q", got, want)
	}
}
