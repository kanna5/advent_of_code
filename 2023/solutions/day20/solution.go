// Solution for https://adventofcode.com/2023/day/20
package day20

// Note: The key to solving part 2 is to analyze the input structure.
//
// This solution includes code to draw a GraphViz diagram for inspection
// (activated with env DRAW=1)

import (
	"fmt"
	"io"
	"log"
	"os"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/lib"
	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func (s *sol) SolvePart1() (string, error) {
	sc, err := readScene(s.input)
	if err != nil {
		return "", err
	}

	high, low := 0, 0
	for range 1000 {
		tH, tL := sc.PushBtn()
		high += tH
		low += tL
	}

	return strconv.FormatInt(int64(high)*int64(low), 10), nil
}

func decodeBinaryCounter(sc *Scene, entry string) (int64, error) {
	var num int64

	cur := entry
	for bit := 0; ; bit++ {
		curMod, ok := sc.modules[cur]
		if !ok || !IsFlipFlop(curMod.Module) {
			return 0, fmt.Errorf("invalid module %q", cur)
		}
		// Loosely validate the structure...
		if len(curMod.inputs) > 2 || len(curMod.inputs) == 0 ||
			len(curMod.outputs) > 2 || len(curMod.outputs) == 0 {
			return 0, fmt.Errorf("invalid structure at module %#v", curMod)
		}
		// If a module has output to a conjunction, it must be 1; otherwise, 0.
		// If a module has no output to a flip-flop, it is the last one.
		conjunction := ""
		next := ""
		for _, oName := range curMod.outputs {
			oMod, ok := sc.modules[oName]
			if !ok {
				return 0, fmt.Errorf("invalid structure at module %q: unknown output %q", cur, oName)
			}
			switch {
			case IsConjunction(oMod.Module):
				if conjunction != "" {
					return 0, fmt.Errorf("invalid structure at module %q: more than one conjunction", cur)
				}
				conjunction = oName
			case IsFlipFlop(oMod.Module):
				if next != "" {
					return 0, fmt.Errorf("invalid structure at module %q: more than one flip-flop", cur)
				}
				next = oName
			default:
				return 0, fmt.Errorf("invalid structure at module %q: output %q has invalid type", cur, oName)
			}
		}
		if conjunction != "" {
			num += 1 << bit
		}
		if next == "" {
			break
		}
		cur = next
	}
	return num, nil
}

func (s *sol) SolvePart2() (string, error) {
	sc, err := readScene(s.input)
	if err != nil {
		return "", err
	}

	if os.Getenv("DRAW") == "1" {
		diag := drawDiagram(sc)
		diagFile, err := os.OpenFile("day20.dot", os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
		if err != nil {
			return "", fmt.Errorf("failed to write diagram: %v", err)
		}
		defer func() { _ = diagFile.Close() }()
		if _, err = diagFile.WriteString(diag); err != nil {
			return "", fmt.Errorf("failed to write diagram: %v", err)
		}
		log.Println(diagramMsg)
	}

	numbers := make([]int64, 0, 4)
	broadcaster, ok := sc.modules["broadcaster"]
	if !ok {
		return "", fmt.Errorf("no broadcaster node")
	}
	for _, o := range broadcaster.outputs {
		num, err := decodeBinaryCounter(sc, o)
		if err != nil {
			return "", fmt.Errorf("failed to decode binary counter at entry %q: %v", o, err)
		}
		numbers = append(numbers, num)
	}

	return strconv.FormatInt(lib.LcmSeq(numbers), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[20] = &sol{}
}
