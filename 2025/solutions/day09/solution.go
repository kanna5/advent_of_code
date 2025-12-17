// Solution for https://adventofcode.com/2025/day/09
package day09

import (
	"bufio"
	"fmt"
	"io"
	"slices"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

// XXX: Despite yielding a correct answer for the given input, not all possible
// cases have been considered.

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	coords, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	maxArea := 0
	for i := range len(coords) - 1 {
		for j := i + 1; j < len(coords); j++ {
			maxArea = max(maxArea, coords[i].Area(&coords[j]))
		}
	}
	return strconv.FormatInt(int64(maxArea), 10), nil
}

func toEdges(points []Coord) ([]*Line, error) {
	edges := make([]*Line, 0, len(points))
	var last *Line
	turns := 0
	for i := range points {
		line := points[i].LineWith(&points[(i+1)%len(points)])
		if last != nil {
			switch {
			case last.Dir.TurnRight() == line.Dir:
				turns++
			case last.Dir.TurnLeft() == line.Dir:
				turns--
			default:
				return nil, fmt.Errorf("invalid input: expected turns")
			}
		}
		last = line
		edges = append(edges, line)
	}
	// Make sure coordinates are connected clockwise
	if turns < 0 {
		slices.Reverse(edges)
		for i := range edges {
			edges[i] = edges[i].Flip()
		}
	}
	return edges, nil
}

func toRect(a, b Coord) []Coord {
	if a.X == b.X || a.Y == b.Y {
		return []Coord{a, b}
	}
	return []Coord{a, {a.X, b.Y}, b, {b.X, a.Y}}
}

func isRectInside(a, b Coord, edges []*Line) bool {
	recP := toRect(a, b)
	rectE, err := toEdges(recP)
	if err != nil {
		return false // just skip the case where it's a thin line
	}

	for _, e := range edges {
		for _, rE := range rectE {
			if e.Overlays(rE) && e.Dir != rE.Dir {
				return false
			}
			if e.Cuts(rE) {
				return false
			}
		}
	}
	return true
}

func (s *sol) SolvePart2() (string, error) {
	coords, err := readInput(s.input)
	if err != nil {
		return "", err
	}
	edges, err := toEdges(coords)
	if err != nil {
		return "", err
	}

	maxArea := 0
	for i := range len(coords) - 1 {
		for j := i + 1; j < len(coords); j++ {
			if !isRectInside(coords[i], coords[j], edges) {
				continue
			}
			newMA := max(maxArea, coords[i].Area(&coords[j]))
			if newMA != maxArea {
				maxArea = newMA
			}
		}
	}
	return strconv.FormatInt(int64(maxArea), 10), nil
}

func readInput(input io.Reader) ([]Coord, error) {
	ret := []Coord{}
	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		nums, err := lib.StrToIntSlice[int](line)
		if err != nil || len(nums) != 2 {
			return nil, fmt.Errorf("invalid input %q", line)
		}

		ret = append(ret, Coord{nums[0], nums[1]})
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	return ret, nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[9] = &sol{}
}
