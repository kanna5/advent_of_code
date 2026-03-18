//! Implements a solution for https://adventofcode.com/2022/day/16

use std::{
    cmp::min,
    collections::{HashMap, VecDeque, hash_map},
    io::BufRead,
};

use crate::{answer, solutions::Solution};
use anyhow::{Context, bail};

type ID = u16;
type Flowrates = HashMap<ID, i64>;
type Connections = HashMap<ID, Vec<ID>>;

pub struct Day16;

impl<R: BufRead> Solution<R> for Day16 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let time_cap = 30;
        let (mut flowrates, connections) = read_input(input)?;
        let ctx = SearchCtx {
            time_cap,
            distances: build_dist_matrix(&flowrates, &connections),
        };

        let flr: i64 = flowrates.iter().map(|kv| *kv.1).sum();
        let min_waste = ctx.find_min_waste(0, 0, flr, 0, flr * time_cap, &mut flowrates);
        answer!(flr * time_cap - min_waste)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let time_cap = 26;
        let (flowrates, connections) = read_input(input)?;
        let ctx = SearchCtx {
            time_cap,
            distances: build_dist_matrix(&flowrates, &connections),
        };

        let flr: i64 = flowrates.iter().map(|kv| *kv.1).sum();
        let mut min_waste = flr * time_cap;
        let valves: Vec<_> = flowrates.keys().copied().collect();

        for seed in 1..(1 << valves.len()) - 1 {
            let (mut a, mut b) = partition(seed, &valves, &flowrates);
            let flr_a: i64 = a.iter().map(|kv| *kv.1).sum();
            let flr_b: i64 = b.iter().map(|kv| *kv.1).sum();

            let w_a = ctx.find_min_waste(0, 0, flr_a, 0, min_waste, &mut a);
            let w_b = ctx.find_min_waste(0, 0, flr_b, 0, min_waste, &mut b);
            min_waste = min(min_waste, w_a + w_b);
        }

        answer!(flr * time_cap - min_waste)
    }
}

fn partition(seed: i64, valves: &[ID], flowrates: &Flowrates) -> (Flowrates, Flowrates) {
    let mut a = Flowrates::new();
    let mut b = Flowrates::new();

    for (offset, &id) in valves.iter().enumerate() {
        let flr = *flowrates.get(&id).unwrap();
        let mask = 1 << offset;
        match seed & mask == mask {
            true => a.insert(id, flr),
            false => b.insert(id, flr),
        };
    }
    (a, b)
}

fn build_dist_matrix(flowrates: &Flowrates, connections: &Connections) -> HashMap<ID, Vec<i64>> {
    let walk_from = |id: ID| -> Vec<i64> {
        let mut dists = vec![i64::MAX; connections.len()];
        let mut queue = VecDeque::<ID>::new();
        queue.push_back(id);
        dists[id as usize] = 0;

        while !queue.is_empty() {
            let c = queue.pop_front().unwrap();
            for &target in connections.get(&c).into_iter().flatten() {
                if dists[target as usize] > dists[c as usize] + 1 {
                    dists[target as usize] = dists[c as usize] + 1;
                    queue.push_back(target);
                }
            }
        }
        dists
    };

    let mut ret = HashMap::<ID, Vec<i64>>::new();
    ret.insert(0, walk_from(0)); // AA

    for &i in flowrates.keys() {
        ret.insert(i, walk_from(i));
    }
    ret
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<(Flowrates, Connections)> {
    let mut valves_idx: HashMap<String, ID> = HashMap::new();
    valves_idx.insert("AA".into(), 0);
    let mut next_id = 1;

    let mut get_or_register = |valve: String| -> ID {
        match valves_idx.entry(valve) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                e.insert(next_id);
                next_id += 1;
                next_id - 1
            }
        }
    };

    let mut flowrates = Flowrates::new();
    let mut connections = Connections::new();

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut it = line.split_ascii_whitespace().skip(1);
        let valve = it.next().context("invalid format")?;

        let mut it = it.skip(2);
        let flowrate: i64 = it
            .next()
            .context("expected flow rate")?
            .strip_prefix("rate=")
            .context("expected rate=")?
            .trim_end_matches(';')
            .parse()
            .context("error parsing flow rate")?;

        let targets: Vec<_> = it.skip(4).map(|s| s.trim_end_matches(',')).collect();
        if targets.is_empty() {
            bail!("tunnels lead to nowhere")
        }

        let valve_idx = get_or_register(valve.into());
        if flowrate > 0 {
            flowrates.insert(valve_idx, flowrate);
        }

        connections.insert(
            valve_idx,
            targets.iter().map(|&i| get_or_register(i.into())).collect(),
        );
    }
    Ok((flowrates, connections))
}

struct SearchCtx {
    time_cap: i64,
    distances: HashMap<ID, Vec<i64>>,
}

impl SearchCtx {
    fn find_min_waste(
        &self,
        cur_loc: ID,
        elapsed: i64,
        fl_closed: i64, // total flow rate of closed valves
        wasted: i64,
        waste_cap: i64,
        flowrates: &mut Flowrates,
    ) -> i64 {
        let openable: Vec<_> = flowrates.keys().copied().collect();
        let dists = self.distances.get(&cur_loc).unwrap();
        let mut min_waste = min(waste_cap, wasted + fl_closed * (self.time_cap - elapsed));

        for valve in openable {
            let time_cost = dists[valve as usize] + 1; // walk + open
            if elapsed + time_cost >= self.time_cap {
                continue;
            }

            let w = fl_closed * time_cost;
            if w + wasted >= waste_cap {
                continue;
            }

            let rate = flowrates.remove(&valve).unwrap();
            let mw = self.find_min_waste(
                valve,
                elapsed + time_cost,
                fl_closed - rate,
                wasted + w,
                min_waste,
                flowrates,
            );
            min_waste = min(min_waste, mw);

            flowrates.insert(valve, rate);
        }
        min_waste
    }
}
