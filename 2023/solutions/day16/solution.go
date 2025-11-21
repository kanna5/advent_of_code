// Solution for https://adventofcode.com/2023/day/16
package day16

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

	map_ = map_.inputLight(Coordinate{0, 0}, Right)
	return strconv.FormatInt(int64(map_.countEnergized()), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	maxN := 0
	for y := range map_ {
		nEnergized1 := map_.inputLight(Coordinate{0, y}, Right).countEnergized()
		nEnergized2 := map_.inputLight(Coordinate{len(map_[0]) - 1, y}, Left).countEnergized()
		maxN = max(maxN, nEnergized1, nEnergized2)
	}
	for x := range map_[0] {
		nEnergized1 := map_.inputLight(Coordinate{x, 0}, Down).countEnergized()
		nEnergized2 := map_.inputLight(Coordinate{x, len(map_) - 1}, Up).countEnergized()
		maxN = max(maxN, nEnergized1, nEnergized2)
	}
	return strconv.FormatInt(int64(maxN), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[16] = &sol{}
}
