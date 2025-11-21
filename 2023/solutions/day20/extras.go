package day20

import (
	"fmt"
	"strings"

	"github.com/kanna5/advent_of_code/2023/lib"
)

func drawDiagram(sc *Scene) string {
	builder := strings.Builder{}
	fmt.Fprintln(&builder, "digraph \"Day 20\" {")

	defined := lib.Set[string]{}

	for _, name := range sc.moduleNames {
		mod := sc.modules[name]
		if !defined.Has(name) {
			if IsConjunction(mod.Module) {
				fmt.Fprintf(&builder, "  %s [shape=Msquare style=filled fillcolor=\"/pastel16/3\"];\n", name)
			} else if IsFlipFlop(mod.Module) {
				fmt.Fprintf(&builder, "  %s [shape=rect];\n", name)
			} else if IsBroadcaster(mod.Module) {
				fmt.Fprintf(&builder, "  %s [shape=doublecircle label=\"START\" style=filled fillcolor=\"/pastel16/1\"];\n", name)
			}
			defined.Add(name)
		}
		for _, oName := range mod.outputs {
			_, ok := sc.modules[oName]
			if !ok && !defined.Has(oName) {
				fmt.Fprintf(&builder, "  %s [shape=star style=filled fillcolor=\"/pastel16/6\"];\n", oName)
				defined.Add(oName)
			}
			fmt.Fprintf(&builder, "  %s -> %s;\n", name, oName)
		}
	}

	fmt.Fprintln(&builder, "}")
	return builder.String()
}

var diagramMsg = "" +
	"Diagram saved to \"./day20.dot\". You can:\n" +
	"- Use the `dot` command from GraphViz to render the diagram to SVG and view it in a browser.\n" +
	"- Or view it directly in an online viewer (e.g., https://dreampuf.github.io/GraphvizOnline/)\n"
