package part1_test

import (
	"testing"

	"github.com/airforce270/advent-of-code/2024/go/day03/part1"
)

func TestProcess(t *testing.T) {
	const in = `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
	const want = "161"

	got, err := part1.Process(in)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != want {
		t.Errorf("Process() = %q, want %q", got, want)
	}
}
