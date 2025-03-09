use super::Solution;
use crate::solutions::InvalidInput;

use std::io::BufRead;

pub struct Day13;

impl<R: BufRead> Solution<R> for Day13 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let machines = read_input(input)?;

        let cost: i64 = machines
            .iter()
            .map(|m| match m.solve() {
                Some((a, b)) => a * 3 + b,
                None => 0,
            })
            .sum();

        Ok(cost.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let machines = read_input(input)?;

        let cost: i64 = machines
            .into_iter()
            .map(|mut m| {
                m.c1 += 10000000000000;
                m.c2 += 10000000000000;
                match m.solve() {
                    Some((a, b)) => a * 3 + b,
                    None => 0,
                }
            })
            .sum();

        Ok(cost.to_string())
    }
}

#[derive(Default, Debug)]
struct Machine {
    a1: i64,
    b1: i64,
    c1: i64,
    a2: i64,
    b2: i64,
    c2: i64,
}

impl Machine {
    fn solve(&self) -> Option<(i64, i64)> {
        let (a1, b1, c1) = (self.a1, self.b1, -self.c1);
        let (a2, b2, c2) = (self.a2, self.b2, -self.c2);

        // cross mult.
        let t = b2 * a1 - b1 * a2;
        if t == 0 {
            return None;
        }

        let x = (b1 * c2 - b2 * c1) / t;
        let y = (c1 * a2 - c2 * a1) / t;

        // verify
        if x < 0 || y < 0 || a1 * x + b1 * y + c1 != 0 || a2 * x + b2 * y + c2 != 0 {
            return None;
        }
        Some((x, y))
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<Machine>, anyhow::Error> {
    let mut ret = Vec::<Machine>::with_capacity(512);

    let mut current: Option<_> = None;
    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            if let Some(v) = current.take() {
                ret.push(v);
            }
            continue;
        }
        let current = current.get_or_insert_default();
        let parts = line
            .split_once(':')
            .ok_or_else(|| InvalidInput("invalid format".to_string()))?;

        match parts.0 {
            "Button A" => (current.a1, current.a2) = parse_nums(parts.1, "+")?,
            "Button B" => (current.b1, current.b2) = parse_nums(parts.1, "+")?,
            "Prize" => (current.c1, current.c2) = parse_nums(parts.1, "=")?,
            _ => return Err(InvalidInput(format!("\"{}\"", line)).into()),
        };
    }
    if let Some(v) = current.take() {
        ret.push(v);
    }

    Ok(ret)
}

fn parse_nums(s: &str, delimiter: &str) -> Result<(i64, i64), anyhow::Error> {
    let fmterr = || InvalidInput(format!("\"{}\"", s));
    let parts = s.split_once(',').ok_or_else(fmterr)?;
    let v1: i64 = parts
        .0
        .trim()
        .split_once(delimiter)
        .ok_or_else(fmterr)?
        .1
        .parse()?;
    let v2: i64 = parts
        .1
        .trim()
        .split_once(delimiter)
        .ok_or_else(fmterr)?
        .1
        .parse()?;
    Ok((v1, v2))
}
