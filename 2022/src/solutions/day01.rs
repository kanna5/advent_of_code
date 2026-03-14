//! Implements a solution for https://adventofcode.com/2022/day/1

use std::io::BufRead;

use crate::solutions::Solution;
use anyhow::{Context, Ok};

pub struct Day01;

impl<R: BufRead> Solution<R> for Day01 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut elves = read_input(input)?;
        elves.sort();
        Ok(elves
            .last()
            .context("no enough elves (expected 1)")?
            .to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut elves = read_input(input)?;
        elves.sort();
        let last_3 = elves
            .last_chunk::<3>()
            .with_context(|| format!("no enough elves. (expected 3, got {})", elves.len()))?;
        Ok(last_3.iter().sum::<i64>().to_string())
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<i64>, anyhow::Error> {
    let mut elves = Vec::with_capacity(128);

    let mut tmp = 0i64;
    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            elves.push(tmp);
            tmp = 0;
            continue;
        }
        tmp += line
            .parse::<i64>()
            .with_context(|| format!("failed to parse into integer: \"{}\"", line))?;
    }

    if tmp > 0 {
        elves.push(tmp);
    }
    Ok(elves)
}
