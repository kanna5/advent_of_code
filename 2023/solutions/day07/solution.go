// Solution for https://adventofcode.com/2023/day/7
package day07

import (
	"bufio"
	"fmt"
	"io"
	"slices"
	"strconv"
	"strings"

	"github.com/kanna5/advent_of_code/2023/solutions"
)

type Card struct {
	label rune
	power uint8
}

func cardTplGen(joker bool) map[rune]Card {
	var jPower uint8 = 9
	var correction uint8 = 0
	if joker {
		jPower = 0
		correction = 1
	}
	return map[rune]Card{
		'2': {'2', 0 + correction},
		'3': {'3', 1 + correction},
		'4': {'4', 2 + correction},
		'5': {'5', 3 + correction},
		'6': {'6', 4 + correction},
		'7': {'7', 5 + correction},
		'8': {'8', 6 + correction},
		'9': {'9', 7 + correction},
		'T': {'T', 8 + correction},
		'J': {'J', jPower},
		'Q': {'Q', 10},
		'K': {'K', 11},
		'A': {'A', 12},
	}
}

var (
	cardTpl       = cardTplGen(false)
	cardTplWJoker = cardTplGen(true)
)

type TypeOfHand uint8

const (
	HighCard TypeOfHand = iota
	OnePair
	TwoPair
	ThreeOfAKind
	FullHouse
	FourOfAKind
	FiveOfAKind
)

type Hand struct {
	cards [5]Card
	power int64
	type_ TypeOfHand
	bid   int64
}

func getType(cards [5]Card, joker bool) TypeOfHand {
	count := make(map[rune]uint8, 5)
	for _, card := range cards {
		count[card.label] += 1
	}

	counts := make([]uint8, 0, 5)
	cJoker := uint8(0)
	for l, c := range count {
		if l == 'J' && joker {
			cJoker = c
			counts = append(counts, 0)
		} else {
			counts = append(counts, c)
		}
	}
	slices.Sort(counts)
	slices.Reverse(counts)
	counts[0] += cJoker

	switch true {
	case counts[0] == 5:
		return FiveOfAKind
	case counts[0] == 4:
		return FourOfAKind
	case counts[0] == 3 && counts[1] == 2:
		return FullHouse
	case counts[0] == 3:
		return ThreeOfAKind
	case counts[0] == 2 && counts[1] == 2:
		return TwoPair
	case counts[0] == 2:
		return OnePair
	}
	return HighCard
}

func getPower(cards [5]Card) int64 {
	var sum int64 = 0
	for _, c := range cards {
		sum = sum*13 + int64(c.power)
	}
	return sum
}

func NewHand(cards [5]Card, bid int64, joker bool) *Hand {
	return &Hand{
		cards: cards,
		bid:   bid,
		power: getPower(cards),
		type_: getType(cards, joker),
	}
}

func parseHand(line string, joker bool) (*Hand, error) {
	tpl := cardTpl
	if joker {
		tpl = cardTplWJoker
	}

	parts := strings.Fields(line)
	if len(parts) != 2 {
		return nil, fmt.Errorf("must have exactly two parts")
	}
	cardsRaw := []rune(parts[0])
	if len(cardsRaw) != 5 {
		return nil, fmt.Errorf("must have exactly five cards")
	}
	cards := [5]Card{}
	for i := range 5 {
		if t, ok := tpl[cardsRaw[i]]; !ok {
			return nil, fmt.Errorf("unknown card %v", cardsRaw[i])
		} else {
			cards[i] = t
		}
	}
	bid, err := strconv.ParseInt(parts[1], 10, 64)
	if err != nil {
		return nil, err
	}
	return NewHand(cards, bid, joker), nil
}

type sol struct {
	input io.Reader
}

func (s *sol) solve(joker bool) (string, error) {
	sc := bufio.NewScanner(s.input)
	var hands []*Hand
	for sc.Scan() {
		line := sc.Text()
		h, err := parseHand(line, joker)
		if err != nil {
			return "", fmt.Errorf("failed to parse %v: %v", line, err)
		}
		hands = append(hands, h)
	}
	if err := sc.Err(); err != nil {
		return "", err
	}

	slices.SortFunc(hands, func(a, b *Hand) int {
		if a.type_ != b.type_ {
			return int(a.type_) - int(b.type_)
		}
		return int(a.power - b.power)
	})
	var sum int64
	for i, h := range hands {
		sum += int64(i+1) * h.bid
	}

	return strconv.FormatInt(sum, 10), nil
}

func (s *sol) SolvePart1() (string, error) {
	return s.solve(false)
}

func (s *sol) SolvePart2() (string, error) {
	return s.solve(true)
}

func (s *sol) WithInput(i io.Reader) solutions.Solver {
	s.input = i
	return s
}

func init() {
	solutions.Days[7] = &sol{}
}
