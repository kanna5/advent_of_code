// Solution for https://adventofcode.com/2025/day/11
package day11

import (
	"fmt"
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/solutions"
)

// Assume there are no loops.

type sol struct {
	input io.Reader
}

func findPaths(conn [][]int, current, dest int, cache []int) int {
	if cache == nil {
		cache = make([]int, len(conn))
		for i := range cache {
			cache[i] = -1
		}
		cache[dest] = 1
	}
	if cache[current] != -1 {
		return cache[current]
	}

	sum := 0
	for _, next := range conn[current] {
		sum += findPaths(conn, next, dest, cache)
	}
	cache[current] = sum
	return sum
}

func (s *sol) SolvePart1() (string, error) {
	rack, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	paths := findPaths(rack.Connections, rack.You, rack.Out, nil)
	return strconv.FormatInt(int64(paths), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	rack, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	svr, ok1 := rack.Nodes["svr"]
	fft, ok2 := rack.Nodes["fft"]
	dac, ok3 := rack.Nodes["dac"]
	if !ok1 || !ok2 || !ok3 {
		return "", fmt.Errorf("no enough nodes. required: svr, fft, dac")
	}

	fft2dac := findPaths(rack.Connections, fft, dac, nil)
	dac2fft := findPaths(rack.Connections, dac, fft, nil)
	var paths int

	switch {
	case fft2dac > 0:
		a := findPaths(rack.Connections, svr, fft, nil)
		b := findPaths(rack.Connections, dac, rack.Out, nil)
		paths = a * fft2dac * b

	case dac2fft > 0:
		a := findPaths(rack.Connections, svr, dac, nil)
		b := findPaths(rack.Connections, fft, rack.Out, nil)
		paths = a * dac2fft * b

	default:
		return "", fmt.Errorf("impossible: no path between fft and dac")
	}

	return strconv.FormatInt(int64(paths), 10), nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[11] = &sol{}
}
