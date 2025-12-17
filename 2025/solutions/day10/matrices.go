package day10

import (
	"slices"

	"github.com/kanna5/advent_of_code/2025/lib"
)

var accuracyThreshold float64 = 1e-12

func isNonZero(f float64) bool {
	return lib.Abs(f) > accuracyThreshold
}

func isZero(f float64) bool {
	return !isNonZero(f)
}

func cmpRow(a, b []float64) int {
	i := 0
	for i < len(a) && isZero(a[i]) && isZero(b[i]) {
		i++
	}
	if i == len(a) || (isNonZero(a[i]) && isNonZero(b[i])) {
		return 0
	}
	if isZero(a[i]) {
		return 1
	}
	return -1
}

func normRow(row []float64) {
	for i := range row {
		if isZero(row[i]) {
			row[i] = 0
		}
	}

	i := 0
	for i < len(row) && row[i] == 0 {
		i++
	}
	if i == len(row) {
		return
	}
	div := row[i]
	for ; i < len(row); i++ {
		row[i] /= div
	}
}

func subRow(a, b []float64, index int) {
	if a[index] == 0 || b[index] == 0 {
		return
	}
	mul := a[index] / b[index]
	for i := range a {
		a[i] -= b[i] * mul
	}
	a[index] = 0
}

func gauss(m [][]float64) [][]float64 {
	slices.SortStableFunc(m, cmpRow)
	width := len(m[0])

	for i := range len(m) {
		firstNonZero := i
		for firstNonZero < width && m[i][firstNonZero] == 0 {
			firstNonZero++
		}
		if firstNonZero >= width-1 {
			m = m[:i]
			break
		}

		normRow(m[i])
		for j := i + 1; j < len(m); j++ {
			if isNonZero(m[j][firstNonZero]) {
				subRow(m[j], m[i], firstNonZero)
			}
		}
		slices.SortStableFunc(m[i+1:], cmpRow)
	}

	for i := len(m) - 1; i > 0; i-- {
		firstNonZero := i
		for firstNonZero < width && m[i][firstNonZero] == 0 {
			firstNonZero++
		}

		for j := i - 1; j >= 0; j-- {
			if m[j][firstNonZero] != 0 {
				subRow(m[j], m[i], firstNonZero)
			}
		}
	}
	return m
}
