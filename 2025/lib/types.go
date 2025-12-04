package lib

import "golang.org/x/exp/constraints"

type Number interface {
	constraints.Integer | constraints.Float
}

type Set[T comparable] map[T]struct{}

func (s Set[T]) Add(elems ...T) {
	for i := range elems {
		s[elems[i]] = struct{}{}
	}
}

func (s Set[T]) Del(elems ...T) {
	for i := range elems {
		delete(s, elems[i])
	}
}

func (s Set[T]) Has(elem T) bool {
	_, ok := s[elem]
	return ok
}

func NewSet[T comparable](items ...T) Set[T] {
	ret := make(Set[T], len(items))
	for i := range items {
		ret.Add(items[i])
	}
	return ret
}
