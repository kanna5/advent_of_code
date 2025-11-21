package day24

type Vec2 struct{ X, Y int64 }
type Vec3 struct{ X, Y, Z int64 }

func (v *Vec3) Vec2() Vec2 { return Vec2{v.X, v.Y} }

type Hailstone struct {
	Pos Vec3
	Vel Vec3
}
