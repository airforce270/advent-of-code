// Day02 implements a solver for AoC 2024 day 2.
package main

import (
	_ "embed"
	"fmt"

	"github.com/airforce270/advent-of-code/2024/go/day03/part1"
	"github.com/airforce270/advent-of-code/2024/go/day03/part2"
)

//go:embed input.txt
var input string

func main() {
	answer1, err := part1.Process(input)
	if err != nil {
		fmt.Printf("part1 failed: %v\n", err)
	}
	fmt.Printf("answer1: %s\n", answer1)

	answer2, err := part2.Process(input)
	if err != nil {
		fmt.Printf("part2 failed: %v\n", err)
	}
	fmt.Printf("answer2: %s\n", answer2)
}
