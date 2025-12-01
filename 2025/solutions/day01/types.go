package day01

type Direction byte

const (
	Left  Direction = 'L'
	Right Direction = 'R'
)

type Instruction struct {
	dir  Direction
	dist int
}

func (i *Instruction) ToInt() int {
	if i.dir == Left {
		return i.dist * -1
	}
	return i.dist
}
