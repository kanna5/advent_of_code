// Solution for https://adventofcode.com/2025/day/10
package day10

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/lib"
	"github.com/kanna5/advent_of_code/2025/solutions"
)

type sol struct {
	input io.Reader
}

func minPushes1(m *Machine) int {
	ret := math.MaxInt

	for i := range 1 << len(m.BtnWirings) {
		nOnes := 0
		result := 0

		for j := 0; i > 0; i >>= 1 {
			if i|1 == i {
				result ^= m.BtnWirings[j]
				nOnes++
			}
			j += 1
		}

		if result == m.Diagram {
			ret = min(ret, nOnes)
		}
	}
	return ret
}

func (s *sol) SolvePart1() (string, error) {
	machines, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	sum := 0
	for i := range machines {
		sum += minPushes1(machines[i])
	}
	return strconv.FormatInt(int64(sum), 10), nil
}

func toMatrix(m *Machine) [][]float64 {
	ret := make([][]float64, m.Len())
	for i := range ret {
		row := make([]float64, len(m.BtnWirings)+1)
		row[len(row)-1] = float64(m.Joltages[i])

		offset := 1 << i
		for j, wiring := range m.BtnWirings {
			if wiring|offset == wiring {
				row[j] = 1
			}
		}
		ret[i] = row
	}
	return ret
}

func getFreedom(m [][]float64) (
	undetermined map[int][]float64, freeElems []int, known []int,
) {
	undetermined = map[int][]float64{}
	known = make([]int, len(m[0])-1)

	for i, j := 0, 0; i < len(m[0])-1; i++ {
		// Row j, Col i
		if j == len(m) {
			freeElems = append(freeElems, i)
			continue
		}
		if m[j][i] == 0 {
			freeElems = append(freeElems, i)
			continue
		}

		determined := true
		for k := i + 1; k < len(m[j])-1; k++ {
			if isNonZero(m[j][k]) {
				determined = false
				break
			}
		}
		if !determined {
			undetermined[i] = m[j]
		} else {
			known[i] = int(math.Round(m[j][len(m[j])-1]))
		}

		j++
	}
	return
}

func substractJolReq(jolReq []int, wiring int, nPushes int) bool {
	for offset := range jolReq {
		if wiring|(1<<offset) == wiring {
			jolReq[offset] -= nPushes
		}
	}
	for _, j := range jolReq {
		if j < 0 {
			return false
		}
	}
	return true
}

func bruteFindMin(undetermined map[int][]float64, free []int, known []int, machine *Machine) ([]int, error) {
	type Und struct {
		index    int
		equation []float64
	}
	und := make([]Und, 0, len(undetermined))
	for idx := range undetermined {
		und = append(und, Und{idx, undetermined[idx]})
	}

	jolRequired := make([]int, machine.Len())
	copy(jolRequired, machine.Joltages)
	for i, n := range known {
		if !substractJolReq(jolRequired, machine.BtnWirings[i], n) {
			return nil, fmt.Errorf("impossible")
		}
	}

	jolReqBfr := make([]int, len(jolRequired))
	derived := make([]int, len(machine.BtnWirings))
	bingo := func(pushes []int) bool {
		jolReq := jolReqBfr
		copy(jolReq, jolRequired)

		for i := range pushes {
			wiring := machine.BtnWirings[free[i]]
			if !substractJolReq(jolReq, wiring, pushes[i]) {
				return false
			}
			derived[free[i]] = pushes[i]
		}

		for i := range und {
			eq := und[i].equation
			rhs := eq[len(eq)-1]
			for j, freeBtn := range free {
				if isNonZero(eq[freeBtn]) {
					rhs -= eq[freeBtn] * float64(pushes[j])
				}
			}
			rhsRounded := math.Round(rhs)
			if isNonZero(rhsRounded - rhs) {
				return false // rhs is not an integer
			}
			rhsI := int(rhsRounded)
			if rhsI < 0 {
				return false
			}

			wiring := machine.BtnWirings[und[i].index]
			if !substractJolReq(jolReq, wiring, rhsI) {
				return false
			}
			derived[und[i].index] = rhsI
		}

		for _, req := range jolReq {
			if req != 0 {
				return false
			}
		}
		return true
	}

	maxPushes := make([]int, len(free)) // search boundary
	for i := range free {
		p := math.MaxInt
		wiring := machine.BtnWirings[free[i]]
		for j := range machine.Len() {
			if wiring|(1<<j) == wiring {
				p = min(p, jolRequired[j])
			}
		}
		maxPushes[i] = p
	}

	minPushes := math.MaxInt
	minDerived := make([]int, len(derived))

	// Iterate all possible cases
	pushes := make([]int, len(free))
Outer:
	for {
		if bingo(pushes) {
			s := lib.Sum(derived...)
			if s < minPushes {
				minPushes = s
				copy(minDerived, derived)
			}
		}

		pushes[0]++
		for i, p := range pushes { // carry over
			if p <= maxPushes[i] {
				break
			}
			pushes[i] = 0
			if i+1 == len(pushes) {
				break Outer
			}
			pushes[i+1]++
		}
	}
	if minPushes == math.MaxInt {
		return nil, fmt.Errorf("impossible")
	}

	// Combine known with derived
	for i := range minDerived {
		minDerived[i] += known[i]
	}
	return minDerived, nil
}

func (s *sol) SolvePart2() (string, error) {
	machines, err := readInput(s.input)
	if err != nil {
		return "", err
	}
	sum := 0

	for i := range machines {
		m := gauss(toMatrix(machines[i]))

		freedom := max(0, len(m[0])-len(m)-1)
		undetermined, freeElems, known := getFreedom(m)
		if (freedom == 0 && len(undetermined) > 0) ||
			freedom != len(freeElems) {
			return "", fmt.Errorf("sanity check failed")
		}

		if freedom == 0 {
			sum += lib.Sum(known...)
			continue
		}
		result, err := bruteFindMin(undetermined, freeElems, known, machines[i])
		if err != nil {
			return "", fmt.Errorf("failed to solve input line %d: %v", i+1, err)
		}
		sum += lib.Sum(result...)
	}

	return strconv.FormatInt(int64(sum), 10), nil
}

func (s *sol) WithInput(i io.Reader) {
	s.input = i
}

func readInput(input io.Reader) ([]*Machine, error) {
	ret := []*Machine{}
	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		m, err := parseMachine(line)
		if err != nil {
			return nil, fmt.Errorf("failed to parse %q: %v", line, err)
		}
		ret = append(ret, m)
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return ret, nil
}

func init() {
	solutions.Days[10] = &sol{}
}
