//! Implements a solution for https://adventofcode.com/2022/day/19

use std::{
    io::BufRead,
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::{answer, solutions::Solution};
use anyhow::{Context, bail};

pub struct Day19;

impl<R: BufRead> Solution<R> for Day19 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let blueprints = read_input(input)?;

        let mut sum_qual = 0;
        for blp in &blueprints {
            let g = max_geode(Scene::new(24), blp, 0);
            sum_qual += blp.id * g;
        }
        answer!(sum_qual)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let blueprints = read_input(input)?;

        let mut mul = 1;
        for blp in blueprints.iter().take(3) {
            mul *= max_geode(Scene::new(32), blp, 0);
        }
        answer!(mul)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

use Resource::*;
const ALL_RESOURCES: [Resource; 4] = [Ore, Clay, Obsidian, Geode];

impl<T> Index<Resource> for [T; 4] {
    type Output = T;

    fn index(&self, index: Resource) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Resource> for [T; 4] {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Debug, Clone, Copy)]
struct Scene {
    time_limit: usize,
    elapsed: usize,
    robots: [usize; 4],
    resources: [usize; 4],
}

impl Scene {
    fn new(time_limit: usize) -> Self {
        Self {
            time_limit,
            elapsed: 0,
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        }
    }

    fn can_produce(&self, res: Resource) -> bool {
        self.robots[res] > 0
    }

    fn elapse(&self, time: usize) -> Self {
        let mut ret = *self;
        for res in ALL_RESOURCES {
            ret.resources[res] += time * ret.robots[res]
        }
        ret.elapsed += time;
        ret
    }

    fn time_to_produce(&self, resource: Resource, amount: usize) -> usize {
        let (n_robots, cur_amount) = (self.robots[resource], self.resources[resource]);
        match amount.saturating_sub(cur_amount) {
            0 => 0,
            needed => needed.div_ceil(n_robots),
        }
    }

    fn make_robot(&self, r_type: Resource, blp: &Blueprint) -> Option<Self> {
        let cost = blp.costs[r_type];

        let mut time_needed = 0;
        for res in ALL_RESOURCES {
            let res_cost = cost[res];
            if res_cost > 0 && !self.can_produce(res) {
                return None;
            }
            time_needed = time_needed.max(self.time_to_produce(res, res_cost) + 1)
        }
        if time_needed + self.elapsed > self.time_limit {
            return None;
        }

        let mut new_state = self.elapse(time_needed);
        for (r, c) in new_state.resources.iter_mut().zip(cost) {
            *r -= c
        }
        new_state.robots[r_type] += 1;
        Some(new_state)
    }

    /// Theoretical limit of how many geodes it can produce
    fn potential(&self) -> usize {
        let remaining_time = self.time_limit - self.elapsed;
        let limit = self.resources[Geode] + self.robots[Geode] * remaining_time;
        // Imagine producing 1 geode robot every minute
        limit + remaining_time.saturating_sub(1) * remaining_time / 2
    }
}

fn max_geode(scene: Scene, blueprint: &Blueprint, best: usize) -> usize {
    if scene.potential() <= best {
        return 0;
    }
    let mut max_g =
        scene.resources[Geode] + (scene.time_limit - scene.elapsed) * scene.robots[Geode];

    for typ in [Geode, Obsidian, Clay, Ore] {
        if typ != Geode {
            let max_need = blueprint.costs.iter().map(|c| c[typ]).max().unwrap();
            if scene.robots[typ] >= max_need {
                // More than we can consume, since the factory can only make one robot per minute.
                continue;
            }
        }

        if let Some(next_scene) = scene.make_robot(typ, blueprint) {
            max_g = max_g.max(max_geode(next_scene, blueprint, best.max(max_g)));
        }
    }
    max_g
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: usize,
    costs: [[usize; 4]; 4], // [robot_type][resource_type]
}

fn parse_costs(costs: &str) -> anyhow::Result<[usize; 4]> {
    let mut result = [0; 4];
    for part in costs.split(" and ") {
        let (num, res) = part
            .trim_ascii()
            .split_once(' ')
            .context("invalid cost format")?;
        let num: usize = num.parse()?;

        match res.trim_ascii() {
            "ore" => result[Ore] = num,
            "clay" => result[Clay] = num,
            "obsidian" => result[Obsidian] = num,
            s => bail!("invalid resource name {:?}", s),
        };
    }
    Ok(result)
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_general = "invalid format";

        let s = s.strip_prefix("Blueprint ").context(err_general)?;
        let (id, s) = s.split_once(':').context(err_general)?;
        let id: usize = id.parse().context("parse ID")?;

        let mut costs_parts = s.split('.');
        let mut extract_costs = |prefix: &str| {
            parse_costs(
                costs_parts
                    .next()
                    .context(err_general)?
                    .trim_ascii()
                    .strip_prefix(prefix)
                    .context(err_general)?,
            )
        };

        let ore_robot = extract_costs("Each ore robot costs ")?;
        let clay_robot = extract_costs("Each clay robot costs ")?;
        let obsidian_robot = extract_costs("Each obsidian robot costs ")?;
        let geode_robot = extract_costs("Each geode robot costs ")?;

        Ok(Self {
            id,
            costs: [ore_robot, clay_robot, obsidian_robot, geode_robot],
        })
    }
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Blueprint>> {
    let mut ret = Vec::new();

    for (lineno, line) in input.lines().enumerate() {
        let line = line?;
        if line.trim_ascii().is_empty() {
            break;
        }
        ret.push(
            line.parse()
                .with_context(|| format!("parsing line {}", lineno + 1))?,
        );
    }
    Ok(ret)
}
