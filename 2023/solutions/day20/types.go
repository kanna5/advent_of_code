package day20

import (
	"bufio"
	"fmt"
	"io"
	"slices"
	"strings"
)

type Pulse uint8

const (
	PulseLow Pulse = iota
	PulseHigh
)

type Module interface {
	Name() string
	Signal(from string, p Pulse) *Pulse
	AddInput(name string)
}

type ModuleConnection struct {
	Module
	inputs  []string
	outputs []string
}

func (m *ModuleConnection) AddInput(name string) {
	m.Module.AddInput(name)
	if !slices.Contains(m.inputs, name) {
		m.inputs = append(m.inputs, name)
	}
}

type Scene struct {
	modules     map[string]*ModuleConnection
	moduleNames []string
	rx          bool
}

func (s *Scene) PushBtn() (int, int) {
	pulses := [...]int{1, 0} // low, high

	type queueElem struct {
		from   string
		signal Pulse
		target string
	}
	queue := []queueElem{{"button", PulseLow, "broadcaster"}}
	for ; len(queue) > 0; queue = queue[1:] {
		cur := queue[0]
		if cur.target == "rx" && cur.signal == PulseLow {
			s.rx = true
			continue
		}
		tgtMod, ok := s.modules[cur.target]
		if !ok {
			continue
		}
		output := tgtMod.Signal(cur.from, cur.signal)
		if output == nil {
			continue
		}

		pulses[*output] += len(tgtMod.outputs)
		for _, oName := range tgtMod.outputs {
			queue = append(queue, queueElem{cur.target, *output, oName})
		}
	}
	return pulses[0], pulses[1]
}

func (s *Scene) IsActivated() bool {
	return s.rx
}

func readScene(input io.Reader) (*Scene, error) {
	scene := Scene{
		modules:     map[string]*ModuleConnection{},
		moduleNames: []string{},
	}

	sc := bufio.NewScanner(input)
	for sc.Scan() {
		line := sc.Text()
		if len(line) == 0 {
			break
		}

		parts := strings.FieldsFunc(line, func(r rune) bool {
			return slices.Contains([]rune{' ', ','}, r)
		})
		if len(parts) < 3 || parts[1] != "->" {
			return nil, fmt.Errorf("invalid module definition %q", line)
		}
		name := parts[0]
		outputs := parts[2:]

		var module Module
		switch {
		case name == "broadcaster":
			module = &Broadcaster{}
		case name[0] == '%':
			name = name[1:]
			module = NewFlipFlop(name)
		case name[0] == '&':
			name = name[1:]
			module = NewConjunction(name)
		default:
			return nil, fmt.Errorf("invalid module name %q: unknown type", name)
		}

		if _, ok := scene.modules[name]; ok {
			return nil, fmt.Errorf("duplicated module name %q", name)
		}
		scene.moduleNames = append(scene.moduleNames, name)
		scene.modules[name] = &ModuleConnection{Module: module, outputs: outputs}
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	// Set up connections
	for _, name := range scene.moduleNames {
		modConn := scene.modules[name]
		for _, oName := range modConn.outputs {
			if oMod, ok := scene.modules[oName]; ok {
				oMod.AddInput(name)
			}
		}
	}

	return &scene, nil
}
