package day19

import (
	"fmt"
	"strconv"
	"strings"
)

const (
	TgtAccept  = "A"
	TgtReject  = "R"
	TgtInvalid = ""
)

func Xmas(b byte) int {
	switch b {
	case 'x':
		return 0
	case 'm':
		return 1
	case 'a':
		return 2
	case 's':
		return 3
	}
	return -1
}

type Part [4]int

func (p *Part) GetProp(propName byte) int {
	if idx := Xmas(propName); idx != -1 {
		return p[idx]
	}
	return 0
}

func (p *Part) SetProp(propName byte, val int) {
	if idx := Xmas(propName); idx != -1 {
		p[idx] = val
	}
}

func (p *Part) Sum() int {
	return p[0] + p[1] + p[2] + p[3]
}

type RuleOp uint8

const (
	OpAny RuleOp = iota
	OpGt
	OpLt
)

type Rule struct {
	operator    RuleOp
	propName    byte
	operand     int
	destination string
}

func (r *Rule) Match(p *Part) bool {
	propVal := p.GetProp(r.propName)
	switch r.operator {
	case OpGt:
		return propVal > r.operand
	case OpLt:
		return propVal < r.operand
	case OpAny:
		return true
	}
	return false
}

func (r *Rule) Reverse() *Rule {
	rev := *r
	switch r.operator {
	case OpGt:
		rev.operator = OpLt
		rev.operand += 1
	case OpLt:
		rev.operator = OpGt
		rev.operand -= 1
	}
	return &rev
}

type Workflow struct {
	name  string
	rules []Rule
}

func (w *Workflow) Eval(p *Part) string {
	for i := range w.rules {
		if w.rules[i].Match(p) {
			return w.rules[i].destination
		}
	}
	return TgtInvalid
}

func parseRule(input string) (*Rule, error) {
	parts := strings.Split(input, ":")
	if len(parts) == 1 {
		return &Rule{
			operator:    OpAny,
			destination: parts[0],
		}, nil
	}
	if len(parts) != 2 || len(parts[0]) < 3 {
		return nil, fmt.Errorf("invalid format")
	}

	var op RuleOp
	switch parts[0][1] {
	case '>':
		op = OpGt
	case '<':
		op = OpLt
	default:
		return nil, fmt.Errorf("invalid operator. Must be one of > or <")
	}
	prop := parts[0][0]
	num, err := strconv.ParseInt(parts[0][2:], 10, 64)
	if err != nil {
		return nil, fmt.Errorf("invalid number")
	}
	return &Rule{
		operator:    op,
		propName:    prop,
		operand:     int(num),
		destination: parts[1],
	}, nil
}

func parseWorkflow(input string) (*Workflow, error) {
	idx := strings.Index(input, "{")
	if idx < 1 || !strings.HasSuffix(input, "}") {
		return nil, fmt.Errorf("invalid format")
	}
	name := input[:idx]

	rulesRaw := strings.Split(input[idx+1:len(input)-1], ",")
	rules := make([]Rule, len(rulesRaw))
	for i := range rulesRaw {
		rule, err := parseRule(rulesRaw[i])
		if err != nil {
			return nil, fmt.Errorf("invalid rule %q: %v", rulesRaw[i], err)
		}
		rules[i] = *rule
	}
	return &Workflow{
		name:  name,
		rules: rules,
	}, nil
}

func parsePart(input string) (*Part, error) {
	if len(input) < 17 || !strings.HasPrefix(input, "{") || !strings.HasSuffix(input, "}") {
		return nil, fmt.Errorf("invalid format")
	}

	p := Part{}
	for propDef := range strings.SplitSeq(input[1:len(input)-1], ",") {
		if len(propDef) < 3 || propDef[1] != '=' {
			return nil, fmt.Errorf("invalid property definition %q", propDef)
		}
		prop := propDef[0]
		val, err := strconv.ParseInt(propDef[2:], 10, 64)
		if err != nil {
			return nil, fmt.Errorf("invalid property definition %q", propDef)
		}
		p.SetProp(prop, int(val))
	}
	return &p, nil
}

// Part 2

type Range struct {
	min, max int
}

func (r *Range) Len() int {
	if ret := r.max - r.min + 1; ret > 0 {
		return ret
	}
	return 0
}

func (r *Range) Apply(operator RuleOp, operand int) {
	switch operator {
	case OpGt:
		r.min = operand + 1
	case OpLt:
		r.max = operand - 1
	}
}

type Constraint struct {
	ranges [4]Range
	valid  bool
}

func (c *Constraint) Apply(r *Rule) bool {
	if idx := Xmas(r.propName); idx != -1 {
		c.ranges[idx].Apply(r.operator, r.operand)
		if c.ranges[idx].Len() == 0 {
			c.valid = false
		}
	}
	return c.valid
}

func (c *Constraint) Combinations() int64 {
	if !c.valid {
		return 0
	}
	var ret int64 = 1
	for i := range c.ranges {
		ret *= int64(c.ranges[i].Len())
	}
	return ret
}

func NewConstraint() *Constraint {
	r := Range{1, 4000}
	return &Constraint{
		ranges: [4]Range{r, r, r, r},
		valid:  true,
	}
}
