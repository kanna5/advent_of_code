package day02

import (
	"fmt"
	"strconv"
	"strings"
)

type Range struct {
	L, R int64
}

func parseRange(input string) (Range, error) {
	parts := strings.Split(input, "-")
	if len(parts) != 2 {
		return Range{}, fmt.Errorf("invalid format: expected 2 integers")
	}
	nums := [2]int64{}
	for i, p := range parts {
		num, err := strconv.ParseInt(p, 10, 64)
		if err != nil {
			return Range{}, err
		}
		nums[i] = num
	}
	return Range{nums[0], nums[1]}, nil
}
