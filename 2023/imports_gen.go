//go:build ignore

package main

import (
	"fmt"
	"os"
	"path"
	"path/filepath"
	"slices"
	"strings"
)

func main() {
	paths, err := filepath.Glob(path.Join("solutions", "day??", "*.go"))
	if err != nil {
		panic(err)
	}

	implemented_set := make(map[string]bool)
	for _, pth := range paths {
		parts := strings.Split(pth, string(os.PathSeparator))
		if len(parts) < 2 {
			continue
		}
		implemented_set[parts[1]] = true
	}

	implemented := make([]string, 0, len(implemented_set))
	for k := range implemented_set {
		implemented = append(implemented, k)
	}
	slices.Sort(implemented)

	fd, err := os.OpenFile("imports.go", os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o644)
	if err != nil {
		panic(err)
	}
	defer fd.Close()

	fmt.Fprintf(fd, "package main\n\nimport (\n")
	for _, s := range implemented {
		fmt.Fprintf(fd, "\t_ \"github.com/kanna5/advent_of_code/2023/solutions/%s\"\n", s)
	}
	fmt.Fprintf(fd, ")\n")
}
