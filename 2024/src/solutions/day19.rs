use super::{InvalidInput, Solution};
use std::io::BufRead;

pub struct Day19;

impl<R: BufRead> Solution<R> for Day19 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let onsen = read_input(input)?;
        let cnt: usize = onsen
            .designs
            .iter()
            .map(|d| is_possible(&onsen.towels[..], d) as usize)
            .sum();
        Ok(cnt.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let onsen = read_input(input)?;
        let sum: usize = onsen
            .designs
            .iter()
            .map(|d| ways(&onsen.towels[..], d))
            .sum();
        Ok(sum.to_string())
    }
}

#[derive(Debug)]
struct Onsen {
    towels: Vec<String>,
    designs: Vec<String>,
}

fn is_possible(patterns: &[String], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    patterns
        .iter()
        .filter(|&p| design.starts_with(p))
        .any(|p| is_possible(patterns, &design[p.len()..]))
}

fn ways(patterns: &[String], design: &str) -> usize {
    let mut mem = vec![0usize; design.len() + 1];
    mem[0] = 1;

    for i in 1..=design.len() {
        mem[i] = patterns
            .iter()
            .map(|p| match design[0..i].ends_with(p) {
                true => mem[i - p.len()],
                false => 0,
            })
            .sum();
    }
    mem[design.len()]
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Onsen, anyhow::Error> {
    let line = input
        .lines()
        .next()
        .ok_or_else(|| InvalidInput("No input".to_string()))??;
    _ = input.lines().next();

    let towels = line
        .split(',')
        .map(|v| v.trim().to_string())
        .collect::<Vec<_>>();

    let designs = input
        .lines()
        .map_while(Result::ok)
        .take_while(|v| !v.is_empty())
        .collect();

    Ok(Onsen { towels, designs })
}
