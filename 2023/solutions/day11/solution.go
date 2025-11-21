// Solution for https://adventofcode.com/2023/day/11
package day11

import (
	"bufio"
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

type Coord struct {
	x, y int
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func distance(a, b *Coord) int {
	return abs(a.x-b.x) + abs(a.y-b.y)
}

func (s *sol) solve(expansionFactor int) (string, error) {
	var horizontal [][]*Coord
	var vertical [][]*Coord
	var stars []*Coord

	sc := bufio.NewScanner(s.input)
	y := 0
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}

		if horizontal == nil {
			horizontal = make([][]*Coord, len(line))
		}
		vert := []*Coord{}
		for x, c := range line {
			if c != '#' {
				continue
			}
			star := &Coord{x, y}
			horizontal[x] = append(horizontal[x], star)
			vert = append(vert, star)
			stars = append(stars, star)
		}
		vertical = append(vertical, vert)
		y += 1
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	// Adjust coordinates according to space expansion
	// Horizontal
	correctionH := 0
	for _, bkt := range horizontal {
		if len(bkt) == 0 {
			correctionH += expansionFactor
			continue
		}
		for _, star := range bkt {
			star.x += correctionH
		}
	}
	// Vertical
	correctionV := 0
	for _, bkt := range vertical {
		if len(bkt) == 0 {
			correctionV += expansionFactor
			continue
		}
		for _, star := range bkt {
			star.y += correctionV
		}
	}

	var sum int64
	for i := range len(stars) - 1 {
		for j := i + 1; j < len(stars); j++ {
			sum += int64(distance(stars[i], stars[j]))
		}
	}
	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) SolvePart1() (string, error) {
	return s.solve(1)
}

func (s *sol) SolvePart2() (string, error) {
	return s.solve(999_999)
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[11] = &sol{}
}
