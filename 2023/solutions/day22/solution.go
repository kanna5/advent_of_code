// Solution for https://adventofcode.com/2023/day/22
package day22

import (
	"bufio"
	"fmt"
	"io"
	"runtime"
	"slices"
	"strconv"
	"strings"
	"sync"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

// dropBricks Drops bricks, and calculate supporting structure
func dropBricks(bricks []BrickSupports) {
	type topBrick struct {
		height int
		brick  *BrickSupports
	}

	slices.SortFunc(bricks, func(a, b BrickSupports) int { return a.Bottom() - b.Bottom() })
	tops := make(map[Coord2]topBrick, len(bricks))

	updateTops := func(b *BrickSupports) {
		for c := range b.Iter() {
			info, ok := tops[c.XY()]
			if !ok || info.height < c.Z {
				tops[c.XY()] = topBrick{
					height: c.Z,
					brick:  b,
				}
			}
		}
	}

	findSupports := func(c Coord2) (int, *BrickSupports) {
		if info, ok := tops[c]; ok {
			return info.height, info.brick
		}
		return 0, nil
	}

	for i := range bricks {
		b := &bricks[i]
		supportedBy := lib.Set[*BrickSupports]{}
		supHeight := 0
		for c := range b.Iter() {
			sH, s := findSupports(c.XY())
			if s == nil {
				continue
			}
			if sH > supHeight {
				supportedBy = lib.NewSet(s)
				supHeight = sH
			} else if sH == supHeight {
				supportedBy.Add(s)
			}
		}
		bottom := b.Bottom()
		if supHeight+1 < bottom {
			dist := bottom - supHeight - 1
			b.Brick[0].Z -= dist
			b.Brick[1].Z -= dist
		}
		for sb := range supportedBy {
			sb.supports.Add(b)
			b.supportedBy = supportedBy
		}
		updateTops(b)
	}
}

func (s *sol) SolvePart1() (string, error) {
	bricks, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	dropBricks(bricks)

	// If every brick a brick supports has more than one support, the brick is
	// replaceable (can be disintegrated)
	cnt := 0
	for i := range bricks {
		supports := bricks[i].supports
		if len(supports) == 0 {
			cnt++
			continue
		}
		replaceable := true
		for sb := range supports {
			if len(sb.supportedBy) == 1 {
				replaceable = false
				break
			}
		}
		if replaceable {
			cnt++
		}
	}

	return strconv.FormatInt(int64(cnt), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	bricks, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	dropBricks(bricks)

	findNFalling := func(node *BrickSupports) int {
		fallen := lib.NewSet(node)
		queue := []*BrickSupports{node}
		for ; len(queue) > 0; queue = queue[1:] {
			cur := queue[0]
			for sb := range cur.supports {
				wouldFall := true
				for spb := range sb.supportedBy {
					if !fallen.Has(spb) {
						wouldFall = false
					}
				}
				if wouldFall {
					queue = append(queue, sb)
					fallen.Add(sb)
				}
			}
		}
		return len(fallen) - 1
	}

	inputCh := make(chan *BrickSupports, 100)
	cnt := 0
	cntL := &sync.Mutex{}

	wg := sync.WaitGroup{}
	for range runtime.NumCPU() {
		wg.Go(func() {
			for b := range inputCh {
				n := findNFalling(b)
				cntL.Lock()
				cnt += n
				cntL.Unlock()
			}
		})
	}
	for i := range bricks {
		inputCh <- &bricks[i]
	}
	close(inputCh)
	wg.Wait()

	return strconv.FormatInt(int64(cnt), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func readInput(input io.Reader) ([]BrickSupports, error) {
	sc := bufio.NewScanner(input)
	bricks := []BrickSupports{}
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		parts := strings.Split(line, "~")
		if len(parts) != 2 {
			return nil, fmt.Errorf("invalid input %q", line)
		}
		a, err := lib.StrToIntSlice[int](parts[0])
		if err != nil {
			return nil, err
		}
		b, err := lib.StrToIntSlice[int](parts[1])
		if err != nil {
			return nil, err
		}

		bricks = append(bricks, BrickSupports{
			Brick:       Brick{{a[0], a[1], a[2]}, {b[0], b[1], b[2]}},
			supports:    lib.Set[*BrickSupports]{},
			supportedBy: lib.Set[*BrickSupports]{},
		})
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return bricks, nil
}

func init() {
	solutions.Days[22] = &sol{}
}
