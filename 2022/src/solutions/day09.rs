//! Implements a solution for https://adventofcode.com/2022/day/9
//!
use std::{collections::HashSet, io::BufRead, str::FromStr};

use crate::{answer, solutions::Solution};
use anyhow::{Context, anyhow};

pub struct Day09;

impl<R: BufRead> Solution<R> for Day09 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut visited = HashSet::<(i64, i64)>::new();
        visited.insert((0, 0));

        let (mut head, mut tail) = ((0i64, 0i64), (0i64, 0i64));
        for line in input.lines() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            let cmd =
                parse_input(&line).with_context(|| anyhow!("failed to parse line {:?}", line))?;

            let (dx, dy) = cmd.dir.vec();
            for _ in 0..cmd.dist {
                head = (head.0 + dx, head.1 + dy);
                let (dist_x, dist_y) = (head.0 - tail.0, head.1 - tail.1);
                if dist_x.abs() > 1 || dist_y.abs() > 1 {
                    tail = (tail.0 + dist_x.signum(), tail.1 + dist_y.signum());
                    visited.insert(tail);
                }
            }
        }
        answer!(visited.len())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut visited = HashSet::<(i64, i64)>::new();
        visited.insert((0, 0));
        let mut knots = [(0i64, 0i64); 10];

        for line in input.lines() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            let cmd =
                parse_input(&line).with_context(|| anyhow!("failed to parse line {:?}", line))?;

            let (dx, dy) = cmd.dir.vec();
            for _ in 0..cmd.dist {
                knots[0] = (knots[0].0 + dx, knots[0].1 + dy);
                for i in 1..10 {
                    let (dist_x, dist_y) =
                        (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);
                    if dist_x.abs() > 1 || dist_y.abs() > 1 {
                        knots[i] = (knots[i].0 + dist_x.signum(), knots[i].1 + dist_y.signum());
                    }
                }
                visited.insert(knots[9]);
            }
        }
        answer!(visited.len())
    }
}

enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn vec(self) -> (i64, i64) {
        match self {
            Dir::U => (0, -1),
            Dir::R => (1, 0),
            Dir::D => (0, 1),
            Dir::L => (-1, 0),
        }
    }
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::U,
            "R" => Self::R,
            "D" => Self::D,
            "L" => Self::L,
            e => Err(anyhow!("unknown direction {:?}", e))?,
        })
    }
}

struct Command {
    dir: Dir,
    dist: i64,
}

fn parse_input(line: &str) -> Result<Command, anyhow::Error> {
    let mut splt = line.split_whitespace();
    let dir: Dir = splt.next().context("no enough fields")?.parse()?;
    let dist: i64 = splt.next().context("no enough fields")?.parse()?;

    if splt.next().is_some() {
        return Err(anyhow!("invalid number of fields"));
    }
    Ok(Command { dir, dist })
}
