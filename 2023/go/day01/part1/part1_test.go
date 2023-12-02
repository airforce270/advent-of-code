package part1_test

import (
	"testing"

	"github.com/airforce270/advent-of-code/2023/go/day01/part1"
)

func TestProcess(t *testing.T) {
	const in = `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`
	const want = "142"

	got, err := part1.Process(in)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != want {
		t.Errorf("Process() = %q, want %q", got, want)
	}
}
