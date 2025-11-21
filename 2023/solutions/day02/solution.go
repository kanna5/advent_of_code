// Solution for https://adventofcode.com/2023/day/2
package day02

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func parseGame(game string, targetRGB [3]int64) (gameId int64, possible bool, power int64) {
	var maxR, maxG, maxB int64
	possible = true

	parts := strings.Split(game, ": ")
	if len(parts) != 2 {
		return 0, false, 0
	}
	gameIdParts := strings.Fields(parts[0])
	if len(gameIdParts) != 2 {
		return 0, false, 0
	}
	gameId, err := strconv.ParseInt(gameIdParts[1], 10, 64)
	if err != nil {
		return 0, false, 0
	}
	for iter := range strings.SplitSeq(parts[1], "; ") {
		for color := range strings.SplitSeq(iter, ", ") {
			colorParts := strings.Fields(color)
			if len(colorParts) != 2 {
				return 0, false, 0
			}
			count, colorName := colorParts[0], colorParts[1]
			cnt, err := strconv.ParseInt(count, 10, 64)
			if err != nil {
				return 0, false, 0
			}
			var tgt int64
			switch colorName {
			case "red":
				tgt = targetRGB[0]
				maxR = max(cnt, maxR)
			case "green":
				tgt = targetRGB[1]
				maxG = max(cnt, maxG)
			case "blue":
				tgt = targetRGB[2]
				maxB = max(cnt, maxB)
			default:
				return 0, false, 0
			}
			if cnt > tgt {
				possible = false
			}
		}
	}
	power = maxR * maxG * maxB
	return
}

func (s *sol) SolvePart1() (string, error) {
	var sum int64 = 0
	sc := bufio.NewScanner(s.input)
	target := [3]int64{12, 13, 14}
	for sc.Scan() {
		line := sc.Text()
		if gameId, possible, _ := parseGame(line, target); possible {
			sum += gameId
		} else if gameId == 0 {
			log.Printf("failed to parse input: %v", line)
		}
	}
	return fmt.Sprintf("%d", sum), nil
}

func (s *sol) SolvePart2() (string, error) {
	var sum int64 = 0
	sc := bufio.NewScanner(s.input)
	target := [3]int64{0, 0, 0} // noop
	for sc.Scan() {
		line := sc.Text()
		if gameId, _, power := parseGame(line, target); gameId != 0 {
			sum += power
		} else {
			log.Printf("failed to parse input: %v", line)
		}
	}
	return fmt.Sprintf("%d", sum), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[2] = &sol{}
}
