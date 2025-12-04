// Solution for https://adventofcode.com/2025/day/04
package day04

import (
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	cnt := 0
	for c := range map_.Iter() {
		cell := map_.At(c)
		if cell.typ == PaperRoll && cell.neighbors < 4 {
			cnt++
		}
	}
	return strconv.FormatInt(int64(cnt), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	queue := make([]Coord, 0, 256)
	removed := 0

	for c := range map_.Iter() {
		queue = append(queue, c)
	}

	for ; len(queue) > 0; queue = queue[1:] {
		c, cell := queue[0], map_.At(queue[0])
		if cell.typ == PaperRoll && cell.neighbors < 4 {
			// Remove it
			cell.typ = Ground
			removed++

			// Decrement to neighbors
			for nc := range map_.NeighborsSeq(c) {
				ncell := map_.At(nc)
				if ncell.typ == PaperRoll {
					ncell.neighbors--
					queue = append(queue, nc)
				}
			}
		}
	}
	return strconv.FormatInt(int64(removed), 10), nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[4] = &sol{}
}
