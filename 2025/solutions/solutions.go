package solutions

import "io"

type Solver interface {
	WithInput(i io.Reader) Solver
	SolvePart1() (string, error)
	SolvePart2() (string, error)
}

var (
	Days [26]Solver // Use index 1 ~ 25
)
