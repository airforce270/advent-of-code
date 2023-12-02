package part2_test

import (
	"testing"

	"github.com/airforce270/advent-of-code/2023/go/day01/part2"
)

func TestProcess(t *testing.T) {
	const in = `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`
	const want = "281"

	got, err := part2.Process(in)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != want {
		t.Errorf("Process() = %q, want %q", got, want)
	}
}
