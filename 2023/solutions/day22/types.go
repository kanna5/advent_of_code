package day22

import (
	"iter"

	"github.com/kanna5/advent_of_code/2023/lib"
)

type Coord2 struct{ X, Y int }
type Coord3 struct{ X, Y, Z int }

func (c *Coord3) XY() Coord2 { return Coord2{c.X, c.Y} }

type Brick [2]Coord3

func (b *Brick) Len() int {
	if l := b[0].X - b[1].X; l != 0 {
		return lib.Abs(l) + 1
	}
	if l := b[0].Y - b[1].Y; l != 0 {
		return lib.Abs(l) + 1
	}
	if l := b[0].Z - b[1].Z; l != 0 {
		return lib.Abs(l) + 1
	}
	return 1
}

func (b *Brick) normalize() [3]int {
	dir := 1
	if b[0].X > b[1].X || b[0].Y > b[1].Y || b[0].Z > b[1].Z {
		dir = -1
	}
	if b[0].X != b[1].X {
		return [3]int{dir, 0, 0}
	}
	if b[0].Y != b[1].Y {
		return [3]int{0, dir, 0}
	}
	if b[0].Z != b[1].Z {
		return [3]int{0, 0, dir}
	}
	return [3]int{0, 0, 0}
}

func (b *Brick) Bottom() int {
	return min(b[0].Z, b[1].Z)
}

func (b *Brick) Iter() iter.Seq[Coord3] {
	return func(yield func(v Coord3) bool) {
		vec := b.normalize()
		for i := range b.Len() {
			if !yield(Coord3{
				b[0].X + vec[0]*i,
				b[0].Y + vec[1]*i,
				b[0].Z + vec[2]*i,
			}) {
				return
			}
		}
	}
}

type BrickSupports struct {
	Brick
	supports    lib.Set[*BrickSupports]
	supportedBy lib.Set[*BrickSupports]
}
