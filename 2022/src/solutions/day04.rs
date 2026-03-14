//! Implements a solution for https://adventofcode.com/2022/day/4

use std::io::{BufRead, Lines};

use crate::solutions::Solution;
use anyhow::Context;

pub struct Day04;

impl<R: BufRead> Solution<R> for Day04 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let r = Reader::new(input);

        let mut pairs = 0;
        for pair in r {
            let pair = pair?;
            if pair[0] <= pair[2] && pair[1] >= pair[3] || pair[2] <= pair[0] && pair[3] >= pair[1]
            {
                pairs += 1
            }
        }
        Ok(pairs.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let r = Reader::new(input);

        let mut pairs = 0;
        for pair in r {
            let pair = pair?;
            let (range1, range2) = (pair[0]..=pair[1], pair[2]..=pair[3]);

            if range1.contains(&pair[2])
                || range1.contains(&pair[3])
                || range2.contains(&pair[0])
                || range2.contains(&pair[1])
            {
                pairs += 1
            }
        }
        Ok(pairs.to_string())
    }
}

struct Reader<'a, R: BufRead> {
    lines: Lines<&'a mut R>,
}

impl<'a, R: BufRead> Reader<'a, R> {
    fn new(input: &'a mut R) -> Self {
        Self {
            lines: input.lines(),
        }
    }
}

impl<'a, R: BufRead> Iterator for Reader<'a, R> {
    type Item = Result<[i64; 4], anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.lines.next();
        n.map(|r| {
            let s = r?;
            let err_ctx = || format!("invalid line \"{}\"", s);

            let (a, b) = s.split_once(',').with_context(err_ctx)?;
            let (a1, a2) = a.split_once('-').with_context(err_ctx)?;
            let (b1, b2) = b.split_once('-').with_context(err_ctx)?;

            Ok([a1.parse()?, a2.parse()?, b1.parse()?, b2.parse()?])
        })
    }
}
