// Solution for https://adventofcode.com/2025/day/06
package day06

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func mulCol(nums [][]int64, col int) int64 {
	var ret int64 = 1
	for i := range nums {
		ret *= nums[i][col]
	}
	return ret
}

func addCol(nums [][]int64, col int) int64 {
	var ret int64 = 0
	for i := range nums {
		ret += nums[i][col]
	}
	return ret
}

func (s *sol) SolvePart1() (string, error) {
	var numbers [][]int64
	var grandSum int64 = 0

	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		row, err := lib.StrToIntSlice[int64](line)
		if err == nil {
			if len(numbers) > 0 && len(numbers[0]) != len(row) {
				return "", fmt.Errorf("got variable length rows: %q", line)
			}
			numbers = append(numbers, row)

		} else {
			if len(numbers) == 0 {
				return "", fmt.Errorf("no numbers")
			}
			ops := strings.Fields(line)
			if len(ops) != len(numbers[0]) {
				return "", fmt.Errorf("invalid number of operators. Expected %d, got %d", len(numbers[0]), len(ops))
			}

			for i, op := range ops {
				if len(op) != 1 {
					return "", fmt.Errorf("invalid operator %q", op)
				}
				switch Operator(op[0]) {
				case Add:
					grandSum += addCol(numbers, i)
				case Mul:
					grandSum += mulCol(numbers, i)
				default:
					return "", fmt.Errorf("invalid operator %q", op)
				}
			}
			break
		}
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	return strconv.FormatInt(grandSum, 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	var sheet []string

	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		if len(sheet) > 0 && len(sheet[0]) != len(line) {
			return "", fmt.Errorf("got variable length at %q", line)
		}
		sheet = append(sheet, line)
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	var curNum, curSum, grandSum int64 = 0, 0, 0
	var curOp = InvalidOperator

	for col := range sheet[0] {
		empty := true
		for row := range sheet {
			c := sheet[row][col]
			if c == ' ' {
				continue
			}

			empty = false
			switch {
			case c >= '0' && c <= '9':
				curNum = curNum*10 + int64(c-'0')
			case Operator(c) == Add:
				curSum = 0
				curOp = Add
			case Operator(c) == Mul:
				curSum = 1
				curOp = Mul
			default:
				return "", fmt.Errorf("invalid character %q", c)
			}
		}

		if empty {
			grandSum += curSum
			curNum, curSum = 0, 0
		} else {
			switch curOp {
			case Add:
				curSum += curNum
			case Mul:
				curSum *= curNum
			default:
				return "", fmt.Errorf("invalid operator %q", curOp)
			}
			curNum = 0
		}
	}
	grandSum += curSum

	return strconv.FormatInt(grandSum, 10), nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func init() {
	solutions.Days[6] = &sol{}
}
