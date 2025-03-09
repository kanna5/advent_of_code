use super::Solution;
use anyhow::Context;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

/// Note: all page numbers in the input are covered by at least one rule.
pub struct Day5;

impl<R: BufRead> Solution<R> for Day5 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let rules = read_rules(input)?;
        let updates = read_updates(input)?;

        let mut sum = 0;
        for update in &updates {
            if is_valid(update, &rules) {
                sum += update[update.len() / 2]
            }
        }
        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let rules = read_rules(input)?;
        let updates = read_updates(input)?;

        let mut sum = 0;
        for update in &updates {
            if is_valid(update, &rules) {
                continue;
            }
            let r = reorder(update, &rules);
            sum += r[r.len() / 2];
        }
        Ok(sum.to_string())
    }
}

fn reorder(update: &Vec<u32>, rules: &HashMap<u32, Ruleset>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::with_capacity(update.len());
    let mut visited: HashSet<u32> = HashSet::with_capacity(update.len());
    let relevant: HashSet<u32> = HashSet::from_iter(update.iter().cloned());

    fn ensure_rule(
        page: &u32,
        rules: &HashMap<u32, Ruleset>,
        ret: &mut Vec<u32>,
        visited: &mut HashSet<u32>,
        relevant: &HashSet<u32>,
    ) {
        if visited.contains(page) || !relevant.contains(page) {
            return;
        }
        visited.insert(*page);

        if let Some(rule) = rules.get(page) {
            for a in &rule.after {
                ensure_rule(a, rules, ret, visited, relevant)
            }
        }
        ret.push(*page);
    }

    for p in update {
        ensure_rule(p, rules, &mut ret, &mut visited, &relevant);
    }
    ret
}

fn is_valid(update: &[u32], rules: &HashMap<u32, Ruleset>) -> bool {
    let mut active_rules: HashMap<u32, (usize, &Ruleset)> = HashMap::new();
    for (pos, page) in update.iter().enumerate() {
        if let Some(rule) = rules.get(page) {
            active_rules.insert(*page, (pos, rule));
        }
    }
    for (a_pos, rule) in active_rules.values() {
        for b in &rule.before {
            if let Some((b_pos, _)) = active_rules.get(b) {
                if b_pos < a_pos {
                    return false;
                }
            }
        }
    }
    true
}

#[derive(Default, Debug)]
struct Ruleset {
    before: HashSet<u32>,
    after: HashSet<u32>,
}

fn read_rules<R: BufRead>(input: &mut R) -> Result<HashMap<u32, Ruleset>, anyhow::Error> {
    let mut ret: HashMap<u32, Ruleset> = HashMap::new();

    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let (a, b) = line
            .split_once("|")
            .with_context(|| format!("Invalid input: {}", line))?;
        let a = a.parse::<u32>()?;
        let b = b.parse::<u32>()?;

        // a must be before b
        ret.entry(a).or_default().before.insert(b);
        ret.entry(b).or_default().after.insert(a);
    }
    Ok(ret)
}

fn read_updates<R: BufRead>(input: &mut R) -> Result<Vec<Vec<u32>>, anyhow::Error> {
    let mut ret: Vec<Vec<u32>> = Vec::new();

    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        ret.push(
            line.split(',')
                .map(|n| n.parse())
                .collect::<Result<_, _>>()?,
        );
    }

    Ok(ret)
}
