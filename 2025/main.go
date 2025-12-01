package main

//go:generate go run gen_tpls.go

import (
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"path"
	"strconv"

	"github.com/kanna5/advent_of_code/2025/solutions"
)

func usage() {
	_, _ = fmt.Fprintf(flag.CommandLine.Output(), "Usage: %s <day> <part> [input_file|-] \n", os.Args[0])
	flag.PrintDefaults()
}

func usage_err(msg string) {
	if len(msg) > 0 {
		fmt.Fprintf(os.Stderr, "Invalid usage: %s\n", msg)
	}
	usage()
	os.Exit(1)
}

func init() {
	flag.Usage = usage
}

func get_input_stream(day int, path_ *string) (io.ReadCloser, error) {
	var path_t string
	if path_ == nil {
		path_t = path.Join("input", fmt.Sprintf("day-%02d.txt", day))
	} else {
		path_t = *path_
	}

	if path_t == "-" {
		log.Printf("Reading from STDIN")
		return os.Stdin, nil
	} else {
		return os.Open(path_t)
	}
}

func main() {
	flag.Parse()
	args := flag.Args()

	if len(args) < 2 {
		usage_err("<day> and <part> are required.")
	}
	day, err := strconv.Atoi(args[0])
	if err != nil || day <= 0 || day > 25 {
		usage_err("<day> can be 1~25")
	}
	part, err := strconv.Atoi(args[1])
	if err != nil || part <= 0 || part > 2 {
		usage_err("<part> can be 1 or 2")
	}

	solver := solutions.Days[day]
	if solver == nil {
		log.Fatalf("Solution for day %d is not implemented yet", day)
	}

	var path_ *string = nil
	if len(args) >= 3 {
		path_ = &args[2]
	}
	input_stream, err := get_input_stream(day, path_)
	if err != nil {
		log.Fatalf("Failed to read input file: %v\n", err)
	}
	defer func() { _ = input_stream.Close() }()
	solver.WithInput(input_stream)

	var result string
	switch part {
	case 1:
		result, err = solver.SolvePart1()
	case 2:
		result, err = solver.SolvePart2()
	}
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(result)
}
