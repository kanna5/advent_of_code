package day10

import (
	"fmt"
	"strings"

	"github.com/kanna5/advent_of_code/2025/lib"
)

type Machine struct {
	Diagram    int   // bitmap
	BtnWirings []int // bitmaps
	Joltages   []int
}

func (m *Machine) Len() int {
	return len(m.Joltages)
}

func parseMachine(line string) (*Machine, error) {
	parts := strings.Fields(line)
	if len(parts) < 3 {
		return nil, fmt.Errorf("invalid format")
	}

	diagram, err := parseDiagram(parts[0])
	if err != nil {
		return nil, err
	}

	wirings := make([]int, len(parts)-2)
	for i, s := range parts[1 : len(parts)-1] {
		w, err := parseWiring(s)
		if err != nil {
			return nil, err
		}
		wirings[i] = w
	}

	joltages, err := parseJoltages(parts[len(parts)-1])
	if err != nil {
		return nil, err
	}
	return &Machine{
		Diagram:    diagram,
		BtnWirings: wirings,
		Joltages:   joltages,
	}, nil
}

func parseDiagram(str string) (int, error) {
	if len(str) < 3 || str[0] != '[' || str[len(str)-1] != ']' {
		return 0, fmt.Errorf("invalid diagram format")
	}
	str = str[1 : len(str)-1]
	diagram := 0
	for i := range str {
		switch str[i] {
		case '#':
			diagram += 1 << i
		case '.':
		default:
			return 0, fmt.Errorf("invalid character in diagram: %q", str[i])
		}
	}
	return diagram, nil
}

func parseWiring(str string) (int, error) {
	if len(str) < 3 || str[0] != '(' || str[len(str)-1] != ')' {
		return 0, fmt.Errorf("invalid wiring format %q", str)
	}
	nums, err := lib.StrToIntSlice[int](str[1 : len(str)-1])
	if err != nil {
		return 0, err
	}

	wiring := 0
	for _, num := range nums {
		wiring += 1 << num
	}
	return wiring, nil
}

func parseJoltages(str string) ([]int, error) {
	if len(str) < 3 || str[0] != '{' || str[len(str)-1] != '}' {
		return nil, fmt.Errorf("invalid joltages format %q", str)
	}
	return lib.StrToIntSlice[int](str[1 : len(str)-1])
}
