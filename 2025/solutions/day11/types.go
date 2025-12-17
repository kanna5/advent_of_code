package day11

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

type Rack struct {
	Nodes       map[string]int
	Connections [][]int
	You, Out    int
}

func readInput(input io.Reader) (*Rack, error) {
	r := Rack{
		Nodes:       map[string]int{},
		Connections: [][]int{},
		You:         -1,
		Out:         -1,
	}
	names := []string{}

	regOrGetId := func(name string) int {
		id, ok := r.Nodes[name]
		if !ok {
			id = len(names)
			names = append(names, name)
			r.Connections = append(r.Connections, []int{})
			r.Nodes[name] = id
		}
		return id
	}

	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}

		parts := strings.SplitN(line, ": ", 2)
		if len(parts) != 2 || len(parts[0]) == 0 {
			return nil, fmt.Errorf("bad input %q: no enough fields", line)
		}
		from := regOrGetId(parts[0])
		if parts[0] == "you" {
			r.You = from
		}
		for toName := range strings.FieldsSeq(parts[1]) {
			to := regOrGetId(toName)
			if r.Out == -1 && toName == "out" {
				r.Out = to
			}
			r.Connections[from] = append(r.Connections[from], to)
		}
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	if r.You == -1 || r.Out == -1 {
		return nil, fmt.Errorf("missing %q or %q", "you", "out")
	}

	return &r, nil
}
