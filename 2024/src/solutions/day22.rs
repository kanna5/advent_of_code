use super::Solution;
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    io::BufRead,
};

pub struct Day22;

impl<R: BufRead> Solution<R> for Day22 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let init_secs = read_input(input)?;
        let sum: u64 = init_secs
            .iter()
            .map(|n| {
                let mut b = Buyer { secret: *n };
                b.nth(1999).unwrap()
            })
            .sum();
        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let init_secs = read_input(input)?;
        let mut max_profit: HashMap<PriceChanges, u64> = HashMap::with_capacity(2048);

        let mut local_profit: HashMap<PriceChanges, i8> = HashMap::with_capacity(2048);
        let mut price_changes = VecDeque::<i8>::with_capacity(5);
        for secret in init_secs {
            local_profit.clear();
            price_changes.clear();
            let mut prev_price = (secret % 10) as i8;
            let mut monkey = Buyer { secret };

            for _ in 0..2000 {
                let next_price = (monkey.next().unwrap() % 10) as i8;
                let change = next_price - prev_price;
                prev_price = next_price;
                price_changes.push_back(change);

                if price_changes.len() > 4 {
                    price_changes.pop_front();
                }
                if price_changes.len() == 4 {
                    let idx = price_changes.iter().collect::<PriceChanges>();
                    let Entry::Vacant(ent) = local_profit.entry(idx) else {
                        continue;
                    };
                    ent.insert(next_price);
                }
            }

            for (k, v) in local_profit.iter() {
                let global = max_profit.entry(k.clone()).or_default();
                *global += *v as u64;
            }
        }

        let max = max_profit.iter().map(|v| *v.1).max().unwrap();
        Ok(max.to_string())
    }
}

const PRUNE_TARGET: u64 = 16777216;

struct Buyer {
    secret: u64,
}

impl Iterator for Buyer {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = ((self.secret << 6) ^ self.secret).rem_euclid(PRUNE_TARGET);
        next = ((next >> 5) ^ next).rem_euclid(PRUNE_TARGET);
        next = ((next << 11) ^ next).rem_euclid(PRUNE_TARGET);
        self.secret = next;
        Some(next)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PriceChanges(i8, i8, i8, i8);

impl<'a> FromIterator<&'a i8> for PriceChanges {
    fn from_iter<T: IntoIterator<Item = &'a i8>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();
        let d = iter.next().unwrap();
        Self(*a, *b, *c, *d)
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<u64>, anyhow::Error> {
    Ok(input
        .lines()
        .map_while(Result::ok)
        .take_while(|l| !l.is_empty())
        .map(|l| l.parse::<u64>())
        .collect::<Result<_, _>>()?)
}
