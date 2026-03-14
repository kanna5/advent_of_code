//! Implements a solution for https://adventofcode.com/2022/day/5

use std::{cmp, io::BufRead};

use crate::solutions::Solution;
use anyhow::anyhow;

pub struct Day05;

impl<R: BufRead> Solution<R> for Day05 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (mut stacks, moves) = read_input(input)?;

        for m in &moves {
            for _ in 0..m.amount {
                if stacks[m.from as usize].is_empty() {
                    break;
                }
                let c = stacks[m.from as usize].pop().unwrap();
                stacks[m.to as usize].push(c);
            }
        }
        Ok(tops(stacks))
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (mut stacks, moves) = read_input(input)?;

        for m in &moves {
            let src = &mut stacks[m.from as usize];
            let cut_at = cmp::max(0, src.len() as i64 - m.amount) as usize;
            let moved = src.split_off(cut_at);

            let dst = &mut stacks[m.to as usize];
            dst.extend(moved);
        }
        Ok(tops(stacks))
    }
}

struct Move {
    from: i64,
    to: i64,
    amount: i64,
}

fn tops(stacks: Vec<Vec<u8>>) -> String {
    let mut ret = String::with_capacity(stacks.len());
    for s in &stacks {
        match s.last() {
            Some(&c) => ret.push(c as char),
            None => ret.push(' '),
        }
    }
    ret
}

fn read_input<R: BufRead>(input: &mut R) -> Result<(Vec<Vec<u8>>, Vec<Move>), anyhow::Error> {
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    let mut moves: Vec<Move> = Vec::with_capacity(128);
    let mut read_stack = true;
    let mut n_stacks = 0;

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            read_stack = false;
            continue;
        }

        if read_stack {
            if n_stacks == 0 {
                n_stacks = (line.len() + 1) / 4;
                stacks.reserve_exact(n_stacks);
                for _ in 0..n_stacks {
                    stacks.push(Vec::with_capacity(256));
                }
            }

            let line = line.as_bytes();
            for i in 0..n_stacks {
                let c = line[i * 4 + 1];
                if c.is_ascii_digit() {
                    break; // ID of stacks, skip
                }

                if c != b' ' {
                    stacks[i].push(c);
                }
            }
        } else {
            // Read moves
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 6 || parts[0] != "move" || parts[2] != "from" || parts[4] != "to" {
                return Err(anyhow!("invalid move: \"{}\"", line));
            }
            moves.push(Move {
                from: parts[3].parse::<i64>()? - 1,
                to: parts[5].parse::<i64>()? - 1,
                amount: parts[1].parse()?,
            });
        }
    }

    // Reverse the stacks
    for s in &mut stacks {
        s.reverse();
    }
    Ok((stacks, moves))
}
