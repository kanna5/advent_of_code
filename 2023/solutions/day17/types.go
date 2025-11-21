package day17

import (
	"bufio"
	"fmt"
	"io"
	"slices"
)

type Direction uint8

const (
	Up Direction = iota
	Right
	Down
	Left
)

func (d Direction) TurnLeft() Direction {
	return (d + 3) % 4
}

func (d Direction) TurnRight() Direction {
	return (d + 1) % 4
}

var vects = [...][2]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}

type bestResult struct {
	steps uint8 // steps in a straight line
	loss  int   // accumulated loss, less is better
}

func (b bestResult) noBetterThan(a bestResult) bool {
	return b.steps >= a.steps && b.loss >= a.loss
}

type Cell struct {
	loss uint8
	// Possible best results on each direction
	best [4][]bestResult
}

func (c *Cell) updateBest(dir Direction, steps uint8, loss int) (updated bool) {
	cur := bestResult{steps: steps, loss: loss}
	if c.best[dir] == nil {
		c.best[dir] = []bestResult{cur}
		return true
	}

	bests := c.best[dir]
	if slices.ContainsFunc(bests, cur.noBetterThan) {
		return false
	}

	newBests := make([]bestResult, 0, len(bests)+1)
	for _, b := range bests {
		if b.noBetterThan(cur) {
			continue
		}
		newBests = append(newBests, b)
	}
	c.best[dir] = append(newBests, cur)
	return true
}

func (c *Cell) leastLoss() int {
	ll := -1
	for i := range c.best {
		for _, b := range c.best[i] {
			if ll == -1 {
				ll = b.loss
			}
			ll = min(ll, b.loss)
		}
	}
	return ll
}

type Coordinate struct {
	x int
	y int
}

func (c Coordinate) Move(dir Direction, distance int) Coordinate {
	v := vects[dir]
	return Coordinate{
		x: c.x + v[0]*distance,
		y: c.y + v[1]*distance,
	}
}

type Map [][]Cell

func (m Map) Contains(c Coordinate) bool {
	return c.x >= 0 && c.y >= 0 && len(m) > 0 &&
		c.y < len(m) && c.x < len(m[0])
}

type queueElem struct {
	coord           Coordinate
	dir             Direction
	steps           uint8
	accumulatedLoss int
}

func (m Map) findLeastLoss() int {
	queue := make([]queueElem, 0, 1024)
	queue = append(queue,
		queueElem{Coordinate{1, 0}, Right, 1, 0},
		queueElem{Coordinate{0, 1}, Down, 1, 0},
	)
	for ; len(queue) > 0; queue = queue[1:] {
		cur := &queue[0]
		cell := &m[cur.coord.y][cur.coord.x]
		lDir := cur.dir.TurnLeft()
		rDir := cur.dir.TurnRight()
		loss := cur.accumulatedLoss + int(cell.loss)

		var maybeBetter = []bool{
			cell.updateBest(cur.dir, cur.steps, loss),
			cell.updateBest(lDir, 0, loss),
			cell.updateBest(rDir, 0, loss),
		}
		newElems := []queueElem{
			{cur.coord.Move(cur.dir, 1), cur.dir, cur.steps + 1, loss},
			{cur.coord.Move(lDir, 1), lDir, 1, loss},
			{cur.coord.Move(rDir, 1), rDir, 1, loss},
		}
		for i, elem := range newElems {
			if maybeBetter[i] && elem.steps <= 3 && m.Contains(elem.coord) {
				queue = append(queue, elem)
			}
		}
	}

	return m[len(m)-1][len(m[0])-1].leastLoss()
}

func (m Map) findLeastLoss2() int {
	startPos := Coordinate{0, 0}
	accumulateLoss := func(prev int, pos Coordinate, dir Direction, steps int) int {
		sum := prev
		for range steps {
			if pos = pos.Move(dir, 1); m.Contains(pos) {
				sum += int(m[pos.y][pos.x].loss)
			} else {
				break
			}
		}
		return sum
	}

	queue := make([]queueElem, 0, 1024)
	queue = append(queue,
		queueElem{Coordinate{4, 0}, Right, 4, accumulateLoss(0, startPos, Right, 3)},
		queueElem{Coordinate{0, 4}, Down, 4, accumulateLoss(0, startPos, Down, 3)},
	)
	for ; len(queue) > 0; queue = queue[1:] {
		cur := &queue[0]
		cell := &m[cur.coord.y][cur.coord.x]
		lDir := cur.dir.TurnLeft()
		rDir := cur.dir.TurnRight()
		loss := cur.accumulatedLoss + int(cell.loss)

		if !cell.updateBest(cur.dir, cur.steps, loss) {
			continue
		}

		newElems := []queueElem{
			{cur.coord.Move(cur.dir, 1), cur.dir, cur.steps + 1, loss},
			{cur.coord.Move(lDir, 4), lDir, 4, accumulateLoss(loss, cur.coord, lDir, 3)},
			{cur.coord.Move(rDir, 4), rDir, 4, accumulateLoss(loss, cur.coord, rDir, 3)},
		}
		for _, elem := range newElems {
			if elem.steps <= 10 && m.Contains(elem.coord) {
				queue = append(queue, elem)
			}
		}
	}

	return m[len(m)-1][len(m[0])-1].leastLoss()
}

func readMap(input io.Reader) (Map, error) {
	map_ := Map{}

	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}
		if len(map_) != 0 && len(map_[0]) != len(line) {
			return nil, fmt.Errorf("got variable line length")
		}
		row := make([]Cell, len(line))
		for i := range line {
			row[i] = Cell{loss: uint8(line[i] - '0')}
		}
		map_ = append(map_, row)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}
	return map_, nil
}
