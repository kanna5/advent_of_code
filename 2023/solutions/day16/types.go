package day16

import (
	"bufio"
	"fmt"
	"io"
	"slices"
)

type Direction uint8

const (
	Up    Direction = 1
	Right Direction = 2
	Down  Direction = 4
	Left  Direction = 8
)

func (d Direction) Up() bool {
	return d&Up == Up
}

func (d Direction) Right() bool {
	return d&Right == Right
}

func (d Direction) Down() bool {
	return d&Down == Down
}

func (d Direction) Left() bool {
	return d&Left == Left
}

func (d Direction) Components() []Direction {
	ret := make([]Direction, 0, 4)
	if d.Up() {
		ret = append(ret, Up)
	}
	if d.Right() {
		ret = append(ret, Right)
	}
	if d.Down() {
		ret = append(ret, Down)
	}
	if d.Left() {
		ret = append(ret, Left)
	}
	return ret
}

type CellType byte

const (
	Empty     CellType = '.'
	MirrorF   CellType = '/'
	MirrorB   CellType = '\\'
	SplitterH CellType = '-'
	SplitterV CellType = '|'
)

var validCells = []CellType{Empty, MirrorF, MirrorB, SplitterH, SplitterV}

func (c CellType) React(l Direction) Direction {
	var ret Direction
	switch c {
	case Empty:
		ret |= l

	case MirrorB:
		if l.Up() {
			ret |= Left
		}
		if l.Left() {
			ret |= Up
		}
		if l.Down() {
			ret |= Right
		}
		if l.Right() {
			ret |= Down
		}

	case MirrorF:
		if l.Up() {
			ret |= Right
		}
		if l.Right() {
			ret |= Up
		}
		if l.Down() {
			ret |= Left
		}
		if l.Left() {
			ret |= Down
		}

	case SplitterH:
		if l.Up() || l.Down() {
			ret |= Left | Right
		} else {
			ret |= l
		}

	case SplitterV:
		if l.Left() || l.Right() {
			ret |= Up | Down
		} else {
			ret |= l
		}
	}
	return ret
}

type Cell struct {
	CellType
	lightOut Direction
}

type Map [][]Cell

func readMap(input io.Reader) (Map, error) {
	sc := bufio.NewScanner(input)
	map_ := Map{}

	for sc.Scan() {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}
		if len(map_) > 0 && len(line) != len(map_[0]) {
			return nil, fmt.Errorf("got variable line length")
		}
		row := make([]Cell, 0, len(line))
		for _, b := range line {
			if !slices.Contains(validCells, CellType(b)) {
				return nil, fmt.Errorf("invalid cell type %q", rune(b))
			}
			row = append(row, Cell{CellType: CellType(b), lightOut: 0})
		}
		map_ = append(map_, row)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	return map_, nil
}

type Coordinate struct {
	x int
	y int
}

func (c Coordinate) move(d Direction) Coordinate {
	switch {
	case d.Up():
		return Coordinate{x: c.x, y: c.y - 1}
	case d.Right():
		return Coordinate{x: c.x + 1, y: c.y}
	case d.Down():
		return Coordinate{x: c.x, y: c.y + 1}
	case d.Left():
		return Coordinate{x: c.x - 1, y: c.y}
	}
	return c
}

func (m Map) contains(c Coordinate) bool {
	return c.x >= 0 && c.y >= 0 && len(m) > 0 &&
		c.y < len(m) && c.x < len(m[0])
}

func (m Map) countEnergized() int {
	ret := 0
	for y := range len(m) {
		for x := range len(m[y]) {
			if m[y][x].lightOut != 0 {
				ret++
			}
		}
	}
	return ret
}

func (m Map) clone() Map {
	ret := make(Map, len(m))
	for y := range m {
		row := make([]Cell, len(m[y]))
		copy(row, m[y])
		ret[y] = row
	}
	return ret
}

func (m Map) inputLight(c Coordinate, d Direction) Map {
	rMap := m.clone()

	type queueElem struct {
		Coordinate
		Direction
	}

	queue := []queueElem{{c, d}}
	for ; len(queue) > 0; queue = queue[1:] {
		cur := queue[0]
		cell := &rMap[cur.y][cur.x]
		out := cell.React(cur.Direction)
		combined := out | cell.lightOut
		if combined == cell.lightOut {
			continue
		}
		cell.lightOut = combined
		for _, d := range out.Components() {
			c := cur.move(d)
			if rMap.contains(c) {
				queue = append(queue, queueElem{Coordinate: c, Direction: d})
			}
		}
	}
	return rMap
}
