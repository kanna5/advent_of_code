// Solution for https://adventofcode.com/2025/day/03
package day03

import (
	"bufio"
	"fmt"
	"io"
	"iter"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func iterInput(input io.Reader) iter.Seq2[BatteryBank, error] {
	sc := bufio.NewScanner(input)
	return func(yield func(BatteryBank, error) bool) {
		for sc.Scan() {
			line := sc.Bytes()
			if len(line) == 0 {
				break
			}
			nums := make([]uint8, len(line))
			for i, b := range line {
				if b < '0' || b > '9' {
					yield(nil, fmt.Errorf("invalid input %q: invalid number %q", line, b))
					return
				}
				nums[i] = b - '0'
			}
			if !yield(BatteryBank(nums), nil) {
				return
			}
		}
		if err := sc.Err(); err != nil {
			yield(nil, err)
		}
	}
}

func findMaxJoltage(bb BatteryBank, n int) int64 {
	var ret int64
	pos := -1

	for i := range n {
		// find the (i+1)th battery
		p, maxJ := pos, uint8(0)
		rem := n - i - 1
		for j := pos + 1; j < len(bb)-rem; j++ {
			if bb[j] > maxJ {
				maxJ = bb[j]
				p = j
			}
		}
		pos = p
		ret = ret*10 + int64(maxJ)
	}
	return ret
}

func (s *sol) SolvePart1() (string, error) {
	var sum int64
	for bb, err := range iterInput(s.input) {
		if err != nil {
			return "", err
		}
		sum += findMaxJoltage(bb, 2)
	}

	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	var sum int64
	for bb, err := range iterInput(s.input) {
		if err != nil {
			return "", err
		}
		sum += findMaxJoltage(bb, 12)
	}

	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[3] = &sol{}
}
