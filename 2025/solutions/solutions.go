package solutions

import "io"

type Solver interface {
	WithInput(i io.Reader)
	SolvePart1() (string, error)
	SolvePart2() (string, error)
}

var (
	Days [13]Solver // Use index 1 ~ 12
)
