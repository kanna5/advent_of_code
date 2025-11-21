package day23

import (
	"bufio"
	"fmt"
	"io"

	"github.com/kanna5/advent_of_code/2023/lib"
)

type Direction uint8

const (
	Up Direction = iota
	Right
	Down
	Left
)

var vects = [4][2]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}

type Coord struct{ X, Y int }

type CellType byte

const (
	Forest CellType = '#'
	Path   CellType = '.'
	Slope  CellType = '+'
)

type Cell struct {
	typ CellType
	dir Direction // only used when type is Slope
}

func parseCell(byt byte) (Cell, error) {
	switch byt {
	case '.':
		return Cell{typ: Path}, nil
	case '#':
		return Cell{typ: Forest}, nil
	case '^':
		return Cell{typ: Slope, dir: Up}, nil
	case '>':
		return Cell{typ: Slope, dir: Right}, nil
	case 'v':
		return Cell{typ: Slope, dir: Down}, nil
	case '<':
		return Cell{typ: Slope, dir: Left}, nil
	}
	return Cell{}, fmt.Errorf("invalid cell %q", byt)
}

type Map [][]Cell

func (m Map) W() int {
	if m.H() > 0 {
		return len(m[0])
	}
	return 0
}

func (m Map) H() int { return len(m) }

func (m Map) Contains(c Coord) bool {
	w, h := m.W(), m.H()
	return c.X >= 0 && c.Y >= 0 && c.X < w && c.Y < h
}

func (m Map) WalkableFrom(c Coord, ignoreSlope bool) []Coord {
	if !m.Contains(c) {
		return nil
	}
	cell := m[c.Y][c.X]
	if cell.typ == Forest {
		return nil
	}

	dirs := []Direction{Up, Right, Down, Left}
	if !ignoreSlope && cell.typ == Slope {
		dirs = []Direction{cell.dir}
	}
	ret := make([]Coord, 0, 4)
	for _, d := range dirs {
		nC := Coord{c.X + vects[d][0], c.Y + vects[d][1]}
		if !m.Contains(nC) {
			continue
		}
		nCell := m[nC.Y][nC.X]
		if nCell.typ == Forest ||
			(!ignoreSlope && nCell.typ == Slope && nCell.dir != d) {
			continue
		}
		ret = append(ret, nC)
	}

	return ret
}

func (m Map) ToGraph() *Graph {
	nodes := []Coord{
		{1, 0},                 // start
		{m.W() - 2, m.H() - 1}, // goal
	}
	nodesRev := map[Coord]int{}
	for i, c := range nodes {
		nodesRev[c] = i
	}
	links := [][3]int{} // flat list of [from, to, distance]

	type qElem struct {
		from, dist int
		coord      Coord
	}
	queue := []qElem{{0, 0, nodes[0]}, {1, 0, nodes[1]}}
	cache := make(map[Coord]*qElem, m.W()*m.H()) // visited locations

	for ; len(queue) > 0; queue = queue[1:] {
		cur := &queue[0]
		if cached := cache[cur.coord]; cached != nil {
			if cached.from != cur.from {
				links = append(links, [3]int{cur.from, cached.from, cur.dist + cached.dist})
			}
			continue
		}

		next := m.WalkableFrom(cur.coord, true)
		if len(next) > 2 {
			// Register new node
			var nID int
			if i, ok := nodesRev[cur.coord]; !ok {
				nodes = append(nodes, cur.coord)
				nID = len(nodes) - 1
				nodesRev[cur.coord] = nID
			} else {
				nID = i
			}
			links = append(links, [3]int{cur.from, nID, cur.dist})
			cur = &qElem{nID, 0, cur.coord}
		}

		cache[cur.coord] = cur
		for i := range next {
			queue = append(queue, qElem{cur.from, cur.dist + 1, next[i]})
		}
	}

	ret := Graph{
		Nodes: nodes,
		Links: make([]map[int]int, len(nodes)),
	}
	for i := range ret.Links {
		ret.Links[i] = make(map[int]int, 4)
	}
	for i := range links {
		from, to, distance := links[i][0], links[i][1], links[i][2]
		ret.Links[from][to] = distance
		ret.Links[to][from] = distance
	}
	return &ret
}

type Graph struct {
	Nodes []Coord
	Links []map[int]int
}

func (g *Graph) LongestPath() int {
	var search func(int, int, int, lib.Set[int]) int
	search = func(cur, goal, curLen int, visited lib.Set[int]) int {
		if cur == goal {
			return curLen
		}
		visited.Add(cur)
		defer visited.Del(cur)

		maxLen := -1
		for n, dist := range g.Links[cur] {
			if !visited.Has(n) {
				maxLen = max(maxLen, search(n, goal, curLen+dist, visited))
			}
		}
		return maxLen
	}
	return search(0, 1, 0, lib.NewSet[int]())
}

func readMap(input io.Reader) (Map, error) {
	m := Map{}
	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}

		if len(m) > 0 && len(m[0]) != len(line) {
			return nil, fmt.Errorf("got variable line length")
		}
		row := make([]Cell, len(line))
		for i, byt := range line {
			cell, err := parseCell(byt)
			if err != nil {
				return nil, err
			}
			row[i] = cell
		}
		m = append(m, row)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return m, nil
}
