// Solution for https://adventofcode.com/2023/day/3
package day03

import (
	"bufio"
	"fmt"
	"io"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

type ObjType uint8

const (
	TypeNumber ObjType = iota
	TypeSymbol
)

type Object struct {
	typ  ObjType
	num  int64
	sym  rune
	x, y int
}

func newNumber(x, y int, firstDigit rune) *Object {
	return &Object{
		x:   x,
		y:   y,
		typ: TypeNumber,
		num: int64(firstDigit - '0'),
	}
}

func newSymbol(x, y int, sym rune) *Object {
	return &Object{
		x:   x,
		y:   y,
		typ: TypeSymbol,
		sym: sym,
	}
}

type Map struct {
	w       int
	h       int
	data    [][]*Object
	symbols []*Object
	numbers []*Object
}

func (o *Object) appendDigit(digit rune) {
	o.num = o.num*10 + int64(digit-'0')
}

func readInput(r io.Reader) (*Map, error) {
	m := Map{
		data:    make([][]*Object, 0, 1024),
		symbols: make([]*Object, 0, 1024),
		numbers: make([]*Object, 0, 1024),
	}
	sc := bufio.NewScanner(r)
	var curObj *Object = nil

	for sc.Scan() {
		line := []rune(sc.Text())
		width := len(line)
		if width == 0 {
			continue
		}
		if m.w == 0 {
			m.w = width
		} else if m.w != width {
			return nil, fmt.Errorf("got variable width rows")
		}

		m.data = append(m.data, make([]*Object, width))
		curObj = nil
		for x, sym := range line {
			switch {
			case sym >= '0' && sym <= '9':
				if curObj != nil && curObj.typ == TypeNumber {
					curObj.appendDigit(sym)
				} else {
					curObj = newNumber(x, m.h, sym)
					m.numbers = append(m.numbers, curObj)
				}
			case sym == '.':
				curObj = nil
			default:
				curObj = newSymbol(x, m.h, sym)
				m.symbols = append(m.symbols, curObj)
			}
			m.data[m.h][x] = curObj
		}
		m.h += 1
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return &m, nil
}

func (s *sol) SolvePart1() (string, error) {
	sch, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	var numbersIdx = make(lib.Set[*Object], len(sch.numbers))
	for _, s := range sch.symbols {
		for x := s.x - 1; x <= s.x+1; x += 1 {
			for y := s.y - 1; y <= s.y+1; y += 1 {
				if x < 0 || x >= sch.w || y < 0 || y >= sch.h {
					continue
				}
				cur := sch.data[y][x]
				if cur != nil && cur.typ == TypeNumber {
					numbersIdx.Add(cur)
				}
			}
		}
	}
	var sum int64
	for s := range numbersIdx {
		sum += s.num
	}
	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	sch, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	var sum int64
	for _, s := range sch.symbols {
		if s.sym != '*' {
			continue
		}
		nums := make(lib.Set[*Object], 9)
		for x := s.x - 1; x <= s.x+1; x += 1 {
			for y := s.y - 1; y <= s.y+1; y += 1 {
				if x < 0 || x >= sch.w || y < 0 || y >= sch.h {
					continue
				}
				cur := sch.data[y][x]
				if cur != nil && cur.typ == TypeNumber {
					nums.Add(cur)
				}
			}
		}
		if len(nums) == 2 {
			var ratio int64 = 1
			for n := range nums {
				ratio *= n.num
			}
			sum += ratio
		}
	}
	return strconv.FormatInt(sum, 10), nil
}

func init() {
	solutions.Days[3] = &sol{}
}
