//go:build ignore

package main

import (
	_ "embed"
	"fmt"
	"os"
	"path"
	"text/template"
)

//go:embed solutions/solution.go.tpl
var solutionTplS string

var solutionTpl = template.Must(template.New("solution").Parse(solutionTplS))

type solutionArgs struct {
	DayNumber int
	Day       string
}

func ensureDir(pth string) error {
	st, err := os.Stat(pth)
	if os.IsNotExist(err) {
		return os.Mkdir(pth, 0o755)
	}
	if err != nil {
		return err
	}
	if !st.IsDir() {
		return fmt.Errorf("%q is not a directory", pth)
	}
	return nil
}

func main() {
	st, err := os.Stat(path.Join(".", "solutions"))
	if err != nil || !st.IsDir() {
		panic("\"solutions\" must be a directory and accessible.")
	}

	for i := range 12 {
		day := fmt.Sprintf("%02d", i+1)
		dayDir := path.Join(".", "solutions", "day"+day)
		solFile := path.Join(dayDir, "solution.go")
		if err := ensureDir(dayDir); err != nil {
			panic(err)
		}

		_, err := os.Stat(solFile)
		if os.IsNotExist(err) {
			fd, err := os.OpenFile(solFile, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o644)
			if err != nil {
				panic(err)
			}
			if err := solutionTpl.Execute(fd, solutionArgs{Day: day, DayNumber: i + 1}); err != nil {
				panic(err)
			}
			fd.Close()
		} else if err != nil {
			panic(err)
		}
	}

	fd, err := os.OpenFile("imports.go", os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o644)
	if err != nil {
		panic(err)
	}
	defer fd.Close()

	fmt.Fprintf(fd, "package main\n\nimport (\n")
	for i := range 12 {
		day := fmt.Sprintf("day%02d", i+1)
		fmt.Fprintf(fd, "\t_ \"github.com/kanna5/advent_of_code/2025/solutions/%s\"\n", day)
	}
	fmt.Fprintf(fd, ")\n")
}
