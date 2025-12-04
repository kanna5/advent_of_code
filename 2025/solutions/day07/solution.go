// Solution for https://adventofcode.com/2025/day/07
package day07

import (
	"bufio"
	"io"
	"slices"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct{ input io.Reader }

func (s *sol) SolvePart1() (string, error) {
	layout, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	nSplt := 0
	beams := []int{layout.startPos}
	for row := range layout.splitters {
		if len(layout.splitters[row]) == 0 {
			continue
		}
		newBeams := lib.NewSet[int]()
		for _, pos := range beams {
			if slices.Contains(layout.splitters[row], pos) {
				nSplt++
				newBeams.Add(pos-1, pos+1)
			} else {
				newBeams.Add(pos)
			}
		}
		beams = beams[:0]
		for p := range newBeams {
			beams = append(beams, p)
		}
	}

	return strconv.FormatInt(int64(nSplt), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	layout, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	beams := map[int]int{layout.startPos: 1}
	for row := range layout.splitters {
		if len(layout.splitters[row]) == 0 {
			continue
		}
		newBeams := map[int]int{}
		for pos, val := range beams {
			if slices.Contains(layout.splitters[row], pos) {
				newBeams[pos-1] += val
				newBeams[pos+1] += val
			} else {
				newBeams[pos] += val
			}
		}
		beams = newBeams
	}

	total := 0
	for _, val := range beams {
		total += val
	}

	return strconv.FormatInt(int64(total), 10), nil
}

func readInput(input io.Reader) (*Layout, error) {
	startPos := 0
	splitters := [][]int{}

	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}

		spltrs := []int{} // splitter position
		for i := range line {
			switch line[i] {
			case '.':
				continue
			case '^':
				spltrs = append(spltrs, i)
			case 'S':
				startPos = i
			}
		}
		splitters = append(splitters, spltrs)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return &Layout{
		startPos:  startPos,
		splitters: splitters,
	}, nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[7] = &sol{}
}
