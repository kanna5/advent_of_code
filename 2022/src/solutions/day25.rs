//! Implements a solution for https://adventofcode.com/2022/day/25

use std::io::BufRead;

use crate::{answer, solutions::Solution};
use anyhow::{Context, bail};

pub struct Day25;

impl<R: BufRead> Solution<R> for Day25 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut sum = 0;
        for (lineno, line) in input.lines().enumerate() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            sum += decode(&line).with_context(|| format!("decode line {}", lineno))?;
        }
        answer!(encode(sum))
    }

    fn part2(&self, _input: &mut R) -> Result<String, anyhow::Error> {
        answer!("🌟🍓🥤✨🎄⭐")
    }
}

fn decode(snafu: &str) -> anyhow::Result<isize> {
    let mut num = 0;
    let mut base = 1;
    for b in snafu.as_bytes().iter().rev() {
        let cur = match b {
            b'2' => 2,
            b'1' => 1,
            b'0' => 0,
            b'-' => -1,
            b'=' => -2,
            b => bail!("invalid character {:?}", char::from(*b)),
        };
        num += cur * base;
        base *= 5
    }
    Ok(num)
}

fn encode(mut num: isize) -> String {
    let mut s = vec![];

    while num > 0 {
        let c = match num % 5 {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => {
                num += 5;
                b'='
            }
            4 => {
                num += 5;
                b'-'
            }
            _ => unreachable!(),
        };
        s.push(c);
        num /= 5
    }
    s.reverse();
    String::from_utf8(s).unwrap()
}
