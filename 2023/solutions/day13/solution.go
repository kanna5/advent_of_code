// Solution for https://adventofcode.com/2023/day/13
package day13

import (
	"bufio"
	"io"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func findMirrorPos[T comparable](arr []T) int {
	for pos := range len(arr) - 1 {
		for i := 0; ; i++ {
			if pos-i < 0 || pos+i+1 >= len(arr) {
				return pos + 1
			}
			if arr[pos-i] != arr[pos+i+1] {
				break
			}
		}
	}
	return 0
}

func transpose(orig []string) []string {
	var buffers = make([]strings.Builder, len(orig[0]))
	for _, row := range orig {
		for col := range len(row) {
			buffers[col].WriteByte(row[col])
		}
	}
	ret := make([]string, len(buffers))
	for i := range buffers {
		ret[i] = buffers[i].String()
	}
	return ret
}

type scoreFn func([]string) int

func (s *sol) solve(score scoreFn) (string, error) {
	sum := 0
	lines := []string{}

	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			if len(lines) > 0 {
				sum += score(lines)
				lines = []string{}
				continue
			}
		}
		lines = append(lines, line)
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	if len(lines) > 0 {
		sum += score(lines)
	}
	return strconv.FormatInt(int64(sum), 10), nil
}

func (s *sol) SolvePart1() (string, error) {
	score := func(lines []string) int {
		if rows := findMirrorPos(lines); rows != 0 {
			return 100 * rows
		}
		return findMirrorPos(transpose(lines))
	}
	return s.solve(score)
}

// getNErr compares two rows and returns the number of mismatches
func getNErr[T comparable](a, b []T) int {
	if len(a) != len(b) {
		return max(len(a), len(b))
	}
	ret := 0
	for i := range a {
		if a[i] != b[i] {
			ret++
		}
	}
	return ret
}

func findMirrorPosWError[T comparable](arr [][]T, nError int) int {
	for pos := range len(arr) - 1 {
		nErrFound := 0
		for i := 0; ; i++ {
			if pos-i < 0 || pos+i+1 >= len(arr) {
				if nErrFound == nError {
					return pos + 1
				}
				break
			}
			if nErrFound += getNErr(arr[pos-i], arr[pos+i+1]); nErrFound > 1 {
				break
			}
		}
	}
	return 0
}

func toByteSlices(orig []string) [][]byte {
	ret := make([][]byte, len(orig))
	for i := range orig {
		ret[i] = []byte(orig[i])
	}
	return ret
}

func (s *sol) SolvePart2() (string, error) {
	score := func(lines []string) int {
		linesB := toByteSlices(lines)
		if rows := findMirrorPosWError(linesB, 1); rows != 0 {
			return 100 * rows
		}
		return findMirrorPosWError(toByteSlices(transpose(lines)), 1)
	}
	return s.solve(score)
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[13] = &sol{}
}
