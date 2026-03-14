//! Implements a solution for https://adventofcode.com/2022/day/11

use std::{cmp, io::BufRead, mem, str::FromStr};

use crate::{answer, solutions::Solution};
use anyhow::{Context, anyhow, bail};

pub struct Day11;

impl<R: BufRead> Solution<R> for Day11 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut monkeys = read_input(input)?;
        run_rounds(&mut monkeys, 20, |n| n / 3);

        monkeys.sort_unstable_by_key(|m| cmp::Reverse(m.inspected));
        answer!(monkeys[0].inspected * monkeys[1].inspected)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut monkeys = read_input(input)?;

        let mut cap: i64 = 1;
        for m in &monkeys {
            cap *= m.test_divisible; // works. Ideally LCM
        }

        run_rounds(&mut monkeys, 10000, |n| (n + cap) % cap);

        monkeys.sort_unstable_by_key(|m| cmp::Reverse(m.inspected));
        answer!(monkeys[0].inspected * monkeys[1].inspected)
    }
}

fn run_rounds(monkeys: &mut [Monkey], rounds: usize, relief: impl Fn(i64) -> i64) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[i].items);
            monkeys[i].inspected += items.len();
            let (t, f) = (monkeys[i].target_if_true, monkeys[i].target_if_false);

            for item in items {
                let after = relief(monkeys[i].inspect(item));
                if after % monkeys[i].test_divisible == 0 {
                    monkeys[t].items.push(after);
                } else {
                    monkeys[f].items.push(after);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mul,
            "/" => Self::Div,
            s => bail!("invalid operator {:?}", s),
        })
    }
}

impl Operator {
    fn exec(self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Plus => a + b,
            Operator::Minus => a - b,
            Operator::Mul => a * b,
            Operator::Div => a / b,
        }
    }
}

#[derive(Debug)]
enum OpToken {
    Number(i64),
    Operator(Operator),
    OldVal,
}

impl FromStr for OpToken {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Self::OldVal)
        } else if let Ok(num) = s.parse::<i64>() {
            Ok(Self::Number(num))
        } else if let Ok(op) = s.parse::<Operator>() {
            Ok(Self::Operator(op))
        } else {
            bail!("parse failed: {:?}", s)
        }
    }
}

#[derive(Debug)]
struct Monkey {
    _id: usize,
    items: Vec<i64>,
    operation: Vec<OpToken>,
    test_divisible: i64,
    target_if_true: usize,
    target_if_false: usize,
    inspected: usize,
}

impl Monkey {
    fn inspect(&self, item: i64) -> i64 {
        let mut num: Option<i64> = None;
        let mut op: Option<Operator> = None;

        fn process_number(n: i64, op: &mut Option<Operator>, num: &mut Option<i64>) {
            if let Some(op_t) = *op {
                *num = Some(op_t.exec(num.unwrap(), n));
                *op = None;
            } else if num.replace(n).is_some() {
                panic!("invalid expression");
            }
        }
        fn process_operator(n: Operator, op: &mut Option<Operator>) {
            if op.replace(n).is_some() {
                panic!("invalid expression");
            }
        }

        for o in &self.operation {
            match o {
                OpToken::OldVal => process_number(item, &mut op, &mut num),
                OpToken::Number(n) => process_number(*n, &mut op, &mut num),
                OpToken::Operator(operator) => process_operator(*operator, &mut op),
            }
        }
        num.unwrap()
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<Monkey>, anyhow::Error> {
    let mut lines = input.lines();

    let mut read_monkey = || -> Result<Option<Monkey>, anyhow::Error> {
        let mut id: Option<usize> = None;
        let mut items: Option<Vec<i64>> = None;
        let mut operation: Option<Vec<OpToken>> = None;
        let mut test_divisible: Option<i64> = None;
        let mut target_if_true: Option<usize> = None;
        let mut target_if_false: Option<usize> = None;

        for line in lines.by_ref() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            let mut parts = line.splitn(2, ':').map(|s| s.trim());

            let k = parts.next().unwrap(); // splitn should yield at least one
            if let Some(id_str) = k.strip_prefix("Monkey ") {
                id = Some(id_str.parse().context("failed to parse id")?);
            } else {
                let v = parts
                    .next()
                    .with_context(|| anyhow!("no value specified for {:?}", k))?;
                match k {
                    "Starting items" => {
                        items = Some(
                            v.split(',')
                                .map(|s| s.trim().parse())
                                .collect::<Result<_, _>>()?,
                        );
                    }
                    "Operation" => {
                        let v = v.strip_prefix("new =").context("invalid operation")?;
                        operation = Some(
                            v.split_whitespace()
                                .map(|s| s.parse())
                                .collect::<Result<_, _>>()?,
                        )
                    }
                    "Test" => {
                        let v = v
                            .strip_prefix("divisible by ")
                            .context("invalid test specification")?;
                        test_divisible = Some(v.parse()?)
                    }
                    "If true" => {
                        let v = v
                            .strip_prefix("throw to monkey ")
                            .context("invalid test action")?;
                        target_if_true = Some(v.parse()?)
                    }
                    "If false" => {
                        let v = v
                            .strip_prefix("throw to monkey ")
                            .context("invalid test action")?;
                        target_if_false = Some(v.parse()?)
                    }
                    s => bail!("invalid property name {:?}", s),
                }
            }
        }

        Ok(match id {
            Some(id) => Some(Monkey {
                _id: id,
                items: items.with_context(|| anyhow!("items not defined for {}", id))?,
                operation: operation
                    .with_context(|| anyhow!("operation not defined for {}", id))?,
                test_divisible: test_divisible
                    .with_context(|| anyhow!("test not defined for {}", id))?,
                target_if_true: target_if_true
                    .with_context(|| anyhow!("target if true not defined for {}", id))?,
                target_if_false: target_if_false
                    .with_context(|| anyhow!("target if false not defined for {}", id))?,
                inspected: 0,
            }),
            None => None,
        })
    };

    let mut monkeys = Vec::<Monkey>::new();
    while let Some(m) = read_monkey()? {
        monkeys.push(m);
    }
    Ok(monkeys)
}
