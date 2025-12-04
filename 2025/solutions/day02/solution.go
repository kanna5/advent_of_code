// Solution for https://adventofcode.com/2025/day/02
package day02

import (
	"bufio"
	"fmt"
	"io"
	"iter"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func iterInput(input io.Reader) iter.Seq2[Range, error] {
	reader := bufio.NewReader(input)
	return func(yield func(Range, error) bool) {
		for {
			s, err := reader.ReadString(',')
			eof := err == io.EOF
			if err != nil && !eof {
				yield(Range{}, err)
				return
			}
			s = strings.Trim(s, ",\n\r")
			if len(s) == 0 {
				return
			}
			if r, err := parseRange(s); err != nil {
				yield(Range{}, fmt.Errorf("invalid range %q: %v", s, err))
				return
			} else {
				if !yield(r, nil) || eof {
					return
				}
			}
		}
	}
}

func isInvalid1(num int64) bool {
	str := strconv.FormatInt(num, 10)
	if len(str)%2 != 0 {
		return false
	}
	half := len(str) / 2
	return str[0:half] == str[half:]
}

func (s *sol) SolvePart1() (string, error) {
	var sumInvalid int64
	for range_, err := range iterInput(s.input) {
		if err != nil {
			return "", err
		}

		for num := range_.L; num <= range_.R; num++ {
			if isInvalid1(num) {
				sumInvalid += num
			}
		}
	}
	return strconv.FormatInt(sumInvalid, 10), nil
}

func isRepeated(s string, lenSeq int) bool {
	if len(s)%lenSeq != 0 {
		return false
	}
	for l := lenSeq; l+lenSeq <= len(s); l += lenSeq {
		if s[0:lenSeq] != s[l:l+lenSeq] {
			return false
		}
	}
	return true
}

func isInvalid2(num int64) bool {
	str := strconv.FormatInt(num, 10)
	for l := 1; l <= len(str)/2; l++ {
		if isRepeated(str, l) {
			return true
		}
	}
	return false
}

func (s *sol) SolvePart2() (string, error) {
	var sumInvalid int64
	for range_, err := range iterInput(s.input) {
		if err != nil {
			return "", err
		}

		for num := range_.L; num <= range_.R; num++ {
			if isInvalid2(num) {
				sumInvalid += num
			}
		}
	}
	return strconv.FormatInt(sumInvalid, 10), nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[2] = &sol{}
}
