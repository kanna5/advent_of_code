//! Implements a solution for https://adventofcode.com/2022/day/13

use std::{cmp::Ordering, io::BufRead};

use crate::{answer, solutions::Solution};
use anyhow::{Context, Ok, bail};

pub struct Day13;

impl<R: BufRead> Solution<R> for Day13 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let pairs = read_input(input)?;

        let sum = pairs
            .iter()
            .enumerate()
            .map(|(index, pair)| if pair.0 < pair.1 { index + 1 } else { 0 })
            .sum::<usize>();
        answer!(sum)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let pairs = read_input(input)?;

        let mut list = Vec::with_capacity(pairs.len() * 2 + 2);
        for p in &pairs {
            list.push((&p.0, false));
            list.push((&p.1, false));
        }
        let divider = (
            Data::List(vec![Data::List(vec![Data::Int(2)])]),
            Data::List(vec![Data::List(vec![Data::Int(6)])]),
        );
        list.push((&divider.0, true));
        list.push((&divider.1, true));

        list.sort_by(|a, b| a.0.cmp_internal(b.0));
        let mut result = 1;
        for (i, a) in list.iter().enumerate() {
            if a.1 {
                result *= i + 1
            }
        }
        answer!(result)
    }
}

#[derive(Debug)]
enum Data {
    Int(u8),
    List(Vec<Data>),
}

fn parse_list(input: &[u8]) -> anyhow::Result<(usize, Data)> {
    let mut p = 1;
    let mut elements = Vec::<Data>::new();

    while p < input.len() {
        match input[p] {
            b']' => return Ok((p + 1, Data::List(elements))),
            b',' | b' ' => p += 1,
            _ => {
                let (sz, data) = parse(&input[p..])?;
                elements.push(data);
                p += sz;
            }
        }
    }
    bail!("no matching ] found")
}

fn parse_int(input: &[u8]) -> anyhow::Result<(usize, Data)> {
    let mut num = input[0] - b'0';
    let mut p = 1;
    while p < input.len() && input[p].is_ascii_digit() {
        num = num * 10 + input[p] - b'0';
        p += 1;
    }
    Ok((p, Data::Int(num)))
}

fn parse(input: &[u8]) -> anyhow::Result<(usize, Data)> {
    if input.is_empty() {
        bail!("truncated input")
    }
    match input[0] {
        b'[' => parse_list(input),
        s if s.is_ascii_digit() => parse_int(input),
        s => bail!("invalid character {:?}", char::from(s)),
    }
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<(Data, Data)>> {
    let mut ret = Vec::with_capacity(128);
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let line = line?;
        let (_, d1) = parse(line.as_bytes())?;

        let l2 = lines.next().context("incomplete pair")??;
        let (_, d2) = parse(l2.as_bytes())?;
        ret.push((d1, d2));

        match lines.next() {
            Some(s) => {
                if !s?.is_empty() {
                    bail!("expected empty line after {:?}", l2)
                }
            }
            None => break,
        }
    }
    Ok(ret)
}

impl Data {
    fn cmp_internal(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Int(a), Data::Int(b)) => a.cmp(b),
            (Data::Int(a), Data::List(_)) => Data::List(vec![Data::Int(*a)]).cmp_internal(other),
            (Data::List(_), Data::Int(b)) => self.cmp_internal(&Data::List(vec![Data::Int(*b)])),
            (Data::List(a), Data::List(b)) => {
                let (mut a, mut b) = (a.iter(), b.iter());
                loop {
                    match (a.next(), b.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(a), Some(b)) => match a.cmp_internal(b) {
                            Ordering::Equal => continue,
                            s => return s,
                        },
                    }
                }
            }
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_internal(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_internal(other) == Ordering::Equal
    }
}
