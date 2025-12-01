// Solution for https://adventofcode.com/2025/day/01
package day01

import (
	"bufio"
	"fmt"
	"io"
	"iter"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func iterInput(input io.Reader) iter.Seq2[*Instruction, error] {
	sc := bufio.NewScanner(input)
	return func(yield func(*Instruction, error) bool) {
		for sc.Scan() {
			line := sc.Text()
			if len(line) == 0 {
				break
			}
			var dir Direction
			switch line[0] {
			case 'L':
				dir = Left
			case 'R':
				dir = Right
			default:
				yield(nil, fmt.Errorf("invalid direction %q", line[0]))
				return
			}

			num, err := strconv.ParseInt(line[1:], 10, 64)
			if err != nil {
				yield(nil, fmt.Errorf("invalid number %q", line[1:]))
				return
			}
			if !yield(&Instruction{
				dir:  dir,
				dist: int(num),
			}, nil) {
				return
			}
		}
		if err := sc.Err(); err != nil {
			yield(nil, err)
		}
	}
}

func (s *sol) SolvePart1() (string, error) {
	count, pos := 0, 50

	for inst, err := range iterInput(s.input) {
		if err != nil {
			return "", err
		}

		pos = (pos + inst.ToInt()) % 100
		switch {
		case pos < 0:
			pos += 100
		case pos == 0:
			count += 1
		}
	}
	return strconv.FormatInt(int64(count), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	count, pos := 0, 50

	for inst, err := range iterInput(s.input) {
		if err != nil {
			return "", err
		}

		move := inst.ToInt()
		count += lib.Abs(move / 100)
		rem := move % 100
		if rem != 0 {
			pos += rem
			if pos%100 != pos {
				count++
				pos %= 100
			}
			if pos < 0 {
				count++
				pos += 100
			}
		}
	}
	return strconv.FormatInt(int64(count), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[1] = &sol{}
}
