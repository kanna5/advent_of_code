// Solution for https://adventofcode.com/2023/day/8
package day08

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
	"unicode"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

type Node struct {
	id string
	l  string
	r  string
}

type Map struct {
	instructions string
	nodes        map[string]*Node
}

func readInput(input io.Reader) (*Map, error) {
	sc := bufio.NewScanner(input)
	line, err := lib.NextLine(sc)
	if err != nil {
		return nil, err
	}
	instructions := line
	for _, i := range instructions {
		switch i {
		case 'L':
		case 'R':
		default:
			return nil, fmt.Errorf("invalid instruction %v", i)
		}
	}

	_, err = lib.NextLine(sc) // skip empty line
	if err != nil {
		return nil, err
	}

	nodes := make(map[string]*Node, 0)
	for sc.Scan() {
		line = sc.Text()
		parts := strings.FieldsFunc(line, func(r rune) bool {
			return !unicode.IsDigit(r) && !unicode.IsLetter(r)
		})
		if len(parts) != 3 {
			return nil, fmt.Errorf("invalid input %v", line)
		}
		node := Node{
			id: parts[0],
			l:  parts[1],
			r:  parts[2],
		}
		nodes[node.id] = &node
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return &Map{instructions, nodes}, nil
}

func (s *sol) SolvePart1() (string, error) {
	map_, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	var steps int64
	p := map_.nodes["AAA"]
outer:
	for {
		for _, i := range map_.instructions {
			if i == 'L' {
				p = map_.nodes[p.l]
			} else {
				p = map_.nodes[p.r]
			}
			if p == nil {
				return "", fmt.Errorf("missing node")
			}
			steps++
			if p.id == "ZZZ" {
				break outer
			}
		}
	}

	return strconv.FormatInt(steps, 10), nil
}

func findSteps2(map_ *Map, start *Node) (int64, error) {
	var steps int64
	p := start
	for {
		for _, i := range map_.instructions {
			if i == 'L' {
				p = map_.nodes[p.l]
			} else {
				p = map_.nodes[p.r]
			}
			if p == nil {
				return 0, fmt.Errorf("missing node")
			}
			steps++
			if strings.HasSuffix(p.id, "Z") {
				return steps, nil
			}
		}
	}
}

func (s *sol) SolvePart2() (string, error) {
	map_, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	startPos := make([]*Node, 0, len(map_.nodes))
	// look for all nodes ending with A
	for id, node := range map_.nodes {
		if strings.HasSuffix(id, "A") {
			startPos = append(startPos, node)
		}
	}
	allSteps := make([]int64, 0, len(startPos))

	for _, node := range startPos {
		steps, err := findSteps2(map_, node)
		if err != nil {
			return "", err
		}
		allSteps = append(allSteps, steps)
	}

	result := lib.LcmSeq(allSteps)
	return strconv.FormatInt(result, 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[8] = &sol{}
}
