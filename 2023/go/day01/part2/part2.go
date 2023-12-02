// Package part2 implements the solver for AoC 2023 Day 1 - Part 2.
package part2

import (
	"strconv"
	"strings"
)

var nums = map[string]int{
	"1":     1,
	"2":     2,
	"3":     3,
	"4":     4,
	"5":     5,
	"6":     6,
	"7":     7,
	"8":     8,
	"9":     9,
	"0":     0,
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func Process(input string) (string, error) {
	var total int

	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}

		var found []int
		for i := range line {
			for num, val := range nums {
				if strings.HasPrefix(line[i:], num) {
					found = append(found, val)
				}
			}
		}
		first := found[0]
		last := found[len(found)-1]
		total += (first * 10) + last
	}

	return strconv.Itoa(total), nil
}
