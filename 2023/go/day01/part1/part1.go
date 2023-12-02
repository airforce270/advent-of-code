// Package part1 implements the solver for AoC 2023 Day 1 - Part 1.
package part1

import (
	"fmt"
	"strconv"
	"strings"
)

var nums = []string{"1", "2", "3", "4", "5", "6", "7", "8", "9", "0"}

func Process(input string) (string, error) {
	var total int

	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}

		var found []string
		for i := range line {
			for _, num := range nums {
				if strings.HasPrefix(line[i:], num) {
					found = append(found, num)
				}
			}
		}
		first := found[0]
		last := found[len(found)-1]
		lineVal, err := strconv.Atoi(first + last)
		if err != nil {
			return "", fmt.Errorf("failed to parse %s to int", first+last)
		}
		total += lineVal
	}

	return strconv.Itoa(total), nil
}
