use std::{collections::HashMap, error::Error, io::BufRead, str::FromStr};

use anyhow::Context;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub trait Solution<R: BufRead = Box<dyn BufRead>> {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error>;
    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error>;
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid input: \"{0}\"")]
pub struct InvalidInput(String);

pub fn get_solution(day: u8, options: Options) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        2 => Some(Box::new(day02::Day02)),
        3 => Some(Box::new(day03::Day03)),
        4 => Some(Box::new(day04::Day04)),
        5 => Some(Box::new(day05::Day05)),
        6 => Some(Box::new(day06::Day06)),
        7 => Some(Box::new(day07::Day07)),
        8 => Some(Box::new(day08::Day08)),
        9 => Some(Box::new(day09::Day09)),
        10 => Some(Box::new(day10::Day10)),
        11 => Some(Box::new(day11::Day11)),
        12 => Some(Box::new(day12::Day12)),
        13 => Some(Box::new(day13::Day13)),
        14 => Some(Box::new(day14::Day14 { opts: options })),
        15 => Some(Box::new(day15::Day15 { opts: options })),
        16 => Some(Box::new(day16::Day16)),
        17 => Some(Box::new(day17::Day17)),
        18 => Some(Box::new(day18::Day18)),
        19 => Some(Box::new(day19::Day19)),
        20 => Some(Box::new(day20::Day20)),
        21 => Some(Box::new(day21::Day21)),
        22 => Some(Box::new(day22::Day22)),
        23 => Some(Box::new(day23::Day23)),
        24 => Some(Box::new(day24::Day24)),
        25 => Some(Box::new(day25::Day25)),
        _ => None,
    }
}

pub struct Options(HashMap<String, String>);

impl Options {
    pub fn new(opts: HashMap<String, String>) -> Self {
        Self(opts)
    }

    pub fn try_get<F: FromStr>(&self, name: &str, default: F) -> anyhow::Result<F>
    where
        F::Err: Error + Send + Sync + 'static,
    {
        match self.0.get(name) {
            Some(v) => v
                .parse()
                .with_context(|| format!("cannot parse option {:?}", name)),
            None => Ok(default),
        }
    }
}

#[macro_export]
macro_rules! answer {
    ($x:expr) => {
        Ok(($x).to_string())
    };
}
