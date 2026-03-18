//! Implements a solution for https://adventofcode.com/2022/day/23

use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    io::BufRead,
};

use crate::{answer, solutions::Solution};
use anyhow::bail;

pub struct Day23;

impl<R: BufRead> Solution<R> for Day23 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut sim = Sim::new(read_input(input)?);
        for _ in 0..10 {
            sim.run();
        }

        let (cmin, cmax) = bounding_rect(&sim.elves);
        answer!((cmax.0 - cmin.0 + 1) * (cmax.1 - cmin.1 + 1) - sim.elves.len() as isize)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut sim = Sim::new(read_input(input)?);
        loop {
            if sim.run() == 0 {
                return answer!(sim.rounds);
            }
        }
    }
}

struct Sim {
    elves: Vec<Elf>,

    coords: HashSet<Coord>,
    proposals: HashMap<Coord, Option<usize>>,
    rounds: usize,
}

impl Sim {
    fn new(elves: Vec<Elf>) -> Self {
        Self {
            coords: elves.iter().map(|e| (e.x, e.y)).collect(),
            proposals: HashMap::with_capacity(elves.len()),
            elves,
            rounds: 0,
        }
    }

    fn run(&mut self) -> usize {
        let mut moved = 0;
        self.proposals.clear();

        for (i, e) in self.elves.iter().enumerate() {
            let (dx, dy) = match e.decide_move(self.rounds, &self.coords) {
                Some(val) => val.vec(),
                None => continue,
            };

            let target = (e.x + dx, e.y + dy);
            match self.proposals.entry(target) {
                Entry::Occupied(mut ent) => &ent.insert(None),
                Entry::Vacant(ent) => ent.insert(Some(i)),
            };
        }

        for (&coord, id) in self.proposals.iter().filter_map(|(k, v)| Some((k, (*v)?))) {
            let elf = &mut self.elves[id];
            let (old_c, new_c) = ((elf.x, elf.y), coord);
            (elf.x, elf.y) = new_c;

            self.coords.remove(&old_c);
            self.coords.insert(new_c);
            moved += 1
        }
        self.rounds += 1;
        moved
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

type Coord = (isize, isize);

impl Dir {
    fn vec(self) -> Coord {
        match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        }
    }
}

const CONSIDERATIONS: [Dir; 4] = [Dir::N, Dir::S, Dir::W, Dir::E];

#[derive(Debug, Clone)]
struct Elf {
    x: isize,
    y: isize,
}

impl Elf {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn decide_move(&self, round: usize, coords: &HashSet<Coord>) -> Option<Dir> {
        let mut options = [true; 4];

        const DIRS: &[(Coord, &[Dir])] = &[
            ((0, -1), &[Dir::N]),
            ((1, -1), &[Dir::N, Dir::E]),
            ((1, 0), &[Dir::E]),
            ((1, 1), &[Dir::E, Dir::S]),
            ((0, 1), &[Dir::S]),
            ((-1, 1), &[Dir::S, Dir::W]),
            ((-1, 0), &[Dir::W]),
            ((-1, -1), &[Dir::W, Dir::N]),
        ];

        // Rule out directions
        for ((dx, dy), dirs) in DIRS {
            let new_coord = (self.x + dx, self.y + dy);
            if coords.contains(&new_coord) {
                for &d in *dirs {
                    options[d as usize] = false
                }
            }
        }
        if options == [true; 4] {
            return None;
        }

        // Pick first available
        for i in 0..4 {
            let dir = CONSIDERATIONS[(round + i) % 4];
            if options[dir as usize] {
                return Some(dir);
            }
        }
        None
    }
}

fn bounding_rect(elves: &[Elf]) -> (Coord, Coord) {
    let (mut xmin, mut ymin) = (isize::MAX, isize::MAX);
    let (mut xmax, mut ymax) = (isize::MIN, isize::MIN);

    for elf in elves {
        (xmin, ymin) = (xmin.min(elf.x), ymin.min(elf.y));
        (xmax, ymax) = (xmax.max(elf.x), ymax.max(elf.y));
    }
    ((xmin, ymin), (xmax, ymax))
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Elf>> {
    let mut elves = vec![];

    for (y, line) in input.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        for (x, byte) in line.bytes().enumerate() {
            match byte {
                b'#' => elves.push(Elf::new(x as isize, y as isize)),
                b'.' => (),
                c => bail!(
                    "invalid character {:?} at ({}, {})",
                    c as char,
                    x + 1,
                    y + 1
                ),
            };
        }
    }
    Ok(elves)
}
