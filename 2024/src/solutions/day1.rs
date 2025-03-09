use crate::solutions::{InvalidInput, Solution};
use std::{collections::HashMap, io::BufRead};

pub struct Day1;

impl<R: BufRead> Solution<R> for Day1 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut left: Vec<i64> = Vec::with_capacity(512);
        let mut right: Vec<i64> = Vec::with_capacity(512);

        let lines = input.lines();
        for line in lines.map_while(Result::ok) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            left.push(
                parts
                    .first()
                    .ok_or_else(|| InvalidInput(line.clone()))?
                    .parse::<i64>()?,
            );
            right.push(
                parts
                    .get(1)
                    .ok_or_else(|| InvalidInput(line.clone()))?
                    .parse::<i64>()?,
            );
        }

        left.sort();
        right.sort();

        let mut total_diff: i64 = 0;
        for i in 0..left.len() {
            total_diff += (left[i] - right[i]).abs()
        }

        Ok(total_diff.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut left: Vec<i64> = Vec::with_capacity(512);
        let mut right: HashMap<i64, usize> = HashMap::new();

        let lines = input.lines();
        for line in lines.map_while(Result::ok) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let num1 = parts
                .first()
                .ok_or_else(|| InvalidInput(line.clone()))?
                .parse::<i64>()?;
            let num2 = parts
                .get(1)
                .ok_or_else(|| InvalidInput(line.clone()))?
                .parse::<i64>()?;

            left.push(num1);
            *(right.entry(num2).or_default()) += 1;
        }

        let mut similarity: i64 = 0;
        for i in left.iter() {
            let freq = right.get(i).cloned().unwrap_or_default();
            similarity += *i * freq as i64
        }

        Ok(similarity.to_string())
    }
}
