# Advent of Code 2022 Solution in Rust

## Usage

```
cargo run -- <day> <part> [input_path|-] [key=value ...]
```

Arguments:

- `day`: Day number (1-25).
- `part`: `1` or `2`.
- `input_path` (optional):
  When omitted, reads `input/day-XX.txt`;
  when it's a valid path, reads from that file;
  when it's `-`, reads from stdin.
- `key=value` options (optional): Passed to the solution code.
