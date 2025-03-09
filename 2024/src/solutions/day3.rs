use super::Solution;
use std::io::BufRead;

pub struct Day3;

impl<R: BufRead> Solution<R> for Day3 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut mem: String = String::with_capacity(4096);
        input.read_to_string(&mut mem)?;

        let pattern = regex::Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").unwrap();
        let mut sum = 0;
        for (_, [a, b]) in pattern.captures_iter(&mem).map(|c| c.extract()) {
            sum += a.parse::<i32>()? * b.parse::<i32>()?
        }

        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut mem: String = String::with_capacity(4096);
        input.read_to_string(&mut mem)?;

        let pattern =
            regex::Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)|do(n't)?\(\)").unwrap();
        let mut sum = 0;

        let mut enabled = true;
        for c in pattern.captures_iter(&mem) {
            match &c[0] {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ if enabled => sum += c[1].parse::<i32>()? * c[2].parse::<i32>()?,
                _ => (),
            }
        }

        Ok(sum.to_string())
    }
}
