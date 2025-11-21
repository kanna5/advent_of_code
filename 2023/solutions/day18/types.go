package day18

import (
	"encoding/hex"
	"fmt"
	"image/color"
	"slices"
	"strconv"
	"strings"
)

type Direction uint8

const (
	Up Direction = iota
	Right
	Down
	Left
	InvalidDirection Direction = 255
)

var (
	vects = [...][2]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}
)

func parseDirection(s string) Direction {
	if len(s) != 1 {
		return InvalidDirection
	}

	switch s[0] {
	case 'U', '3':
		return Up
	case 'R', '0':
		return Right
	case 'D', '1':
		return Down
	case 'L', '2':
		return Left
	}
	return InvalidDirection
}

type Instruction struct {
	distance  int64
	direction Direction
	color     color.RGBA
}

func parseInstructionText(text string) (Instruction, error) {
	parts := strings.Fields(text)
	if len(parts) != 3 {
		return Instruction{}, fmt.Errorf("invalid number of fields")
	}
	var dir Direction
	if dir = parseDirection(parts[0]); dir == InvalidDirection {
		return Instruction{}, fmt.Errorf("invalid direction %q", parts[0])
	}
	distance, err := strconv.ParseInt(parts[1], 10, 64)
	if err != nil {
		return Instruction{}, fmt.Errorf("invalid distance %q", parts[1])
	}
	if len(parts[2]) != 9 {
		return Instruction{}, fmt.Errorf("invalid color %q", parts[2])
	}
	rgb, err := hex.DecodeString(parts[2][2:8])
	if err != nil {
		return Instruction{}, fmt.Errorf("invalid color %q: %v", parts[2], err)
	}

	return Instruction{
		direction: dir,
		distance:  distance,
		color:     color.RGBA{rgb[0], rgb[1], rgb[2], 255},
	}, nil
}

func parseInstructionBinary(text string) (Instruction, error) {
	bin := []byte(text)
	pos := slices.Index(bin, '#')
	if pos == -1 {
		return Instruction{}, fmt.Errorf("invalid format: no # found")
	}
	if pos+7 > len(bin) {
		return Instruction{}, fmt.Errorf("invalid format: too short")
	}
	decoded, err := hex.DecodeString(string(bin[pos+1 : pos+7]))
	if err != nil {
		return Instruction{}, fmt.Errorf("invalid format: failed to decode hex digits")
	}

	var dist int64
	for _, i := range decoded {
		dist = dist<<8 + int64(i)
	}
	dist >>= 4

	dir := parseDirection(string(bin[pos+6 : pos+7]))
	if dir == InvalidDirection {
		return Instruction{}, fmt.Errorf("invalid direction")
	}
	return Instruction{
		direction: dir,
		distance:  dist,
		// no color
	}, nil
}

type State struct {
	y       int64
	x       int64
	area    int64
	pathLen int64
}

func (s *State) Run(inst Instruction) {
	vect := vects[inst.direction]
	dx := int64(vect[0]) * inst.distance
	dy := int64(vect[1]) * inst.distance

	s.x += dx
	s.y += dy
	s.area += dx * s.y
	s.pathLen += inst.distance
}

func (s *State) Volume() int64 {
	if s.x != 0 || s.y != 0 {
		return 0 // not closed
	}
	ret := s.area
	if ret < 0 {
		ret = -ret
	}
	ret += 1 + s.pathLen/2
	return ret
}
