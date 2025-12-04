package day04

import (
	"bufio"
	"fmt"
	"io"
	"iter"
)

type CellType byte

const (
	Ground    CellType = '.'
	PaperRoll CellType = '@'
)

type Cell struct {
	typ       CellType
	neighbors int
}

type Coord struct{ X, Y int }

type Map struct {
	data [][]Cell
	w, h int
}

func (m *Map) Contains(c Coord) bool {
	return c.X >= 0 && c.X < m.w && c.Y >= 0 && c.Y < m.h
}

func (m *Map) NeighborsSeq(c Coord) iter.Seq[Coord] {
	return func(yield func(Coord) bool) {
		for tx := range 3 {
			for ty := range 3 {
				if tx == 1 && ty == 1 {
					continue
				}
				tc := Coord{c.X - 1 + tx, c.Y - 1 + ty}
				if !m.Contains(tc) {
					continue
				}
				if !yield(tc) {
					return
				}
			}
		}
	}
}

func (m *Map) Iter() iter.Seq[Coord] {
	return func(yield func(Coord) bool) {
		for x := range m.w {
			for y := range m.h {
				if !yield(Coord{x, y}) {
					return
				}
			}
		}
	}
}

func (m *Map) At(c Coord) *Cell {
	return &m.data[c.Y][c.X]
}

func readMap(input io.Reader) (*Map, error) {
	m := Map{}
	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}
		if m.w == 0 {
			m.w = len(line)
		} else if len(line) != m.w {
			return nil, fmt.Errorf("invalid input: got variable line length")
		}
		row := make([]Cell, m.w)
		for i := range line {
			switch CellType(line[i]) {
			case Ground, PaperRoll:
				row[i] = Cell{typ: CellType(line[i])}
			default:
				return nil, fmt.Errorf("invalid cell type %q", line[i])
			}
		}
		m.data = append(m.data, row)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	m.h = len(m.data)

	// Calculate number of neighbors
	for c := range m.Iter() {
		if m.At(c).typ == PaperRoll {
			for cc := range m.NeighborsSeq(c) {
				cell := m.At(cc)
				if cell.typ == PaperRoll {
					cell.neighbors++
				}
			}
		}
	}
	return &m, nil
}
