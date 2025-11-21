// Solution for https://adventofcode.com/2023/day/17
package day17

import (
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}
	l := map_.findLeastLoss()
	return strconv.FormatInt(int64(l), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}
	l := map_.findLeastLoss2()
	return strconv.FormatInt(int64(l), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[17] = &sol{}
}
