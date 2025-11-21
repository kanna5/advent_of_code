// Solution for https://adventofcode.com/2023/day/18
package day18

// Ref: https://en.wikipedia.org/wiki/Shoelace_formula

import (
	"bufio"
	"fmt"
	"image/png"
	"io"
	"log"
	"os"
	"strconv"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

type parseFunc func(string) (Instruction, error)

func (s *sol) readInstructions(parse parseFunc) ([]Instruction, error) {
	ret := []Instruction{}
	sc := bufio.NewScanner(s.input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		inst, err := parse(line)
		if err != nil {
			return nil, fmt.Errorf("parse %q: %v", line, err)
		}
		ret = append(ret, inst)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return ret, nil
}

func (s *sol) SolvePart1() (string, error) {
	instructions, err := s.readInstructions(parseInstructionText)
	if err != nil {
		return "", err
	}

	if os.Getenv("DRAW") == "1" {
		img := drawMap(instructions)
		imgFileName := "day18.png"
		imgFile, err := os.OpenFile(imgFileName, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
		if err != nil {
			return "", err
		}
		defer func() { _ = imgFile.Close() }()
		if err := png.Encode(imgFile, img); err != nil {
			return "", err
		}
		log.Printf("Image saved to %q", imgFileName)
	}

	state := State{}
	for _, inst := range instructions {
		state.Run(inst)
	}
	return strconv.FormatInt(state.Volume(), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	instructions, err := s.readInstructions(parseInstructionBinary)
	if err != nil {
		return "", err
	}

	state := State{}
	for _, inst := range instructions {
		state.Run(inst)
	}
	return strconv.FormatInt(state.Volume(), 10), nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[18] = &sol{}
}
