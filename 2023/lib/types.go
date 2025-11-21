package lib

type Set[T comparable] map[T]struct{}

func (s Set[T]) Add(elem T) {
	s[elem] = struct{}{}
}

func (s Set[T]) Del(elem T) {
	delete(s, elem)
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
