package day25

type NodeID int

type Edge [2]NodeID

func NewEdge(a, b NodeID) Edge {
	if a < b {
		return Edge{a, b}
	}
	return Edge{b, a}
}
