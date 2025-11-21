// Solution for https://adventofcode.com/2023/day/21
package day21

// The structure of the actual input (again) is key to solving part 2.
// - The starting point is at the center of the map, and the center row and
//   column are empty.
// - The map is a square with an odd side length.
// - The given number of steps equals a multiple of the map's width plus the
//   distance from the center to the edge.

import (
	"fmt"
	"io"
	"os"
	"slices"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func getSteps(def int) (int, error) {
	env := os.Getenv("STEPS")
	if len(env) == 0 {
		return def, nil
	}
	parsed, err := strconv.ParseInt(env, 10, 64)
	return int(parsed), err
}

func (s *sol) SolvePart1() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}
	steps, err := getSteps(64)
	if err != nil {
		return "", fmt.Errorf("failed to get target steps: %v", err)
	}

	plots := lib.NewSet(map_.start)
	for range steps {
		nextPlots := make(lib.Set[Coord], len(plots)*11/10)
		for c := range plots {
			n := c.Neighbors()
			for i := range n {
				if map_.Contains(n[i]) && map_.At(n[i]) == Plot {
					nextPlots.Add(n[i])
				}
			}
		}
		plots = nextPlots
	}
	return strconv.FormatInt(int64(len(plots)), 10), nil
}

func Part2Brute(map_ *Map, steps int) []int {
	ret := slices.Grow([]int{1}, steps)

	cur, curN := lib.NewSet(map_.start), 1
	alt, altN := lib.NewSet[Coord](), 0
	for range steps {
		next := make(lib.Set[Coord], len(cur)*11/10)
		for c := range cur {
			neigh := c.Neighbors()
			for i := range neigh {
				if !alt.Has(neigh[i]) && map_.At(neigh[i]) == Plot {
					next.Add(neigh[i])
				}
			}
		}
		altN += len(next)

		curN, altN = altN, curN
		cur, alt = next, cur
		ret = append(ret, curN)
	}
	return ret
}

func walk(map_ *Map, steps int) lib.Set[Coord] {
	covered := lib.NewSet(map_.start)
	expanded := []Coord{map_.start}
	for range steps {
		next := []Coord{}
		for c := range expanded {
			n := expanded[c].Neighbors()
			for i := range n {
				if covered.Has(n[i]) || map_.At(n[i]) != Plot {
					continue
				}
				covered.Add(n[i])
				next = append(next, n[i])
			}
		}
		expanded = next
	}
	return covered
}

func (s *sol) SolvePart2() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}
	steps, err := getSteps(26501365)
	if err != nil {
		return "", fmt.Errorf("failed to get target steps: %v", err)
	}

	var dist = func(x, y int) int { return lib.Abs(x-map_.start.x) + lib.Abs(y-map_.start.y) }

	distToEdge := map_.w / 2
	diamond1 := walk(map_, distToEdge)
	diamond3 := walk(map_, distToEdge+map_.w)

	var nEven, nOdd int64
	for y := range map_.h {
		for x := range map_.w {
			if map_.At(Coord{x, y}) != Plot || !diamond3.Has(Coord{x, y}) {
				continue
			}
			if dist(x, y)%2 == 0 {
				nEven++
			} else {
				nOdd++
			}
		}
	}

	var nDiamondInner int64
	for c := range diamond1 {
		if dist(c.x, c.y)%2 == 1 {
			nDiamondInner++
		}
	}
	nDiamondUncovered := nOdd - nDiamondInner

	var nReverseDiamond int64
	for c := range diamond3 {
		if (c.x < 0 && c.y < 0) || (c.x >= map_.w && c.y < 0) ||
			(c.x >= map_.w && c.y >= map_.h) || (c.x < 0 && c.y >= map_.h) {
			if dist(c.x, c.y)%2 == 0 {
				nReverseDiamond++
			}
		}
	}

	tiles := int64(steps / map_.h)
	n := (tiles+1)*(tiles+1)*nOdd + tiles*tiles*nEven
	n -= nDiamondUncovered * (tiles + 1)
	n += nReverseDiamond * tiles
	return strconv.FormatInt(n, 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[21] = &sol{}
}
