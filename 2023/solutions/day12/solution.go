// Solution for https://adventofcode.com/2023/day/12
package day12

import (
	"bufio"
	"encoding/binary"
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

func isFit(row []State, pos, seq int) bool {
	if len(row) < pos+seq {
		return false
	}
	if len(row) > pos+seq && row[pos+seq] == StateBad {
		return false
	}
	if slices.Contains(row[pos:pos+seq], StateGood) {
		return false
	}
	return true
}

func toCacheKey(row []State, seqs []int) string {
	t := []byte(string(row))
	t = binary.LittleEndian.AppendUint32(t, 0)
	for _, i := range seqs {
		t = binary.LittleEndian.AppendUint32(t, uint32(i))
	}
	return string(t)
}

func findNArrangements(row []State, seqs []int, cache map[string]int64) int64 {
	cacheKey := toCacheKey(row, seqs)
	if cached, ok := cache[cacheKey]; ok {
		return cached
	}

	if len(seqs) == 0 {
		if slices.Contains(row, StateBad) {
			return 0
		}
		return 1
	}

	minSpace := len(seqs) - 1
	for _, v := range seqs {
		minSpace += v
	}
	var ret int64
	for i := range len(row) - minSpace + 1 {
		if isFit(row, i, seqs[0]) {
			nextPos := min(i+seqs[0]+1, len(row))
			ret += findNArrangements(row[nextPos:], seqs[1:], cache)
		}
		// can not move pass a '#'
		if row[i] == StateBad {
			break
		}
	}

	cache[cacheKey] = ret
	return ret
}

func fiveFold(row []State, seqs []int) ([]State, []int) {
	rRow := make([]State, 0, len(row)*5+4)
	rSeqs := make([]int, 0, len(seqs)*5)

	rRow = append(rRow, row...)
	rSeqs = append(rSeqs, seqs...)
	for range 4 {
		rRow = append(rRow, StateUnknown)
		rRow = append(rRow, row...)
		rSeqs = append(rSeqs, seqs...)
	}
	return rRow, rSeqs
}

func (s *sol) solve(unfold bool) (string, error) {
	var sum int64

	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		parts := strings.Fields(line)
		if len(parts) != 2 {
			return "", fmt.Errorf("invalid line %v", line)
		}
		row := []State(parts[0])
		seqs, err := lib.StrToIntSlice[int](parts[1])
		if err != nil {
			return "", err
		}
		if unfold {
			row, seqs = fiveFold(row, seqs)
		}
		sum += findNArrangements(row, seqs, map[string]int64{})
	}
	if err := sc.Err(); err != nil {
		return "", err
	}
	return strconv.FormatInt(int64(sum), 10), nil
}

func (s *sol) SolvePart1() (string, error) {
	return s.solve(false)
}

func (s *sol) SolvePart2() (string, error) {
	return s.solve(true)
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[12] = &sol{}
}
