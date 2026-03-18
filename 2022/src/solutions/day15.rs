//! Implements a solution for https://adventofcode.com/2022/day/15

use std::{
    cmp::{max, min},
    collections::HashSet,
    io::BufRead,
};

use crate::{
    answer,
    solutions::{Options, Solution},
};
use anyhow::{Context, bail};

pub struct Day15 {
    pub opts: Options,
}

impl<R: BufRead> Solution<R> for Day15 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let sensors = read_input(input)?;
        let sensors_dist = sensors.iter().map(|(s, b)| (s, s.dist(b)));

        let y: i64 = self.opts.try_get("y", 2_000_000)?;

        let mut coverages: Vec<_> = sensors_dist
            .filter_map(|(sensor, dist)| {
                let dy = (y - sensor.1).abs();
                if dy > dist {
                    None
                } else {
                    let d = dist - dy;
                    Some((sensor.0 - d, sensor.0 + d))
                }
            })
            .collect();
        coverages.sort_by_key(|c| c.0);

        let mut cov_len: i64 = 0;
        let mut last_r = i64::MIN;
        for (l, r) in coverages {
            cov_len += max(0, r - max(l - 1, last_r));
            last_r = max(last_r, r)
        }

        let n_known_beacons = sensors
            .iter()
            .filter_map(|(_, b)| if b.1 == y { Some(b.0) } else { None })
            .collect::<HashSet<_>>()
            .len() as i64;

        answer!(cov_len - n_known_beacons)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let sensors = read_input(input)?;
        let sensors_dist: Vec<_> = sensors.iter().map(|(s, b)| (s, s.dist(b))).collect();

        let coord_max: i64 = self.opts.try_get("coord_max", 4_000_000)?;
        let tuning_freq_mul: i64 = self.opts.try_get("tuning_freq_mul", 4_000_000)?;

        let is_included = |c: &Coord| -> bool { sensors_dist.iter().any(|(s, d)| s.dist(c) <= *d) };

        for (s, d) in &sensors_dist {
            let test_dist = d + 1;
            for x in max(s.0 - test_dist, 0)..=min(s.0 + test_dist, coord_max) {
                let dy = test_dist - (x - s.0).abs();
                let y = s.1 - dy;
                if y >= 0 && !is_included(&Coord(x, y)) {
                    return answer!(x * tuning_freq_mul + y);
                }
                let y = s.1 + dy;
                if y <= coord_max && !is_included(&Coord(x, y)) {
                    return answer!(x * tuning_freq_mul + y);
                }
            }
        }
        answer!(-1)
    }
}

struct Coord(i64, i64);

impl Coord {
    fn dist(&self, another: &Self) -> i64 {
        (self.0 - another.0).abs() + (self.1 - another.1).abs()
    }
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<(Coord, Coord)>> {
    let mut ret: Vec<(Coord, Coord)> = Vec::with_capacity(128);

    for (i, line) in input.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let lineno = i + 1;

        let parts: Vec<_> = line.split(['=', ',', ':']).collect();
        if parts.len() != 8 {
            bail!("cannot parse line: {}: invalid format", lineno)
        }

        let s_x: i64 = parts[1]
            .parse()
            .with_context(|| format!("line {}: cannot parse sensor_x", lineno))?;
        let s_y: i64 = parts[3]
            .parse()
            .with_context(|| format!("line {}: cannot parse sensor_y", lineno))?;
        let b_x: i64 = parts[5]
            .parse()
            .with_context(|| format!("line {}: cannot parse beacon_x", lineno))?;
        let b_y: i64 = parts[7]
            .parse()
            .with_context(|| format!("line {}: cannot parse beacon_y", lineno))?;

        ret.push((Coord(s_x, s_y), Coord(b_x, b_y)));
    }
    Ok(ret)
}
