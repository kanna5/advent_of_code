use super::Solution;
use core::str;
use std::{collections::HashMap, io::BufRead};

pub struct Day11;

impl<R: BufRead> Solution<R> for Day11 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let stones = read_input(input)?;
        let mut mem = Memory::with_capacity(4096);

        let sum: u64 = stones.iter().map(|st| st.count(25, &mut mem)).sum();
        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let stones = read_input(input)?;
        let mut mem = Memory::with_capacity(4096);

        let sum: u64 = stones.iter().map(|st| st.count(75, &mut mem)).sum();
        Ok(sum.to_string())
    }
}

struct Num(u64);
type MemKey = (u64, u8);
type Memory = HashMap<MemKey, u64>;

impl Num {
    fn transform(&self) -> (Self, Option<Self>) {
        match self.0 {
            0 => (Self(1), None),
            v => {
                let num_digits = v.ilog10() + 1;
                if num_digits % 2 != 0 {
                    return (Self(v * 2024), None);
                }
                let divider = 10u64.pow(num_digits / 2);
                (Self(v / divider), Some(Self(v % divider)))
            }
        }
    }

    fn count(&self, blinks: u8, mem: &mut Memory) -> u64 {
        if blinks == 0 {
            return 1;
        }

        // query mem
        if let Some(cnt) = mem.get(&(self.0, blinks)) {
            return *cnt;
        }

        // actually do slplit
        let (a, b) = self.transform();
        let mut cnt = a.count(blinks - 1, mem);
        if let Some(b) = b {
            cnt += b.count(blinks - 1, mem)
        }

        // remember and return
        mem.insert((self.0, blinks), cnt);
        cnt
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<Num>, anyhow::Error> {
    let mut ret = Vec::<Num>::with_capacity(1024);
    let mut buf = Vec::<u8>::with_capacity(4096);
    while let Ok(v) = input.read_until(b' ', &mut buf) {
        if v == 0 {
            break;
        }
        let s = str::from_utf8(&buf)?;
        let num: u64 = s.trim().parse()?;
        ret.push(Num(num));
        buf.clear();
    }
    Ok(ret)
}
