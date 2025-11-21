// Solution for https://adventofcode.com/2023/day/15
package day15

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

func hash(s string) int {
	h := 0
	for _, b := range []byte(s) {
		h = ((h + int(b)) * 17) % 256
	}
	return h
}

func (s *sol) SolvePart1() (string, error) {
	sc := bufio.NewScanner(s.input)
	line, err := lib.NextLine(sc)
	if err != nil {
		return "", err
	}

	sum := 0
	for s := range strings.SplitSeq(line, ",") {
		sum += hash(s)
	}
	return strconv.FormatInt(int64(sum), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	sc := bufio.NewScanner(s.input)
	line, err := lib.NextLine(sc)
	if err != nil {
		return "", err
	}

	boxes := make([]Box, 256)
	for s := range strings.SplitSeq(line, ",") {
		instruction, err := parseInstruction(s)
		if err != nil {
			return "", fmt.Errorf("failed to parse instruction %q: %v", s, err)
		}
		bin := hash(instruction.label)
		switch instruction.op {
		case Put:
			boxes[bin] = boxes[bin].Put(Lens{
				label:       instruction.label,
				focalLength: instruction.focalLength,
			})
		case Remove:
			boxes[bin] = boxes[bin].Remove(instruction.label)
		}
	}

	sum := 0
	for i := range boxes {
		sum += (i + 1) * boxes[i].Power()
	}
	return strconv.FormatInt(int64(sum), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[15] = &sol{}
}
