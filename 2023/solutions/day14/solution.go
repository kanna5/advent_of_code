// Solution for https://adventofcode.com/2023/day/14
package day14

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

	map_.TiltNorth()
	return strconv.FormatInt(int64(map_.Load()), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	cycle := func() {
		map_.TiltNorth()
		map_.TiltWest()
		map_.TiltSouth()
		map_.TiltEast()
	}

	var loopBase, loopLength int
	cache := map[string]int{}
	loads := []int{}
	for i := 1; ; i++ {
		cycle()
		key := map_.String()
		if first, ok := cache[key]; ok {
			loopBase = first
			loopLength = i - loopBase
			break
		}
		load := map_.Load()
		loads = append(loads, load)
		cache[key] = i
	}
	offset := (1_000_000_000 - loopBase) % loopLength
	return strconv.FormatInt(int64(loads[loopBase-1+offset]), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[14] = &sol{}
}
