package day08

import "math"

type Coord3 struct{ X, Y, Z int64 }

func (c *Coord3) Distance(target *Coord3) float64 {
	dx := math.Abs(float64(c.X - target.X))
	dy := math.Abs(float64(c.Y - target.Y))
	dz := math.Abs(float64(c.Z - target.Z))
	return math.Sqrt(dx*dx + dy*dy + dz*dz)
}
