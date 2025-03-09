use super::Solution;
use std::io::BufRead;

pub struct Day4;

impl<R: BufRead> Solution<R> for Day4 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let puzzle = parse_input(input)?;
        let mut occurs = 0;

        for y in 0..puzzle.height {
            for x in 0..puzzle.width {
                if puzzle.data[y][x] != WORD[0] {
                    continue;
                }
                for dir in SEARCH_PATTERNS {
                    if puzzle.is_match_1(x, y, dir) {
                        occurs += 1
                    }
                }
            }
        }

        Ok(occurs.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let puzzle = parse_input(input)?;
        let mut occurs = 0;

        for y in 1..puzzle.height - 1 {
            for x in 1..puzzle.width - 1 {
                let data = &puzzle.data;
                if data[y][x] != b'A' {
                    continue;
                }
                let extracted = [
                    data[y - 1][x - 1],
                    data[y + 1][x + 1],
                    data[y + 1][x - 1],
                    data[y - 1][x + 1],
                ];
                let m = |a: &[u8]| -> bool { a == b"MS" || a == b"SM" };
                if m(&extracted[0..=1]) && m(&extracted[2..=3]) {
                    occurs += 1
                }
            }
        }

        Ok(occurs.to_string())
    }
}

const WORD: [u8; 4] = *b"XMAS";
const SEARCH_PATTERNS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

struct Puzzle {
    width: usize,
    height: usize,
    data: Vec<Vec<u8>>,
}

impl Puzzle {
    fn is_match_1(&self, x: usize, y: usize, direction: (i8, i8)) -> bool {
        for i in 1..=3 {
            let chk_x = x as i64 + (direction.0 as i64) * i;
            let chk_y = y as i64 + (direction.1 as i64) * i;
            if !(0..self.width as i64).contains(&chk_x) || !(0..self.height as i64).contains(&chk_y)
            {
                return false;
            }
            if self.data[chk_y as usize][chk_x as usize] != WORD[i as usize] {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid input: {0}")]
pub struct ParseError(String);

fn parse_input<R: BufRead>(input: &mut R) -> Result<Puzzle, ParseError> {
    let mut width = 0usize;
    let mut ret: Vec<Vec<u8>> = Vec::new();

    for line in input.lines().map_while(Result::ok) {
        let line = line.into_bytes();
        if line.is_empty() {
            continue;
        }
        let len = line.len();
        match width {
            0 => width = len,
            l if l != len => return Err(ParseError("variable line length".into())),
            _ => (),
        }
        ret.push(line);
    }
    Ok(Puzzle {
        width,
        height: ret.len(),
        data: ret,
    })
}
