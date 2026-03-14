//! Implements a solution for https://adventofcode.com/2022/day/6

use std::io::BufRead;

use crate::solutions::Solution;

pub struct Day06;

impl<R: BufRead> Solution<R> for Day06 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut buf = String::new();
        input.read_line(&mut buf)?;

        Ok(find_marker(&buf, 4).to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut buf = String::new();
        input.read_line(&mut buf)?;

        Ok(find_marker(&buf, 14).to_string())
    }
}

fn find_marker(input: &str, req_distinct: usize) -> isize {
    let buf = input.as_bytes();
    let mut cnts = [0u8; 256];
    let mut distinct = 0;
    for i in 0..buf.len() {
        let c = buf[i];
        cnts[c as usize] += 1;
        if cnts[c as usize] == 1 {
            distinct += 1
        }

        if i >= req_distinct {
            let c = buf[i - req_distinct];
            cnts[c as usize] -= 1;
            if cnts[c as usize] == 0 {
                distinct -= 1
            }
        }

        if distinct == req_distinct {
            return (i + 1) as isize;
        }
    }
    -1
}
