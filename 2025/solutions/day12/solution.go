// Solution for https://adventofcode.com/2025/day/12
package day12

// This is an NP-complete packing problem. A "real" solution could take hours
// to compute. However, the actual input can be solved with simple "shortcuts."

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	shapes, regions, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	maxShapeW, maxShapeH := 1, 1
	for _, s := range shapes {
		maxShapeW = max(maxShapeW, s.w)
		maxShapeH = max(maxShapeH, s.h)
	}

	cnt := 0
	for j, r := range regions {
		minReqSpace, nPresents := 0, 0
		for i, n := range r.presents {
			minReqSpace += n * shapes[i].Vol()
			nPresents += n
		}

		rVol := r.w * r.h
		if rVol < minReqSpace {
			// Definitely not possible
			continue
		}
		if (r.w/maxShapeW)*(r.h/maxShapeH) >= nPresents {
			// Definitely enough
			cnt++
			continue
		}

		return "", fmt.Errorf("unhandled situation: solving region %d requires shape packing, which is not implemented", j)
	}

	return strconv.FormatInt(int64(cnt), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	return "Congrats! 🌟", nil
}

func readInput(input io.Reader) ([]*Shape, []*Region, error) {
	shapes := []*Shape{}
	regions := []*Region{}

	sc := bufio.NewScanner(input)
	for {
		line, err := lib.NextLine(sc)
		if err != nil && err != io.EOF {
			return nil, nil, err
		}
		if len(line) == 0 {
			break
		}

		if strings.Contains(line, "x") {
			r, err := parseRegion(line)
			if err != nil {
				return nil, nil, fmt.Errorf("failed to parse region %q: %v", line, err)
			}
			regions = append(regions, r)
		} else {
			lbl := strings.Trim(line, " :")
			s, err := parseShape(sc, lbl)
			if err != nil {
				return nil, nil, fmt.Errorf("failed to parse shape %q: %v", lbl, err)
			}
			shapes = append(shapes, s)
		}
	}

	return shapes, regions, nil
}

func parseShape(sc *bufio.Scanner, label string) (*Shape, error) {
	s := Shape{label: label, tiles: [][]bool{}}

	for sc.Scan() {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}
		if s.w != 0 && len(line) != s.w {
			return nil, fmt.Errorf("got variable length")
		}
		s.w = len(line)
		s.h++
		row := make([]bool, len(line))
		for i, c := range line {
			switch c {
			case '#':
				row[i] = true
			case '.':
			default:
				return nil, fmt.Errorf("invalid tile %q", c)
			}
		}
		s.tiles = append(s.tiles, row)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	return &s, nil
}

func parseRegion(line string) (*Region, error) {
	r := Region{}

	parts := strings.Split(line, ":")
	if len(parts) != 2 {
		return nil, fmt.Errorf("invalid format")
	}
	dimensions := strings.Split(parts[0], "x")
	if len(dimensions) != 2 {
		return nil, fmt.Errorf("invalid dimensions")
	}
	w, err1 := strconv.ParseInt(dimensions[0], 10, strconv.IntSize)
	h, err2 := strconv.ParseInt(dimensions[1], 10, strconv.IntSize)
	if err1 != nil || err2 != nil || w <= 0 || h <= 0 {
		return nil, fmt.Errorf("invalid dimensions")
	}
	r.w, r.h = int(w), int(h)

	presents, err := lib.StrToIntSlice[int](parts[1])
	if err != nil {
		return nil, fmt.Errorf("failed to parse numbers: %v", err)
	}
	r.presents = presents
	return &r, nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[12] = &sol{}
}
