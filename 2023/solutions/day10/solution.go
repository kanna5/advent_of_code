// Solution for https://adventofcode.com/2023/day/10
package day10

import (
	"fmt"
	"io"
	"maps"
	"slices"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	start := map_.cursor(map_.start.x, map_.start.y)
	cursors := []*MapCursor{}
	// look for directions to proceed
	for _, dir := range []Direction{Up, Right, Down, Left} {
		c := start.move(dir)
		if c == nil {
			continue
		}
		if c.cell.type_ == Pipe && c.cell.directions[dir.Flip()] {
			cursors = append(cursors, c)
			c.cell.distance = 1
		}
	}

	maxDist := 0
	for len(cursors) > 0 {
		cursorsNext := make([]*MapCursor, 0, len(cursors))
		for _, c := range cursors {
			dist := c.cell.distance
			for nextDir, ok := range c.cell.directions {
				if !ok || nextDir == c.cameFrom {
					continue
				}
				// move to next cell
				cNext := c.move(nextDir)
				if cNext == nil {
					// dead end
					continue
				}

				distMin := min(cNext.cell.distance, dist+1)
				if distMin < cNext.cell.distance || distMin == 0 {
					cNext.cell.distance = dist + 1
					cursorsNext = append(cursorsNext, cNext)
				} else {
					// met with the other side
					maxDist = max(maxDist, cNext.cell.distance)
				}
				break
			}
		}
		cursors = cursorsNext
	}

	return strconv.FormatInt(int64(maxDist), 10), nil
}

type Visit struct {
	c       Coord
	in, out Direction
}

func findLoop(map_ *Map, startDir Direction) map[Coord]Visit {
	visits := map[Coord]Visit{
		map_.start: {
			c:   map_.start,
			out: startDir,
		},
	}

	cur := map_.cursor(map_.start.x, map_.start.y)
	nextDir := startDir
	for {
		cur = cur.move(nextDir)
		if cur == nil || !cur.cell.directions[cur.cameFrom] {
			return nil // no loop
		}

		coord := Coord{cur.x, cur.y}
		if v, ok := visits[coord]; ok {
			// already visited. loop found
			v.in = nextDir
			visits[coord] = v
			return visits
		}

		prevDir := nextDir
		nextDir = InvalidDirection
		for d, ok := range cur.cell.directions {
			if ok && d != cur.cameFrom {
				nextDir = d
			}
		}
		if nextDir == InvalidDirection {
			return nil
		}
		visits[coord] = Visit{c: coord, in: prevDir, out: nextDir}
	}
}

func (s *sol) SolvePart2() (string, error) {
	map_, err := readMap(s.input)
	if err != nil {
		return "", err
	}

	// Find the loop
	var loop map[Coord]Visit
	for _, dir := range []Direction{Up, Right, Down, Left} {
		if loop = findLoop(map_, dir); loop != nil {
			break
		}
	}
	if loop == nil {
		return "", fmt.Errorf("no loop found")
	}

	// Gather inside and outside blocks
	sideDirs := [...]Direction{Left, Right}
	sides := [...]lib.Set[Coord]{
		make(lib.Set[Coord], len(loop)),
		make(lib.Set[Coord], len(loop)),
	}
	isOutside := [...]bool{false, false}

	for _, v := range loop {
		cur := map_.cursor(v.c.x, v.c.y)
		for i, dir := range sideDirs {
			for _, neighbor := range [...]*MapCursor{
				cur.move(v.in.Rot(dir)), cur.move(v.out.Rot(dir)), // could be the same
			} {
				if neighbor == nil {
					isOutside[i] = true
					break
				}
				coord := Coord{neighbor.x, neighbor.y}
				if _, ok := loop[coord]; !ok {
					sides[i].Add(coord)
				}
			}
		}
	}

	// Expand both sets until one touches the edge, then the other set is inside
	for side := range sides {
		if isOutside[side] {
			continue
		}
		queue := make([]Coord, 0, len(sides[side]))
		queue = slices.AppendSeq(queue, maps.Keys(sides[side]))
	Loop:
		for i := 0; i < len(queue); i++ {
			cur := map_.cursor(queue[i].x, queue[i].y)
			for _, dir := range []Direction{Up, Right, Down, Left} {
				neighbor := cur.move(dir)
				if neighbor == nil {
					isOutside[side] = true
					break Loop
				}
				coord := Coord{neighbor.x, neighbor.y}
				_, inLoop := loop[coord]
				if !inLoop && !sides[side].Has(coord) {
					queue = append(queue, coord)
					sides[side].Add(coord)
				}
			}
		}
	}

	for side, out := range isOutside {
		if !out {
			return strconv.FormatInt(int64(len(sides[side])), 10), nil
		}
	}
	return "", fmt.Errorf("failed to find the inside")
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[10] = &sol{}
}
