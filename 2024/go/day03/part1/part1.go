// Package part1 implements the solver for
package part1

import (
	"regexp"
	"strconv"
)

var pattern = regexp.MustCompile(`mul\((\d+),(\d+)\)`)

func Process(input string) (string, error) {
	matches := pattern.FindAllStringSubmatch(input, -1)

	total := 0

	for _, match := range matches {
		a, err := strconv.Atoi(match[1])
		if err != nil {
			return "", err
		}
		b, err := strconv.Atoi(match[2])
		if err != nil {
			return "", err
		}

		total += a * b
	}

	return strconv.Itoa(total), nil
}
