package lib

import (
	"bufio"
	"fmt"
	"io"
	"slices"
	"strconv"
	"strings"

	"golang.org/x/exp/constraints"
)

func StrToIntSlice[T int | int64](str string) ([]T, error) {
	fields := strings.FieldsFunc(str, func(r rune) bool {
		return slices.Contains([]rune(" ,:-"), r)
	})
	ret := make([]T, 0, len(fields))
	for _, n := range fields {
		i, err := strconv.ParseInt(n, 10, strconv.IntSize)
		if err != nil {
			return nil, fmt.Errorf("invalid number %v", n)
		}
		ret = append(ret, T(i))
	}
	return ret, nil
}

func NextLine(sc *bufio.Scanner) (string, error) {
	if !sc.Scan() {
		if err := sc.Err(); err != nil {
			return "", err
		}
		return "", io.EOF
	}
	return sc.Text(), nil
}

func Abs[T Number](a T) T {
	if a < 0 {
		return -a
	}
	return a
}

// Gcd finds the greatest common divisor of two integers
// ref: https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
func Gcd[T constraints.Integer](a, b T) T {
	a, b = Abs(a), Abs(b)
	if a < b {
		a, b = b, a
	}
	for b > 0 {
		a, b = b, a%b
	}
	return a
}

// LcmSeq finds the least common multiple of a sequence of numbers
func LcmSeq[T constraints.Integer](nums []T) T {
	switch len(nums) {
	case 0:
		return 0
	case 1:
		return nums[0]
	case 2:
		a, b := nums[0], nums[1]
		return Abs(a) * Abs(b) / Gcd(a, b)
	default:
		a, b := nums[0], LcmSeq(nums[1:])
		return Abs(a) * Abs(b) / Gcd(a, b)
	}
}

func Sum[T Number](nums ...T) T {
	var sum T
	for _, n := range nums {
		sum += n
	}
	return sum
}
