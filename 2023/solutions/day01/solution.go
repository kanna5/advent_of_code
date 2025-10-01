package day01

import (
	"fmt"
	"io"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	return "", fmt.Errorf("unimplemented")
}

func (s *sol) SolvePart2() (string, error) {
	return "", fmt.Errorf("unimplemented")
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[1] = &sol{}
}
