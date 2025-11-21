// Solution for https://adventofcode.com/2023/day/25
package day25

// Minimum cut problem: https://en.wikipedia.org/wiki/Minimum_cut
// Using Karger's algorithm (randomly merging nodes until only 2 are left), the
// solution should be found within a few hundred iterations.

import (
	"bufio"
	"fmt"
	"io"
	"math/rand"
	"runtime"
	"strconv"
	"strings"
	"sync"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type sol struct {
	input io.Reader
}

func findCut(nNodes int, edges []Edge, rng *rand.Rand) (bool, int) {
	nodeSize := make(map[NodeID]int, nNodes)
	for i := range nNodes {
		nodeSize[NodeID(i+1)] = 1
	}
	buf := make([]Edge, len(edges))
	copy(buf, edges)
	edges = buf
	buf = make([]Edge, len(edges))

	for len(nodeSize) > 2 {
		edge := &edges[rng.Intn(len(edges))]
		a, b := edge[0], edge[1]
		// Merge nodes (b into a)
		nodeSize[a] += nodeSize[b]
		delete(nodeSize, b)

		// Update edges
		newEdges := buf[:0]
		for _, e := range edges {
			for i := range e {
				if e[i] == b {
					e[i] = a
				}
			}
			if e[0] != e[1] {
				newEdges = append(newEdges, NewEdge(e[0], e[1]))
			}
		}
		edges, buf = newEdges, edges // reuse buffers to avoid allocation.
	}
	if len(edges) != 3 {
		return false, 0
	}
	ret := 1
	for _, sz := range nodeSize {
		ret *= sz
	}
	return true, ret
}

func (s *sol) SolvePart1() (string, error) {
	nodes, edges, err := readInput(s.input)
	if err != nil {
		return "", err
	}

	// Parallel search
	ans := 0
	ansLock := &sync.Mutex{}
	doneCh := make(chan struct{})

	worker := func(seed int64) func() {
		return func() {
			rng := rand.New(rand.NewSource(seed + 42069))
			for {
				select {
				case <-doneCh:
					return
				default:
				}

				if found, mul := findCut(len(nodes), edges, rng); found {
					ansLock.Lock()
					defer ansLock.Unlock()
					if ans == 0 {
						ans = mul
						close(doneCh)
					}
					return
				}
			}
		}
	}

	wg := sync.WaitGroup{}
	for i := range runtime.NumCPU() {
		wg.Go(worker(int64(i)))
	}
	wg.Wait()

	return strconv.FormatInt(int64(ans), 10), nil
}

func (s *sol) SolvePart2() (string, error) {
	return "👉🔴 🎉❄️🎄⭐️😊", nil
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[25] = &sol{}
}

func readInput(input io.Reader) ([]string, []Edge, error) {
	nodes := []string{}
	nodeIdx := map[string]NodeID{}
	getId := func(name string) NodeID {
		if id, ok := nodeIdx[name]; ok {
			return id
		}
		nodes = append(nodes, name)
		id := NodeID(len(nodes))
		nodeIdx[name] = id
		return id
	}
	edges := []Edge{}

	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}
		parts := strings.Split(line, ":")
		if len(parts) != 2 {
			return nil, nil, fmt.Errorf("invalid line %q: invalid format", line)
		}
		lhs := getId(parts[0])
		for name := range strings.FieldsSeq(parts[1]) {
			rhs := getId(name)
			edges = append(edges, NewEdge(lhs, rhs))
		}
	}
	if err := sc.Err(); err != nil {
		return nil, nil, err
	}

	return nodes, edges, nil
}
