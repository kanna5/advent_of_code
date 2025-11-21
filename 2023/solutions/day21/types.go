package day21

import (
	"bufio"
	"fmt"
	"io"
)

type Cell struct {
	byte
	isStart bool
}

const (
	Rock  = '#'
	Plot  = '.'
	Start = 'S'
)

func parseCell(b byte) (Cell, error) {
	switch b {
	case Rock:
		return Cell{Rock, false}, nil
	case Plot:
		return Cell{Plot, false}, nil
	case Start:
		return Cell{Plot, true}, nil
	}
	return Cell{}, fmt.Errorf("invalid cell %q", b)
}

type Coord struct {
	x, y int
}

func (c Coord) Neighbors() []Coord {
	return []Coord{
		{c.x + 1, c.y + 0},
		{c.x + 0, c.y + 1},
		{c.x - 1, c.y + 0},
		{c.x + 0, c.y - 1},
	}
}

type Map struct {
	data  [][]Cell
	start Coord
	w, h  int
}

func (m *Map) Contains(c Coord) bool {
	return c.x >= 0 && c.y >= 0 && c.x < m.w && c.y < m.h
}

func wrapNum(n, w int) int {
	r := n % w
	if r < 0 {
		return w + r
	}
	return r
}

func (m *Map) At(c Coord) byte {
	return m.data[wrapNum(c.y, m.h)][wrapNum(c.x, m.w)].byte
}

func readMap(input io.Reader) (*Map, error) {
	sc := bufio.NewScanner(input)
	m := Map{data: [][]Cell{}}
	startFound := false
	for y := 0; sc.Scan(); y++ {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}
		if m.w == 0 {
			m.w = len(line)
		} else if m.w != len(line) {
			return nil, fmt.Errorf("got variable line length")
		}
		row := make([]Cell, m.w)
		for x, b := range line {
			cell, err := parseCell(b)
			if err != nil {
				return nil, err
			}
			if cell.isStart {
				startFound = true
				m.start = Coord{x, y}
			}
			row[x] = cell
		}
		m.data = append(m.data, row)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	if !startFound {
		return nil, fmt.Errorf("start position not found")
	}
	m.h = len(m.data)

	return &m, nil
}
