//! Implements a solution for https://adventofcode.com/2022/day/2

use std::{io::BufRead, io::Lines};

use crate::solutions::Solution;
use anyhow::{Context, anyhow};

pub struct Day02;

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> i64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome(self, opponent: Self) -> i64 {
        let (a, b) = (self.score() - 1, opponent.score() - 1);
        if (a + 1) % 3 == b {
            0 // lose
        } else if (a + 2) % 3 == b {
            6 // win
        } else {
            3 // draw
        }
    }

    fn find_win(self) -> Self {
        match self {
            Shape::Rock => Self::Paper,
            Shape::Paper => Self::Scissors,
            Shape::Scissors => Self::Rock,
        }
    }

    fn find_lose(self) -> Self {
        match self {
            Shape::Rock => Self::Scissors,
            Shape::Paper => Self::Rock,
            Shape::Scissors => Self::Paper,
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            s => Err(anyhow!("invalid shape {:?}", s))?,
        })
    }
}

impl<R: BufRead> Solution<R> for Day02 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let r = Reader::new(input);
        let mut score = 0i64;

        for round in r {
            let (a, b) = round?;
            score += b.score() + b.outcome(a);
        }
        Ok(score.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let r = Reader::new(input);
        let mut score = 0i64;

        for round in r {
            let (a, b) = round?;
            let our = match b {
                Shape::Rock => a.find_lose(),
                Shape::Paper => a,
                Shape::Scissors => a.find_win(),
            };

            score += our.score() + our.outcome(a);
        }

        Ok(score.to_string())
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
    type Item = Result<(Shape, Shape), anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.lines.next();
        n.map(|r| {
            let s = r?;
            let (a, b) = s
                .split_once(' ')
                .with_context(|| format!("invalid line \"{}\"", s))?;

            Ok((a.try_into()?, b.try_into()?))
        })
    }
}
