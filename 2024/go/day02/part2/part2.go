// Package part2 implements the solver for AoC Day 2 Part 2
package part2

import (
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

const (
	lower    = 1
	upper    = 3
	sentinel = math.MinInt
)

func cmpAscending(a, b int) int {
	return a - b
}

func cmpDescending(a, b int) int {
	return b - a
}

type report struct {
	levels []int
}

func (r report) isSafe() bool {
	tests := make([]int, len(r.levels)+1)
	tests[0] = sentinel
	copy(tests[1:], r.levels)

	for removedI, removed := range tests {
		levelsRemoved := make([]int, len(r.levels))
		copy(levelsRemoved, r.levels)
		if removed != sentinel {
			levelsRemoved = append(levelsRemoved[:removedI-1], levelsRemoved[removedI:]...)
		}

		sorted := slices.IsSortedFunc(levelsRemoved, cmpAscending) ||
			slices.IsSortedFunc(levelsRemoved, cmpDescending)

		hasBad := false
		for i := 0; i < len(levelsRemoved)-1; i++ {
			diff := math.Abs(float64(levelsRemoved[i] - levelsRemoved[i+1]))
			if !(diff >= lower && diff <= upper) {
				fmt.Printf("levels=%v, [%d->%d] [%d->%d] diff=%v\n", levelsRemoved, i, levelsRemoved[i], i+1, levelsRemoved[i+1], diff)
				hasBad = true
			}
		}

		if sorted && !hasBad {
			fmt.Println("safe")
			return true
		}
	}

	fmt.Println("not safe")
	return false
}

func parse(line string) []int {
	var levels []int
	for _, level := range strings.Split(line, " ") {
		l, err := strconv.Atoi(level)
		if err != nil {
			panic(err)
		}
		levels = append(levels, l)
	}
	return levels
}

func Process(input string) (string, error) {
	var reports []report
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		reports = append(reports, report{levels: parse(line)})
	}

	safe := 0
	for _, report := range reports {
		if report.isSafe() {
			safe++
		}
	}

	return strconv.Itoa(safe), nil
}
