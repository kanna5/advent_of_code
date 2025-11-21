// Solution for https://adventofcode.com/2023/day/4
package day04

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func nMatches(card string) (int64, error) {
	var ret int64
	parts := strings.Split(card, ": ")
	if len(parts) != 2 {
		return 0, fmt.Errorf("invalid input: %#v", card)
	}
	parts = strings.Split(parts[1], " | ")
	if len(parts) != 2 {
		return 0, fmt.Errorf("invalid input: %#v", card)
	}

	winningNums := make(lib.Set[string], 32)
	for n := range strings.FieldsSeq(parts[0]) {
		winningNums.Add(n)
	}
	for n := range strings.FieldsSeq(parts[1]) {
		if winningNums.Has(n) {
			ret += 1
		}
	}
	return ret, nil
}

func (s *sol) SolvePart1() (string, error) {
	var sum int64

	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		matches, err := nMatches(line)
		if err != nil {
			return "", err
		}
		if matches > 0 {
			sum += 1 << (matches - 1)
		}
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	matches := make([]int64, 0, 1024)

	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		m, err := nMatches(line)
		if err != nil {
			return "", err
		}
		matches = append(matches, m)
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	nCards := make([]int64, len(matches))
	for i := 0; i < len(nCards); i += 1 {
		nCards[i] = 1
	}
	var sum int64
	for i := 0; i < len(nCards); i += 1 {
		nI := nCards[i]
		sum += nI

		m := matches[i]
		for j := 1; int64(j) <= m && i+j < len(nCards); j += 1 {
			nCards[i+j] += nI
		}
	}

	return strconv.FormatInt(sum, 10), nil
}

func init() {
	solutions.Days[4] = &sol{}
}
