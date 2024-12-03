// Package part2 implements the solver for AoC Day 2 Part 2
package part2

import (
	"regexp"
	"strconv"
)

var pattern = regexp.MustCompile(`(mul|do|don't)\(((\d+),?(\d+))?\)`)

func Process(input string) (string, error) {
	matches := pattern.FindAllStringSubmatch(input, -1)

	total := 0
	enabled := true

	for _, match := range matches {
		switch match[1] {
		case "do":
			enabled = true
			continue
		case "don't":
			enabled = false
			continue
		}

		if !enabled {
			continue
		}

		if match[3] == "" || match[4] == "" {
			continue
		}

		a, err := strconv.Atoi(match[3])
		if err != nil {
			return "", err
		}
		b, err := strconv.Atoi(match[4])
		if err != nil {
			return "", err
		}

		total += a * b
	}

	return strconv.Itoa(total), nil
}
