// Solution for https://adventofcode.com/2023/day/19
package day19

import (
	"bufio"
	"fmt"
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) readInput() (map[string]*Workflow, []*Part, error) {
	sc := bufio.NewScanner(s.input)

	workflows := map[string]*Workflow{}
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		wf, err := parseWorkflow(line)
		if err != nil {
			return nil, nil, fmt.Errorf("invalid workflow %q: %v", line, err)
		}
		workflows[wf.name] = wf
	}
	if err := sc.Err(); err != nil {
		return nil, nil, err
	}

	parts := []*Part{}
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		pt, err := parsePart(line)
		if err != nil {
			return nil, nil, fmt.Errorf("invalid part %q: %v", line, err)
		}
		parts = append(parts, pt)
	}
	if err := sc.Err(); err != nil {
		return nil, nil, err
	}

	return workflows, parts, nil
}

func (s *sol) SolvePart1() (string, error) {
	workflows, parts, err := s.readInput()
	if err != nil {
		return "", err
	}

	sum := 0
	for _, part := range parts {
		wf, ok := workflows["in"]
		if !ok {
			return "", fmt.Errorf("no workflow with name \"in\"")
		}
		for {
			next := wf.Eval(part)
			if next == TgtAccept {
				sum += part.Sum()
				break
			}
			if next == TgtReject {
				break
			}
			if wf, ok = workflows[next]; !ok {
				break
			}
		}
	}

	return strconv.FormatInt(int64(sum), 10), nil
}

func findCombinations(
	wfName string, wfs map[string]*Workflow, constraint Constraint, walked lib.Set[string],
) int64 {
	if wfName == TgtReject || walked.Has(wfName) || !constraint.valid {
		return 0
	}
	if wfName == TgtAccept {
		return constraint.Combinations()
	}
	wf, ok := wfs[wfName]
	if !ok {
		return 0
	}

	walked.Add(wfName)
	defer walked.Del(wfName)

	var sum int64
	for _, rule := range wf.rules {
		// If rule matches (recurse)
		nextConstraint := constraint
		nextConstraint.Apply(&rule)
		sum += findCombinations(rule.destination, wfs, nextConstraint, walked)

		// If rule didn't match
		constraint.Apply(rule.Reverse())
	}
	return sum
}

func (s *sol) SolvePart2() (string, error) {
	workflows, _, err := s.readInput()
	if err != nil {
		return "", err
	}

	sum := findCombinations("in", workflows, *NewConstraint(), lib.Set[string]{})
	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[19] = &sol{}
}
