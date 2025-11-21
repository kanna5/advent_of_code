package day10

import (
	"bufio"
	"fmt"
	"io"
)

type Coord struct {
	x, y int
}

type Direction uint8

const (
	InvalidDirection Direction = iota
	Up
	Right
	Down
	Left
)

func (d Direction) Flip() Direction {
	return (d+1)%4 + 1
}

func (d Direction) Rot(rotDir Direction) Direction {
	switch rotDir {
	case Left:
		return (d+2)%4 + 1
	case Right:
		return d%4 + 1
	}
	return d
}

var directions = map[Direction][2]int{
	Up:    {0, -1},
	Right: {1, 0},
	Down:  {0, 1},
	Left:  {-1, 0},
}

type CellType uint8

const (
	InvalidCell CellType = iota
	Pipe
	Ground
)

type Cell struct {
	type_      CellType
	directions map[Direction]bool // can go these directions when it's a pipe
	distance   int                // distance from the starting point
}

type Map struct {
	w, h  int
	start Coord

	data [][]Cell
}

type MapCursor struct {
	x, y     int
	cell     *Cell
	map_     *Map
	cameFrom Direction
}

func (c *MapCursor) move(dir Direction) *MapCursor {
	newX := c.x + directions[dir][0]
	newY := c.y + directions[dir][1]
	if newX < 0 || newX >= c.map_.w ||
		newY < 0 || newY >= c.map_.h {
		return nil
	}
	return &MapCursor{
		x:        newX,
		y:        newY,
		cell:     &c.map_.data[newY][newX],
		map_:     c.map_,
		cameFrom: dir.Flip(),
	}
}

func (m *Map) cursor(x, y int) *MapCursor {
	return &MapCursor{
		x:    x,
		y:    y,
		cell: &m.data[y][x],
		map_: m,
	}
}

var cellTemplate = map[rune]Cell{
	'|': {Pipe, map[Direction]bool{Up: true, Down: true}, 0},
	'-': {Pipe, map[Direction]bool{Left: true, Right: true}, 0},
	'L': {Pipe, map[Direction]bool{Up: true, Right: true}, 0},
	'J': {Pipe, map[Direction]bool{Up: true, Left: true}, 0},
	'7': {Pipe, map[Direction]bool{Left: true, Down: true}, 0},
	'F': {Pipe, map[Direction]bool{Right: true, Down: true}, 0},
	'S': {Pipe, map[Direction]bool{Up: true, Right: true, Down: true, Left: true}, 0},
	'.': {Ground, nil, 0},
}

func readMap(input io.Reader) (*Map, error) {
	sc := bufio.NewScanner(input)
	ret := Map{}

	for sc.Scan() {
		line := sc.Text()
		if ret.w == 0 {
			ret.w = len(line)
		} else if len(line) != ret.w {
			return nil, fmt.Errorf("got variable line length")
		}
		row := make([]Cell, 0, ret.w)
		for x, r := range line {
			if r == 'S' {
				ret.start.x, ret.start.y = x, ret.h
			}
			c, ok := cellTemplate[r]
			if !ok {
				return nil, fmt.Errorf("invalid cell '%c'", r)
			}
			row = append(row, c)
		}

		ret.data = append(ret.data, row)
		ret.h++
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	return &ret, nil
}
