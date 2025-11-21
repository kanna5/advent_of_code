package day15

import (
	"fmt"
	"strconv"
	"strings"
)

type Lens struct {
	label       string
	focalLength int
}

type Box []Lens

func (b Box) Put(l Lens) Box {
	for i := range b {
		if b[i].label == l.label {
			b[i] = l
			return b
		}
	}
	return append(b, l)
}

func (b Box) Remove(label string) Box {
	found := false

	for i := range b {
		if !found && b[i].label == label {
			found = true
		}
		if found && i+1 < len(b) {
			b[i] = b[i+1]
		}
	}
	if found {
		return b[:len(b)-1]
	}
	return b
}

func (b Box) Power() int {
	sum := 0
	for i := range b {
		sum += (i + 1) * b[i].focalLength
	}
	return sum
}

type Operation uint8

const (
	Put Operation = iota
	Remove
)

type Instruction struct {
	label       string
	op          Operation
	focalLength int
}

func parseInstruction(i string) (*Instruction, error) {
	parts := strings.Split(i, "=")
	if len(parts) != 2 {
		if len(parts) != 1 || !strings.HasSuffix(parts[0], "-") {
			return nil, fmt.Errorf("invalid instruction format")
		}
		return &Instruction{
			label: parts[0][:len(parts[0])-1],
			op:    Remove,
		}, nil
	}
	focLen, err := strconv.ParseInt(parts[1], 10, 64)
	if err != nil {
		return nil, fmt.Errorf("failed to parse focal length")
	}
	return &Instruction{
		label:       parts[0],
		op:          Put,
		focalLength: int(focLen),
	}, nil
}
