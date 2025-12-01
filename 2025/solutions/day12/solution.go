// Solution for https://adventofcode.com/2025/day/12
package day12

import (
	"io"

	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	panic("unimplemented")
}

func (s *sol) SolvePart2() (string, error) {
	panic("unimplemented")
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[12] = &sol{}
}
