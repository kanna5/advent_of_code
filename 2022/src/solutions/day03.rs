//! Implements a solution for https://adventofcode.com/2022/day/3

use std::io::BufRead;

use crate::solutions::Solution;
use anyhow::anyhow;

pub struct Day03;

fn prio_of_char(c: u8) -> usize {
    if c.is_ascii_lowercase() {
        return (c - b'a' + 1) as usize;
    }
    (c - b'A' + 27) as usize
}

impl<R: BufRead> Solution<R> for Day03 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut prio = 0usize;

        for line in input.lines().map_while(Result::ok) {
            if line.is_empty() {
                break;
            }
            let len = line.len();
            let mut seen = [false; 52];

            for (i, b) in line.bytes().enumerate() {
                let idx = prio_of_char(b) - 1;
                if i < len / 2 {
                    seen[idx] = true;
                } else if seen[idx] {
                    prio += idx + 1;
                    break;
                }
            }
        }
        Ok(prio.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut prio = 0usize;
        let mut n_elves = 0;

        let mut seen = [0u8; 52];
        for line in input.lines().map_while(Result::ok) {
            if line.is_empty() {
                break;
            }
            n_elves += 1;

            for b in line.bytes() {
                let idx = prio_of_char(b) - 1;
                seen[idx] |= 1 << (n_elves % 3);
                if seen[idx] == 0b111 {
                    prio += idx + 1;
                    break;
                }
            }

            if n_elves % 3 == 0 {
                seen = [0u8; 52];
            }
        }
        if n_elves % 3 != 0 {
            return Err(anyhow!("cannot divide into group of 3"));
        }

        Ok(prio.to_string())
    }
}
