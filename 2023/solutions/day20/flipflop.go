package day20

type FlipFlop struct {
	name string
	on   bool
}

func (f *FlipFlop) Name() string {
	return f.name
}

func (f *FlipFlop) AddInput(name string) {
	// no-op
}

func (f *FlipFlop) Signal(from string, p Pulse) *Pulse {
	if p != PulseLow {
		return nil
	}
	f.on = !f.on
	ret := PulseLow
	if f.on {
		ret = PulseHigh
	}
	return &ret
}

func NewFlipFlop(name string) *FlipFlop {
	return &FlipFlop{name: name}
}

var _ Module = &FlipFlop{}

func IsFlipFlop(v Module) bool {
	_, ok := v.(*FlipFlop)
	return ok
}
