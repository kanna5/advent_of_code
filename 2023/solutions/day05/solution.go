// Solution for https://adventofcode.com/2023/day/5
package day05

import (
	"bufio"
	"fmt"
	"io"
	"slices"
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

type Range struct {
	L int64
	R int64
}

type MapRange struct {
	srcStart int64
	dstStart int64
	len      int64
}

type Map struct {
	ranges []MapRange
}

// newMap creates a map with ranges sorted by srcStart in ascending order
func newMap(ranges []MapRange) *Map {
	r := slices.Clone(ranges)
	slices.SortFunc(r, func(a, b MapRange) int { return int(a.srcStart - b.srcStart) })
	return &Map{r}
}

func (m *Map) Conv(input int64) int64 {
	for _, r := range m.ranges {
		if input >= r.srcStart && input < r.srcStart+r.len {
			return input - r.srcStart + r.dstStart
		}
	}
	return input
}

func readInput(input io.Reader) ([]int64, []*Map, error) {
	sc := bufio.NewScanner(input)
	maps := make([]*Map, 0)

	if !sc.Scan() {
		return nil, nil, fmt.Errorf("failed to read input")
	}
	line := sc.Text()
	if !strings.HasPrefix(line, "seeds: ") {
		return nil, nil, fmt.Errorf("invalid input %v, expected seeds", line)
	}
	seeds, err := lib.StrToIntSlice[int64](line[7:])
	if err != nil {
		return nil, nil, fmt.Errorf("invalid input %v, %v", line, err)
	}

	var curMapRanges []MapRange = nil
	for sc.Scan() {
		line = sc.Text()
		if len(line) == 0 || strings.HasSuffix(line, " map:") {
			if curMapRanges != nil {
				maps = append(maps, newMap(curMapRanges))
				curMapRanges = nil
			}
			continue
		}
		nums, err := lib.StrToIntSlice[int64](line)
		if err != nil {
			return nil, nil, fmt.Errorf("invalid input %v: %v", line, err)
		}
		if len(nums) != 3 {
			return nil, nil, fmt.Errorf("invalid input %v: not a valid range", line)
		}
		curMapRanges = append(curMapRanges, MapRange{
			srcStart: nums[1],
			dstStart: nums[0],
			len:      nums[2],
		})
	}
	if err = sc.Err(); err != nil {
		return nil, nil, err
	}
	if curMapRanges != nil {
		maps = append(maps, newMap(curMapRanges))
	}
	return seeds, maps, nil
}

func (s *sol) SolvePart1() (string, error) {
	seeds, maps, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	locs := make([]int64, 0, len(seeds))
	for _, s := range seeds {
		for _, m := range maps {
			s = m.Conv(s)
		}
		locs = append(locs, s)
	}
	return strconv.FormatInt(slices.Min(locs), 10), nil
}

func (m *Map) ConvRange(src Range) []Range {
	dstRanges := make([]Range, 0, len(m.ranges)*2+1)

	l := src.L
	for _, range_ := range m.ranges {
		if l >= range_.srcStart+range_.len {
			continue
		}
		if l < range_.srcStart {
			// mismatch, keep as-is
			dstRanges = append(dstRanges, Range{l, range_.srcStart - 1})
			l = range_.srcStart
		}
		r := min(range_.srcStart+range_.len-1, src.R)
		// convert l, r to dst
		dstRanges = append(dstRanges, Range{
			l - range_.srcStart + range_.dstStart,
			r - range_.srcStart + range_.dstStart,
		})
		l = r + 1
		if l > src.R {
			return dstRanges
		}
	}
	dstRanges = append(dstRanges, Range{l, src.R})
	return dstRanges
}

func findMin(ranges []Range, maps []*Map) int64 {
	if len(maps) == 0 {
		return slices.MinFunc(ranges, func(a, b Range) int { return int(a.L - b.L) }).L
	}
	mins := make([]int64, 0, len(ranges))
	for _, r := range ranges {
		rNext := maps[0].ConvRange(r)
		mins = append(mins, findMin(rNext, maps[1:]))
	}
	return slices.Min(mins)
}

func (s *sol) SolvePart2() (string, error) {
	seeds, maps, err := readInput(s.input)
	if err != nil {
		return "", err
	}
	ranges := make([]Range, 0, len(seeds)/2)
	for i := 0; i < len(seeds); i += 2 {
		ranges = append(ranges, Range{seeds[i], seeds[i] + seeds[i+1] - 1})
	}
	minLoc := findMin(ranges, maps)

	return strconv.FormatInt(minLoc, 10), nil
}

func init() {
	solutions.Days[5] = &sol{}
}
