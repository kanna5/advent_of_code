package day14

import (
	"bufio"
	"fmt"
	"io"
	"slices"
	"strings"
)

type Cell byte

const (
	RoundedRock Cell = 'O'
	CubeRock    Cell = '#'
	Empty       Cell = '.'
)

type Map [][]Cell

func (m Map) Width() int {
	if len(m) == 0 {
		return 0
	}
	return len(m[0])
}

func (m Map) Height() int {
	return len(m)
}

func (m Map) TiltNorth() {
	for x := range m.Width() {
		fallSpot := 0
		for y := range m.Height() {
			cell := m[y][x]
			if cell == CubeRock {
				fallSpot = y + 1
				continue
			}
			if cell == RoundedRock {
				if y == fallSpot {
					fallSpot++
					continue
				}
				m[y][x] = Empty
				m[fallSpot][x] = RoundedRock
				fallSpot++
			}
		}
	}
}

func (m Map) TiltSouth() {
	for x := range m.Width() {
		fallSpot := m.Height() - 1
		for y := m.Height() - 1; y >= 0; y-- {
			cell := m[y][x]
			if cell == CubeRock {
				fallSpot = y - 1
				continue
			}
			if cell == RoundedRock {
				if y == fallSpot {
					fallSpot--
					continue
				}
				m[y][x] = Empty
				m[fallSpot][x] = RoundedRock
				fallSpot--
			}
		}
	}
}

func (m Map) TiltWest() {
	for y := range m.Height() {
		fallSpot := 0
		for x := range m.Width() {
			cell := m[y][x]
			if cell == CubeRock {
				fallSpot = x + 1
				continue
			}
			if cell == RoundedRock {
				if x == fallSpot {
					fallSpot++
					continue
				}
				m[y][x] = Empty
				m[y][fallSpot] = RoundedRock
				fallSpot++
			}
		}
	}
}

func (m Map) TiltEast() {
	for y := range m.Height() {
		fallSpot := m.Width() - 1
		for x := m.Width() - 1; x >= 0; x-- {
			cell := m[y][x]
			if cell == CubeRock {
				fallSpot = x - 1
				continue
			}
			if cell == RoundedRock {
				if x == fallSpot {
					fallSpot--
					continue
				}
				m[y][x] = Empty
				m[y][fallSpot] = RoundedRock
				fallSpot--
			}
		}
	}
}

func (m Map) Load() int {
	ret := 0
	height := m.Height()
	for i := range m {
		rocks := 0
		for _, c := range m[i] {
			if c == RoundedRock {
				rocks++
			}
		}
		ret += rocks * (height - i)
	}
	return ret
}

func (m Map) String() string {
	var buf strings.Builder
	for i := range m {
		buf.WriteString(string(m[i]))
		buf.WriteByte('\n')
	}
	return buf.String()
}

func readMap(input io.Reader) (Map, error) {
	var m Map
	validCells := []Cell{RoundedRock, CubeRock, Empty}

	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := []Cell(sc.Text())
		if len(line) == 0 {
			break
		}
		for _, b := range line {
			if !slices.Contains(validCells, b) {
				return nil, fmt.Errorf("invalid cell %c on line %v", b, string(line))
			}
		}
		if len(m) > 0 && len(line) != len(m[0]) {
			return nil, fmt.Errorf("got variable line length")
		}
		m = append(m, line)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	return m, nil
}
