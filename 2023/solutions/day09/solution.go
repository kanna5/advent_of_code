// Solution for https://adventofcode.com/2023/day/9
package day09

import (
	"bufio"
	"fmt"
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func getHistory(seq []int64) ([][]int64, error) {
	history := [][]int64{seq}
	for {
		cur := history[len(history)-1]
		allZero := true
		for _, val := range cur {
			if val != 0 {
				allZero = false
				break
			}
		}
		if allZero {
			break
		}

		// calculate next row
		if len(cur) <= 1 {
			return nil, fmt.Errorf("last row in history is not all-zero")
		}
		row := make([]int64, 0, len(cur)-1)
		for i := range len(cur) - 1 {
			row = append(row, cur[i+1]-cur[i])
		}
		history = append(history, row)
	}
	return history, nil
}

func extrapolate(seq []int64, forward bool) (int64, error) {
	history, err := getHistory(seq)
	if err != nil {
		return 0, fmt.Errorf("failed to calculate history: %#v", err)
	}

	var val int64
	for i := len(history) - 2; i >= 0; i-- {
		if forward {
			val = history[i][len(history[i])-1] + val
		} else {
			val = history[i][0] - val
		}
	}
	return val, nil
}

func (s *sol) solve(forward bool) (string, error) {
	var sum int64
	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		row, err := lib.StrToIntSlice[int64](line)
		if err != nil {
			return "", fmt.Errorf("failed to parse %#v: %#v", line, err)
		}
		extrapolated, err := extrapolate(row, forward)
		if err != nil {
			return "", fmt.Errorf("unable to find extrapolated values for %#v: %#v", line, err)

		}
		sum += extrapolated
	}

	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) SolvePart1() (string, error) {
	return s.solve(true)
}

func (s *sol) SolvePart2() (string, error) {
	return s.solve(false)
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[9] = &sol{}
}
