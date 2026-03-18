//! Implements a solution for https://adventofcode.com/2022/day/21
//!
//! Spoiler: after examining the input, it's a binary tree.

use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    str::FromStr,
};

use crate::{answer, solutions::Solution};
use anyhow::{Context, bail};

pub struct Day21;

impl<R: BufRead> Solution<R> for Day21 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let monkeys = read_input(input)?;

        let mut by_name: HashMap<String, &Monkey> = HashMap::with_capacity(monkeys.len());
        let mut parent: HashMap<String, String> = HashMap::with_capacity(monkeys.len());

        for m in &monkeys {
            if let MJob::Expr(l, _, r) = &m.job {
                for child in [l, r] {
                    if parent.insert(child.clone(), m.name.clone()).is_some() {
                        bail!("Invalid input structure. expected a binary tree");
                    };
                }
            }
            by_name.insert(m.name.clone(), m);
        }

        answer!(
            calc_node_value(&by_name, "root")
                .context("Invalid input structure. expected a binary tree")?
        )
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let monkeys = read_input(input)?;

        let mut by_name: HashMap<String, &Monkey> = HashMap::with_capacity(monkeys.len());
        let mut parent: HashMap<String, String> = HashMap::with_capacity(monkeys.len());

        for m in &monkeys {
            if let MJob::Expr(l, _, r) = &m.job {
                for child in [l, r] {
                    if parent.insert(child.clone(), m.name.clone()).is_some() {
                        bail!("Invalid input structure. expected a binary tree");
                    };
                }
            }
            by_name.insert(m.name.clone(), m);
        }

        let mut unknown = HashSet::new();
        let mut u = "humn";
        while u != "root" {
            unknown.insert(u);
            u = parent
                .get(u)
                .with_context(|| format!("expect {:?} to have a parent", u))?;
        }

        let root_node = *by_name.get("root").context("missing root node")?;
        let (with_human, without_human) = match &root_node.job {
            MJob::Num(_) => bail!("expect root node to be an expression, got number"),
            MJob::Expr(l, _, r) => {
                if unknown.contains(l as &str) {
                    (l, r)
                } else {
                    (r, l)
                }
            }
        };

        let val = calc_node_value(&by_name, without_human)
            .with_context(|| format!("cannot calculate value of node {:?}", without_human))?;

        let human = find_human(&by_name, &unknown, with_human, val)
            .context("cannot calculate humn value")?;
        answer!(human)
    }
}

fn calc_node_value(by_name: &HashMap<String, &Monkey>, node: &str) -> Option<i64> {
    let node = *by_name.get(node)?;
    Some(match &node.job {
        MJob::Num(v) => *v,
        MJob::Expr(l, op, r) => op.calc(calc_node_value(by_name, l)?, calc_node_value(by_name, r)?),
    })
}

fn find_human(
    by_name: &HashMap<String, &Monkey>,
    unknown: &HashSet<&str>,
    node: &str,
    target_val: i64,
) -> Option<i64> {
    if node == "humn" {
        return Some(target_val);
    }

    let node = *by_name.get(node)?;
    let (l, op, r) = match &node.job {
        MJob::Num(_) => return None,
        MJob::Expr(l, op, r) => (l, op, r),
    };

    let (next_node, next_target) = if unknown.contains(l as &str) {
        let known = calc_node_value(by_name, r)?;
        (l, op.rev().calc(target_val, known))
    } else if unknown.contains(r as &str) {
        let known = calc_node_value(by_name, l)?;
        let new_tgt = match op {
            Op::Add => Op::Sub.calc(target_val, known),
            Op::Sub => Op::Sub.calc(known, target_val),
            Op::Mul => Op::Div.calc(target_val, known),
            Op::Div => Op::Div.calc(known, target_val),
        };
        (r, new_tgt)
    } else {
        return None;
    };

    find_human(by_name, unknown, next_node, next_target)
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: MJob,
}

#[derive(Debug, Clone)]
enum MJob {
    Num(i64),
    Expr(String, Op, String),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn calc(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }

    fn rev(self) -> Self {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            s => bail!("invalid operator {:?}", s),
        })
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, job_str) = s.split_once(':').context("expected ':'")?;
        let name = name.trim_ascii();
        let job_str = job_str.trim_ascii();

        let mut iter = job_str.split_ascii_whitespace();
        let lhs = iter.next().context("expected number or expression")?;

        if let Ok(n) = lhs.parse::<i64>() {
            return Ok(Monkey {
                name: name.into(),
                job: MJob::Num(n),
            });
        }

        let op: Op = iter.next().context("expected operator")?.parse()?;
        let rhs = iter.next().context("expected rhs")?;

        if let Some(extra) = iter.next() {
            bail!("invalid content {:?}", extra);
        }
        Ok(Monkey {
            name: name.into(),
            job: MJob::Expr(lhs.into(), op, rhs.into()),
        })
    }
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Monkey>> {
    input
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            let line = line?;
            line.parse::<Monkey>()
                .with_context(|| format!("parsing line {}", lineno + 1))
        })
        .collect()
}
