// Solution for https://adventofcode.com/2023/day/1
package day01

import (
	"bufio"
	"fmt"
	"io"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	sc := bufio.NewScanner(s.input)
	var sum int64 = 0

	for sc.Scan() {
		var first, last *int64
		line := sc.Text()
		for _, c := range line {
			digit := int64(c - '0')
			if digit <= 0 || digit > 9 {
				continue
			}

			if first == nil {
				first = &digit
			}
			last = &digit
		}
		if first == nil || last == nil {
			return "", fmt.Errorf("invalid input line %v: no enough digits", line)
		}
		sum += *first*10 + *last
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	return fmt.Sprintf("%d", sum), nil
}

var digits = map[string]int64{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func parseDigit(str []rune) *int64 {
	strLen := len(str)
	if strLen > 0 {
		n := int64(str[0] - '0')
		if n > 0 && n <= 9 {
			return &n
		}
	}

	for d := range digits {
		dLen := len(d)
		if strLen >= dLen && string(str[:dLen]) == d {
			ret := digits[d]
			return &ret
		}
	}
	return nil
}

func (s *sol) SolvePart2() (string, error) {
	var sum int64 = 0
	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		var first, last *int64
		line := []rune(sc.Text())

		for ; len(line) > 0; line = line[1:] {
			n := parseDigit(line)
			if n == nil {
				continue
			}
			if first == nil {
				first = n
			}
			last = n
		}

		if first == nil || last == nil {
			return "", fmt.Errorf("invalid input line %v: no enough digits", string(line))
		}
		sum += *first*10 + *last
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	return fmt.Sprintf("%d", sum), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[1] = &sol{}
}
