package day18

import (
	"image"
	"image/draw"
)

func drawMap(instructions []Instruction) image.Image {
	// Find min and max coordinate
	cur := image.Pt(0, 0)
	minPt, maxPt := cur, cur
	for _, inst := range instructions {
		vect := vects[inst.direction]
		cur.X += vect[0] * int(inst.distance)
		cur.Y -= vect[1] * int(inst.distance)
		minPt.X = min(minPt.X, cur.X)
		minPt.Y = min(minPt.Y, cur.Y)
		maxPt.X = max(maxPt.X, cur.X)
		maxPt.Y = max(maxPt.Y, cur.Y)
	}

	img := image.NewRGBA(image.Rect(minPt.X, minPt.Y, maxPt.X+1, maxPt.Y+1))
	draw.Draw(img, img.Bounds(), image.White, image.Pt(0, 0), draw.Src)

	cur = image.Pt(0, 0)
	for _, inst := range instructions {
		vect := vects[inst.direction]
		for range inst.distance {
			cur = image.Pt(cur.X+vect[0], cur.Y-vect[1])
			img.Set(cur.X, cur.Y, inst.color)
		}
	}
	return img
}
