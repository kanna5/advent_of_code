// Solution for https://adventofcode.com/2025/day/08
package day08

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"slices"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func getPairs(def int) (int, error) {
	raw := os.Getenv("PAIRS")
	if len(raw) == 0 {
		return def, nil
	}
	parsed, err := strconv.ParseInt(raw, 10, strconv.IntSize)
	if err != nil {
		return 0, err
	}
	return int(parsed), nil
}

type Dist struct {
	a, b int // id of conjunction
	d    float64
}

func getAllDistances(coords []Coord3) []Dist {
	dists := make([]Dist, 0, len(coords)*len(coords)/2)
	var between = func(a, b int) Dist {
		return Dist{
			a: a, b: b,
			d: coords[a].Distance(&coords[b]),
		}
	}
	for i := range len(coords) - 1 {
		for j := i + 1; j < len(coords); j++ {
			dists = append(dists, between(i, j))
		}
	}
	slices.SortFunc(dists, func(a, b Dist) int {
		switch {
		case a.d > b.d:
			return 1
		case a.d < b.d:
			return -1
		}
		return 0
	})
	return dists
}

func (s *sol) SolvePart1() (string, error) {
	coords, err := readInput(s.input)
	if err != nil {
		return "", err
	}
	nPairs, err := getPairs(1000)
	if err != nil {
		return "", fmt.Errorf("failed to parse number of pairs: %v", err)
	}

	dists := getAllDistances(coords)

	circuitIdx := map[int]*lib.Set[int]{}
	for i := range nPairs {
		tC := make([]*lib.Set[int], 0, 2)
		a, b := dists[i].a, dists[i].b
		if c, ok := circuitIdx[a]; ok {
			tC = append(tC, c)
		}
		if c, ok := circuitIdx[b]; ok {
			tC = append(tC, c)
		}

		var newCircuit *lib.Set[int]
		switch len(tC) {
		case 2:
			if tC[0] == tC[1] {
				continue
			}
			// combine the two circuits
			for d := range *tC[1] {
				tC[0].Add(d)
			}
			newCircuit = tC[0]
		case 1:
			tC[0].Add(a, b)
			newCircuit = tC[0]
		case 0:
			nc := lib.NewSet(a, b)
			newCircuit = &nc
		}
		for i := range *newCircuit {
			circuitIdx[i] = newCircuit
		}
	}

	circuitLengths := []int{}
	circuitsDedup := lib.Set[*lib.Set[int]]{}
	for _, c := range circuitIdx {
		if !circuitsDedup.Has(c) {
			circuitsDedup.Add(c)
			circuitLengths = append(circuitLengths, len(*c))
		}
	}
	for len(circuitLengths) < 3 {
		circuitLengths = append(circuitLengths, 1)
	}
	slices.SortFunc(circuitLengths, func(a, b int) int { return b - a })
	ans := circuitLengths[0] * circuitLengths[1] * circuitLengths[2]

	return strconv.FormatInt(int64(ans), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	coords, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	dists := getAllDistances(coords)

	nCircuits := len(coords)
	circuitIdx := map[int]*lib.Set[int]{}
	var lastPair [2]int
	for i := range dists {
		tC := make([]*lib.Set[int], 0, 2)
		a, b := dists[i].a, dists[i].b
		if c, ok := circuitIdx[a]; ok {
			tC = append(tC, c)
		}
		if c, ok := circuitIdx[b]; ok {
			tC = append(tC, c)
		}

		var newCircuit *lib.Set[int]
		switch len(tC) {
		case 2:
			if tC[0] == tC[1] {
				continue
			}
			// combine the two circuits
			for d := range *tC[1] {
				tC[0].Add(d)
			}
			newCircuit = tC[0]
		case 1:
			tC[0].Add(a, b)
			newCircuit = tC[0]
		case 0:
			nc := lib.NewSet(a, b)
			newCircuit = &nc
		}
		for i := range *newCircuit {
			circuitIdx[i] = newCircuit
		}
		nCircuits--
		if nCircuits == 1 {
			lastPair = [2]int{a, b}
			break
		}
	}

	ans := coords[lastPair[0]].X * coords[lastPair[1]].X
	return strconv.FormatInt(ans, 10), nil
}

func readInput(input io.Reader) ([]Coord3, error) {
	coords := make([]Coord3, 0, 50)

	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}

		nums, err := lib.StrToIntSlice[int64](line)
		if err != nil || len(nums) != 3 {
			return nil, fmt.Errorf("cannot parse input %q: invalid format", line)
		}
		coords = append(coords, Coord3{nums[0], nums[1], nums[2]})
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	return coords, nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[8] = &sol{}
}
