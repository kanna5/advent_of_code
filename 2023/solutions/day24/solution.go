// Solution for https://adventofcode.com/2023/day/24
package day24

// Part 2: Find the system of linear equations, then use Gauss–Jordan
// elimination to solve it.
// Note: float64 does not provide enough precision for the calculations.

import (
	"bufio"
	"fmt"
	"io"
	"math/big"
	"os"
	"slices"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func crossInFutureXY(h1, h2 *Hailstone, rangeMin, rangeMax int64) bool {
	getAB := func(h *Hailstone) (float64, float64) {
		p, v := h.Pos.Vec2(), h.Vel.Vec2()
		a := float64(v.Y) / float64(v.X)
		b := float64(p.Y) - (a * float64(p.X))
		return a, b
	}
	a1, b1 := getAB(h1)
	a2, b2 := getAB(h2)

	if a1 == a2 {
		return false
	}

	x := (b2 - b1) / (a1 - a2)
	y := a1*x + b1

	if x < float64(rangeMin) || x > float64(rangeMax) ||
		y < float64(rangeMin) || y > float64(rangeMax) {
		return false
	}

	d1 := (x - float64(h1.Pos.X)) * float64(h1.Vel.X)
	d2 := (x - float64(h2.Pos.X)) * float64(h2.Vel.X)
	return d1 > 0 && d2 > 0
}

func (s *sol) SolvePart1() (string, error) {
	stones, err := readInput(s.input)
	if err != nil {
		return "", err
	}
	rangeMin, rangeMax, err := getArgs(200000000000000, 400000000000000)
	if err != nil {
		return "", err
	}

	cnt := 0
	for i := range len(stones) - 1 {
		for j := i + 1; j < len(stones); j++ {
			if crossInFutureXY(&stones[i], &stones[j], rangeMin, rangeMax) {
				cnt++
			}
		}
	}
	return fmt.Sprint(cnt), nil
}

func toParams(x1, y1, a1, b1, x2, y2, a2, b2 int64) []*big.Float {
	// (b1-b2)x + (a2-a1)y + (y2-y1)a + (x1-x2)b = x1*b1 - y1*a1 - x2*b2 + y2*a2
	return []*big.Float{
		new(big.Float).SetInt64(b1 - b2), new(big.Float).SetInt64(a2 - a1),
		new(big.Float).SetInt64(y2 - y1), new(big.Float).SetInt64(x1 - x2),
		new(big.Float).SetInt64(x1*b1 - y1*a1 - x2*b2 + y2*a2),
	}
}

func calc(m [][]*big.Float) ([]*big.Float, bool) {
	getNZeros := func(a []*big.Float) int {
		i := 0
		for ; i < len(a) && a[i].Sign() == 0; i++ {
		}
		return i
	}
	cmp := func(a, b []*big.Float) int { return getNZeros(a) - getNZeros(b) }
	normalize := func(a []*big.Float) {
		i := getNZeros(a)
		if i < len(a) {
			first := new(big.Float).Copy(a[i])
			for j := i; j < len(a); j++ {
				a[j].Quo(a[j], first)
			}
		}
	}
	sub := func(a, b []*big.Float, i int) {
		if b[i].Sign() == 0 {
			return
		}
		mul := new(big.Float).Quo(a[i], b[i])
		for i := range a {
			a[i].Sub(a[i], new(big.Float).Mul(b[i], mul))
		}
	}

	for i := range 4 {
		slices.SortStableFunc(m[i:], cmp)
		if m[i][i].Sign() == 0 {
			return nil, false // no unique solution
		}
		normalize(m[i])
		for j := i + 1; j < 4; j++ {
			sub(m[j], m[i], i)
		}
	}
	// Back substitution
	for i := 1; i < 4; i++ {
		for j := 0; j < i; j++ {
			sub(m[j], m[i], i)
		}
	}
	return []*big.Float{m[0][4], m[1][4], m[2][4], m[3][4]}, true
}

func (s *sol) SolvePart2() (string, error) {
	stones, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	// solve x, y
	matrix := [][]*big.Float{}
	s1 := &stones[0]
	for i := 1; i <= 4; i++ {
		s2 := &stones[i]
		matrix = append(matrix, toParams(
			s1.Pos.X, s1.Pos.Y, s1.Vel.X, s1.Vel.Y,
			s2.Pos.X, s2.Pos.Y, s2.Vel.X, s2.Vel.Y,
		))
	}
	ans1, ok1 := calc(matrix)

	// solve x, z
	matrix = [][]*big.Float{}
	for i := 1; i <= 4; i++ {
		s2 := &stones[i]
		matrix = append(matrix, toParams(
			s1.Pos.X, s1.Pos.Z, s1.Vel.X, s1.Vel.Z,
			s2.Pos.X, s2.Pos.Z, s2.Vel.X, s2.Vel.Z,
		))
	}
	ans2, ok2 := calc(matrix)
	if ok1 && ok2 {
		num := new(big.Float).Add(ans1[0], ans1[1])
		num.Add(num, ans2[1])
		numI, _ := num.Int64()
		return strconv.FormatInt(numI, 10), nil
	}
	return "", fmt.Errorf("could not find an solution with chosen hailstones")
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func readInput(input io.Reader) ([]Hailstone, error) {
	ret := []Hailstone{}
	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		parts := strings.Split(line, "@")
		if len(parts) != 2 {
			return nil, fmt.Errorf("invalid format: %q", line)
		}
		pos, err := lib.StrToIntSlice[int64](parts[0])
		if err != nil {
			return nil, fmt.Errorf("cannot parse numbers in %q, %v", line, err)
		}
		vel, err := lib.StrToIntSlice[int64](parts[1])
		if err != nil {
			return nil, fmt.Errorf("cannot parse numbers in %q, %v", line, err)
		}
		ret = append(ret, Hailstone{
			Pos: Vec3{pos[0], pos[1], pos[2]},
			Vel: Vec3{vel[0], vel[1], vel[2]},
		})
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return ret, nil
}

func getArgs(defaultMin, defaultMax int64) (rangeMin, rangeMax int64, err error) {
	rangeMin, rangeMax = defaultMin, defaultMax
	if minT := os.Getenv("RANGE_MIN"); len(minT) != 0 {
		n, err := strconv.ParseInt(minT, 10, 64)
		if err != nil {
			return 0, 0, fmt.Errorf("failed to parse environment variable RANGE_MIN: %v", err)
		}
		rangeMin = n
	}
	if maxT := os.Getenv("RANGE_MAX"); len(maxT) != 0 {
		n, err := strconv.ParseInt(maxT, 10, 64)
		if err != nil {
			return 0, 0, fmt.Errorf("failed to parse environment variable RANGE_MAX: %v", err)
		}
		rangeMax = n
	}
	return
}

func init() {
	solutions.Days[24] = &sol{}
}
