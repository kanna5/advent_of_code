// Solution for https://adventofcode.com/2023/day/6
//
// Ref: https://en.wikipedia.org/wiki/Quadratic_formula
package day06

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

type Game struct {
	time     int64
	distance int64
}

func (g Game) Solve() int64 {
	a := float64(1)
	b := -float64(g.time)
	c := float64(g.distance)

	d := b*b - 4*a*c
	if d <= 0 {
		return 0
	}

	x1 := (-b - math.Sqrt(d)) / 2
	x2 := (-b + math.Sqrt(d)) / 2
	return int64(math.Ceil(x2-1) - math.Floor(x1+1) + 1)
}

func (s *sol) readInput() ([]Game, error) {
	sc := bufio.NewScanner(s.input)
	if !sc.Scan() {
		if e := sc.Err(); e != nil {
			return nil, e
		}
		return nil, fmt.Errorf("failed to read input")
	}
	times, err := lib.StrToIntSlice[int64](sc.Text()[len("Time:"):])
	if err != nil {
		return nil, err
	}

	if !sc.Scan() {
		if e := sc.Err(); e != nil {
			return nil, e
		}
		return nil, fmt.Errorf("failed to read input")
	}
	distances, err := lib.StrToIntSlice[int64](sc.Text()[len("Distance:"):])
	if err != nil {
		return nil, err
	}

	if len(times) != len(distances) {
		return nil, fmt.Errorf("mismatched number of values")
	}

	ret := make([]Game, 0, len(times))
	for i := range times {
		ret = append(ret, Game{times[i], distances[i]})
	}
	return ret, nil
}

func (s *sol) SolvePart1() (string, error) {
	games, err := s.readInput()
	if err != nil {
		return "", err
	}

	ret := int64(1)
	for _, game := range games {
		ret *= game.Solve()
	}
	return strconv.FormatInt(ret, 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	games, err := s.readInput()
	if err != nil {
		return "", err
	}
	times := make([]string, 0, len(games))
	distances := make([]string, 0, len(games))
	for _, g := range games {
		times = append(times, strconv.FormatInt(g.time, 10))
		distances = append(distances, strconv.FormatInt(g.distance, 10))
	}
	realTime, err := strconv.ParseInt(strings.Join(times, ""), 10, 64)
	if err != nil {
		return "", err
	}
	realDistance, err := strconv.ParseInt(strings.Join(distances, ""), 10, 64)
	if err != nil {
		return "", err
	}

	ret := (Game{realTime, realDistance}).Solve()
	return strconv.FormatInt(ret, 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[6] = &sol{}
}
