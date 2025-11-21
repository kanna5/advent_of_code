// Solution for https://adventofcode.com/2023/day/23
package day23

// The longest path problem is NP-hard, but in part 1 the "slopes" add enough
// constraints to make it easy to brute-force a solution.
//
// In part 2, after ignoring the slopes, the map essentially becomes an
// undirected graph. Simulating directly on the map as in part 1 adds too much
// overhead, but compressing it into a simple graph yields a relatively small
// dataset that can be brute-forced in a few seconds.

import (
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func longestTrail(m Map, cur Coord, curLen int, walked lib.Set[Coord]) int {
	if cur.X == m.W()-2 && cur.Y == m.H()-1 {
		return curLen
	}
	walked.Add(cur)
	defer walked.Del(cur)

	next := m.WalkableFrom(cur, false)
	maxLen := -1
	for i := range next {
		if walked.Has(next[i]) {
			continue
		}
		maxLen = max(maxLen, longestTrail(m, next[i], curLen+1, walked))
	}
	return max(maxLen, 0)
}

func (s *sol) SolvePart1() (string, error) {
	m, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	l := longestTrail(m, Coord{1, 0}, 0, lib.Set[Coord]{})
	return strconv.FormatInt(int64(l), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	m, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	l := m.ToGraph().LongestPath()
	return strconv.FormatInt(int64(l), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[23] = &sol{}
}
