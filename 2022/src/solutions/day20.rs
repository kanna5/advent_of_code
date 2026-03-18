//! Implements a solution for https://adventofcode.com/2022/day/20

use std::io::BufRead;

use crate::{answer, solutions::Solution};
use anyhow::Context;

const KEY: i64 = 811589153;

pub struct Day20;

impl<R: BufRead> Solution<R> for Day20 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut numbers = read_input(input)?;
        mix(&mut numbers);

        let ans = decode(&numbers).context("zero not found")?;
        answer!(ans)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut numbers = read_input(input)?;
        for n in numbers.iter_mut() {
            n.val *= KEY
        }

        for _ in 0..10 {
            mix(&mut numbers);
        }
        let ans = decode(&numbers).context("zero not found")?;
        answer!(ans)
    }
}

#[derive(Clone, Copy, Debug)]
struct Num {
    val: i64,
    orig_pos: usize,
}

fn mix(numbers: &mut [Num]) {
    let length = numbers.len();
    let wrap = (length - 1) as i64;

    let mut pos = vec![0usize; length];
    for (p, num) in numbers.iter().enumerate() {
        pos[num.orig_pos] = p;
    }

    for i in 0..length {
        let num = numbers[pos[i]];
        let new_pos = match pos[i] as i64 + num.val % wrap {
            p if p < 0 => p + wrap,
            p if p >= length as i64 => p - wrap,
            p => p,
        } as usize;

        match new_pos.cmp(&pos[i]) {
            std::cmp::Ordering::Less => {
                for j in (new_pos..pos[i]).rev() {
                    pos[numbers[j].orig_pos] += 1;
                    numbers[j + 1] = numbers[j];
                }
            }
            std::cmp::Ordering::Greater => {
                for j in pos[i] + 1..=new_pos {
                    pos[numbers[j].orig_pos] -= 1;
                    numbers[j - 1] = numbers[j];
                }
            }
            std::cmp::Ordering::Equal => (),
        }
        pos[num.orig_pos] = new_pos;
        numbers[new_pos] = num;
    }
}

fn decode(numbers: &[Num]) -> Option<i64> {
    let (zero_pos, _) = numbers.iter().enumerate().find(|&(_, num)| num.val == 0)?;

    let ans: i64 = [1000_usize, 2000, 3000]
        .iter()
        .map(|&d| numbers[(zero_pos + d) % numbers.len()].val)
        .sum();
    Some(ans)
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Num>> {
    let mut ret = Vec::with_capacity(1024);
    for (lineno, line) in input.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        ret.push(Num {
            val: line
                .parse()
                .with_context(|| format!("parsing line {}", lineno + 1))?,
            orig_pos: lineno,
        });
    }
    Ok(ret)
}
