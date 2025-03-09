use super::{InvalidInput, Solution};
use std::io::BufRead;

pub struct Day2;

impl<R: BufRead> Solution<R> for Day2 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let lines = input.lines();
        let mut cnt: usize = 0;
        for line in lines.map_while(Result::ok).filter(|l| !l.is_empty()) {
            let report = parse_line(&line)?;
            if eval_safety(&report) {
                cnt += 1
            }
        }
        Ok(cnt.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let lines = input.lines();
        let mut cnt: usize = 0;
        for line in lines.map_while(Result::ok).filter(|l| !l.is_empty()) {
            let report = parse_line(&line)?;
            if eval_safety(&report) {
                cnt += 1;
                continue;
            }
            for i in 0..report.len() {
                let dampened = omit(&report, i);
                if eval_safety(&dampened) {
                    cnt += 1;
                    break;
                }
            }
        }
        Ok(cnt.to_string())
    }
}

fn parse_line(line: &str) -> Result<Vec<i64>, InvalidInput> {
    line.split_whitespace()
        .map(|i| i.parse::<i64>().map_err(|_| InvalidInput(line.into())))
        .collect()
}

fn omit(v: &[i64], i: usize) -> Vec<i64> {
    v.iter()
        .enumerate()
        .filter(|&(ii, _)| ii != i)
        .map(|(_, &v)| v)
        .collect()
}

fn eval_safety(report: &[i64]) -> bool {
    let mut is_inc: Option<bool> = None;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        let inc = diff > 0;
        match is_inc {
            None => is_inc = Some(inc),
            Some(old_inc) if old_inc != inc => return false,
            _ => (),
        }
    }
    true
}
