package day12

type Shape struct {
	label string
	w, h  int
	tiles [][]bool
}

func (s *Shape) Vol() int {
	v := 0
	for y := range s.tiles {
		for x := range s.tiles[0] {
			if s.tiles[y][x] {
				v++
			}
		}
	}
	return v
}

type Region struct {
	w, h     int
	presents []int
}
