//! Implements a solution for https://adventofcode.com/2022/day/10

use std::io::BufRead;

use crate::{answer, solutions::Solution};
use anyhow::{Context, anyhow};

pub struct Day10;

impl<R: BufRead> Solution<R> for Day10 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let instructions = read_input(input)?;

        let mut x = 1_i64;
        let mut sum = 0_i64;
        let mut cycle = 0;
        for inst in instructions {
            match inst {
                Instruction::Noop => {
                    cycle += 1;
                    if (cycle + 20) % 40 == 0 {
                        sum += cycle as i64 * x
                    }
                }
                Instruction::Addx(n) => {
                    for _ in 0..2 {
                        cycle += 1;
                        if (cycle + 20) % 40 == 0 {
                            sum += cycle as i64 * x
                        }
                    }
                    x += n
                }
            }
            if cycle >= 220 {
                break;
            }
        }
        answer!(sum)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let instructions = read_input(input)?;
        let mut crt = vec!['.'; 40 * 6];

        let mut paint = |cycle: i64, sprite: i64| {
            let cursor = (cycle - 1) % 40;
            if (sprite - 1..=sprite + 1).contains(&cursor) {
                crt[(cycle - 1) as usize] = '█' // Unicode full block for better clarity
            }
        };

        let mut x = 1_i64;
        let mut cycle = 0;
        for inst in instructions {
            match inst {
                Instruction::Noop => {
                    cycle += 1;
                    paint(cycle, x);
                }
                Instruction::Addx(n) => {
                    for _ in 0..2 {
                        cycle += 1;
                        paint(cycle, x);
                    }
                    x += n
                }
            }
            if cycle >= 240 {
                break;
            }
        }

        let mut display = String::with_capacity(41 * 6);
        for (i, c) in crt.iter().enumerate() {
            if i > 0 && i % 40 == 0 {
                display.push('\n');
            }
            display.push(*c);
        }
        answer!(display)
    }
}

enum Instruction {
    Noop,
    Addx(i64),
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<Instruction>, anyhow::Error> {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(128);

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut parts = line.split_ascii_whitespace();
        let inst = parts
            .next()
            .with_context(|| anyhow!("invalid line {:?}", line))?;

        instructions.push(match inst {
            "noop" => Instruction::Noop,
            "addx" => {
                let num: i64 = parts
                    .next()
                    .with_context(|| anyhow!("missing operand: {:?}", line))?
                    .parse()
                    .with_context(|| anyhow!("invalid number in line {:?}", line))?;
                Instruction::Addx(num)
            }
            s => Err(anyhow!("invalid instruction {:?}", s))?,
        });
    }
    Ok(instructions)
}
