## Advent of Code 2024 with Rust

## Usage

### Getting the input

First, you need to save your session cookie in the `.env` file in this directory.

Look for the `session=` cookie, copy the value, and in the `.env` file, add:

`COOKIE_SESSION=<VALUE>`

Replace `<VALUE>` with the value you copied.

Then, just run `bash get_input.sh`, and your puzzle inputs for each day will be saved in the `input` directory.

### Running each solution

For example, executing `cargo run 24 1` will run the solution for day 24 part 1. By default, it will look for puzzle input in the `input` directory.
You can also specify an input file after the `day` and `part` arguments; `-` stands for standard input.
