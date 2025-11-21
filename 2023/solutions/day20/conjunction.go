package day20

type Conjunction struct {
	name        string
	inputs      []string
	inputStates map[string]Pulse
}

func (c *Conjunction) Name() string {
	return c.name
}

func (c *Conjunction) AddInput(name string) {
	if _, ok := c.inputStates[name]; ok {
		return
	}
	c.inputs = append(c.inputs, name)
	c.inputStates[name] = PulseLow
}

func (c *Conjunction) Signal(from string, p Pulse) *Pulse {
	if _, ok := c.inputStates[from]; !ok {
		return nil
	}
	switch p {
	case PulseLow, PulseHigh:
		c.inputStates[from] = p
		ret := PulseLow
		for _, val := range c.inputStates {
			if val != PulseHigh {
				ret = PulseHigh
			}
		}
		return &ret
	}
	return nil
}

func NewConjunction(name string) *Conjunction {
	return &Conjunction{
		name:        name,
		inputs:      []string{},
		inputStates: map[string]Pulse{},
	}
}

var _ Module = &Conjunction{}

func IsConjunction(v Module) bool {
	_, ok := v.(*Conjunction)
	return ok
}
