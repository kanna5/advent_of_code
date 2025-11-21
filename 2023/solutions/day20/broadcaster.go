package day20

type Broadcaster struct{}

func (b *Broadcaster) Name() string {
	return "broadcaster"
}

func (b *Broadcaster) AddInput(name string) {}

func (b *Broadcaster) Signal(from string, p Pulse) *Pulse {
	return &p
}

var _ Module = &Broadcaster{}

func IsBroadcaster(v Module) bool {
	_, ok := v.(*Broadcaster)
	return ok
}
