// Package part1 implements the solver for
package part1

import (
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

const (
	lower = 1
	upper = 3
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
	sorted := slices.IsSortedFunc(r.levels[:], cmpAscending) ||
		slices.IsSortedFunc(r.levels[:], cmpDescending)

	if !sorted {
		fmt.Printf("not sorted: %v\n", r.levels)
		return false
	}

	for i := 0; i < len(r.levels)-1; i++ {
		diff := math.Abs(float64(r.levels[i] - r.levels[i+1]))
		if !(diff >= lower && diff <= upper) {
			fmt.Printf("levels=%v, [%d->%d] [%d->%d] diff=%v\n", r.levels, i, r.levels[i], i+1, r.levels[i+1], diff)
			return false
		}
	}

	fmt.Println("safe")
	return true
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
