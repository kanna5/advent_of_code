use super::{InvalidInput, Solution};
use anyhow::Context;
use std::io::BufRead;

pub struct Day7;

impl<R: BufRead> Solution<R> for Day7 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let equations = read_input(input)?;

        let sum: i64 = equations
            .iter()
            .map(
                |e| match try_equate_1(e.target, e.numbers[0], &e.numbers[1..]) {
                    true => e.target,
                    false => 0,
                },
            )
            .sum();
        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let equations = read_input(input)?;

        let sum: i64 = equations
            .iter()
            .map(
                |e| match try_equate_2(e.target, e.numbers[0], &e.numbers[1..]) {
                    true => e.target,
                    false => 0,
                },
            )
            .sum();
        Ok(sum.to_string())
    }
}

#[derive(Debug)]
struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn new(target: i64, numbers: Vec<i64>) -> Self {
        Equation { target, numbers }
    }
}

fn try_equate_1(target: i64, prev: i64, numbers: &[i64]) -> bool {
    if prev > target {
        return false;
    }
    if numbers.is_empty() {
        return prev == target;
    }
    let next = numbers[0];
    let remains = &numbers[1..];
    try_equate_1(target, prev * next, remains) || try_equate_1(target, prev + next, remains)
}

fn try_equate_2(target: i64, prev: i64, numbers: &[i64]) -> bool {
    if prev > target {
        return false;
    }
    if numbers.is_empty() {
        return prev == target;
    }
    let next = numbers[0];
    let remains = &numbers[1..];
    try_equate_2(target, prev * next, remains)
        || try_equate_2(target, prev + next, remains)
        || try_equate_2(target, concat_num(prev, next), remains)
}

fn concat_num(a: i64, b: i64) -> i64 {
    #[allow(clippy::comparison_chain)]
    if b > 0 {
        return a * 10i64.pow(b.ilog10() + 1) + b;
    } else if b == 0 {
        return a * 10;
    }
    panic!("concatenating of negative numbers is not implemented");
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<Equation>, anyhow::Error> {
    let mut ret: Vec<Equation> = Vec::with_capacity(512);

    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let parts = line
            .split_once(':')
            .ok_or_else(|| InvalidInput(line.clone()))?;
        let target: i64 = parts.0.parse().with_context(|| line.clone())?;
        let numbers: Vec<i64> = parts
            .1
            .split_whitespace()
            .map(|n| n.parse::<i64>())
            .collect::<Result<_, _>>()?;

        ret.push(Equation::new(target, numbers));
    }
    Ok(ret)
}
