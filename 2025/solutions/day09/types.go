package day09

import (
	"github.com/kanna5/advent_of_code/2025/lib"
)

type Dir uint8

const (
	Up Dir = iota
	Right
	Down
	Left
	InvalidDirection
)

func (d Dir) TurnRight() Dir { return (d + 1) % 4 }
func (d Dir) TurnLeft() Dir  { return (d + 3) % 4 }
func (d Dir) Flip() Dir      { return (d + 2) % 4 }

type Coord struct {
	X, Y int
}

func (c *Coord) Area(another *Coord) int {
	return (lib.Abs(c.X-another.X) + 1) * (lib.Abs(c.Y-another.Y) + 1)
}

func (c *Coord) LineWith(another *Coord) *Line {
	ret := &Line{Points: [2]Coord{*c, *another}}
	switch {
	case another.X > c.X:
		ret.Dir = Right
	case another.X < c.X:
		ret.Dir = Left
	case another.Y > c.Y:
		ret.Dir = Down
	case another.Y < c.Y:
		ret.Dir = Up
	default:
		ret.Dir = InvalidDirection
	}
	return ret
}

type Line struct {
	Points [2]Coord
	Dir    Dir
}

func (l *Line) Flip() *Line {
	return &Line{
		Points: [2]Coord{l.Points[1], l.Points[0]},
		Dir:    l.Dir.Flip(),
	}
}

func (l *Line) ContainsPoint(p *Coord) bool {
	a, b := &l.Points[0], &l.Points[1]
	return p.X >= min(a.X, b.X) &&
		p.X <= max(a.X, b.X) &&
		p.Y >= min(a.Y, b.Y) &&
		p.Y <= max(a.Y, b.Y)
}

func (l *Line) Len() int {
	a, b := &l.Points[0], &l.Points[1]
	return lib.Abs(a.X-b.X) + lib.Abs(a.Y-b.Y) + 1
}

func (l *Line) Overlays(another *Line) bool {
	p0, p1 := &l.Points[0], &l.Points[1]
	p2, p3 := &another.Points[0], &another.Points[1]
	var amin, amax, bmin, bmax int

	if p0.X == p1.X && p0.X == p2.X && p0.X == p3.X {
		amin, amax = min(p0.Y, p1.Y), max(p0.Y, p1.Y)
		bmin, bmax = min(p2.Y, p3.Y), max(p2.Y, p3.Y)
	} else if p0.Y == p1.Y && p0.Y == p2.Y && p0.Y == p3.Y {
		amin, amax = min(p0.X, p1.X), max(p0.X, p1.X)
		bmin, bmax = min(p2.X, p3.X), max(p2.X, p3.X)
	} else {
		return false
	}
	if amax <= bmin || amin >= bmax {
		return false
	}
	return true
}

func (l *Line) Cuts(another *Line) bool {
	if l.Dir.TurnLeft() != another.Dir {
		return false
	}
	p0, p1 := &l.Points[0], &l.Points[1]
	p2, p3 := &another.Points[0], &another.Points[1]

	var (
		l0Axis0, l0Axis1Min, l0Axis1Max,
		l1Axis0Min, l1Axis0Max, l1Axis1 int
	)
	if p0.X == p1.X {
		l0Axis0, l0Axis1Min, l0Axis1Max = p0.X, min(p0.Y, p1.Y), max(p0.Y, p1.Y)
		l1Axis0Min, l1Axis0Max, l1Axis1 = min(p2.X, p3.X), max(p2.X, p3.X), p2.Y
	} else {
		l0Axis0, l0Axis1Min, l0Axis1Max = p0.Y, min(p0.X, p1.X), max(p0.X, p1.X)
		l1Axis0Min, l1Axis0Max, l1Axis1 = min(p2.Y, p3.Y), max(p2.Y, p3.Y), p2.X
	}

	if l0Axis0 >= l1Axis0Max || l0Axis0 <= l1Axis0Min {
		return false
	}
	if l0Axis1Min >= l1Axis1 || l0Axis1Max <= l1Axis1 {
		return another.ContainsPoint(p0)
	}
	return true
}
