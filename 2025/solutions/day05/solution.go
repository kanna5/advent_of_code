// Solution for https://adventofcode.com/2025/day/05
package day05

import (
	"bufio"
	"fmt"
	"io"
	"slices"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	ranges, ingredients, err := readInput(s.input)
	if err != nil {
		return "", err
	}
	fresh := 0
Outer:
	for _, i := range ingredients {
		for _, r := range ranges {
			if i <= r[1] && i >= r[0] {
				fresh++
				continue Outer
			}
		}
	}
	return strconv.FormatInt(int64(fresh), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	ranges, _, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	slices.SortFunc(
		ranges,
		func(a, b Range) int { return a[0] - b[0] },
	)

	num, last := 0, 0
	for i := range ranges {
		effective := max(0, ranges[i][1]-max(last+1, ranges[i][0])+1)
		num += effective
		last = max(last, ranges[i][1])
	}
	return strconv.FormatInt(int64(num), 10), nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[5] = &sol{}
}

func readInput(input io.Reader) ([]Range, []int, error) {
	sc := bufio.NewScanner(input)
	ranges := make([]Range, 0, 64)
	ingredients := make([]int, 0, 64)

	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		numbers, err := lib.StrToIntSlice[int](line)
		if err != nil {
			return nil, nil, fmt.Errorf("cannot parse range %q: %v", line, err)
		}
		if len(numbers) != 2 {
			return nil, nil, fmt.Errorf("cannot parse range %q: expected 2 numbers", line)
		}
		ranges = append(ranges, Range{numbers[0], numbers[1]})
	}
	if err := sc.Err(); err != nil {
		return nil, nil, err
	}

	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		number, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			return nil, nil, fmt.Errorf("cannot parse %q: %v", line, err)
		}
		ingredients = append(ingredients, int(number))
	}
	if err := sc.Err(); err != nil {
		return nil, nil, err
	}
	return ranges, ingredients, nil
}
