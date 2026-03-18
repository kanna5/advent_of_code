//! Implements a solution for https://adventofcode.com/2022/day/24

use std::{collections::HashSet, io::BufRead};

use crate::{answer, solutions::Solution};
use anyhow::{Context, bail};

pub struct Day24;

const TIME_LIMIT: usize = 10_000;

impl<R: BufRead> Solution<R> for Day24 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut scene = read_input(input)?;
        match scene.walk((1, 0), (scene.width - 2, scene.height - 1), TIME_LIMIT) {
            Some(t) => answer!(t),
            None => answer!("failed to find a solution within time limit."),
        }
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut scene = read_input(input)?;
        let a = (1, 0);
        let b = (scene.width - 2, scene.height - 1);

        let errmsg = "failed to find a solution within time limit.";
        answer!(
            scene.walk(a, b, TIME_LIMIT).context(errmsg)?
                + scene.walk(b, a, TIME_LIMIT).context(errmsg)?
                + scene.walk(a, b, TIME_LIMIT).context(errmsg)?
        )
    }
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<u8> for Dir {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'^' => Self::Up,
            b'>' => Self::Right,
            b'v' => Self::Down,
            b'<' => Self::Left,
            b => bail!("invalid character {:?}", char::from(b)),
        })
    }
}

impl Dir {
    fn vec(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Scene {
    width: usize,
    height: usize,
    blizzard: Vec<(Coord, Dir)>,
    map: Vec<bool>, // true: empty
}

impl Scene {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn populate_map(&self, map: &mut [bool]) {
        map.copy_from_slice(&self.map); // clear
        for (c, _) in &self.blizzard {
            map[self.idx(c.0, c.1)] = false;
        }
    }

    fn move_blizzard(&mut self) {
        for (coord, dir) in &mut self.blizzard {
            let v = dir.vec();
            let new_c = (
                coord.0.saturating_add_signed(v.0),
                coord.1.saturating_add_signed(v.1),
            );
            *coord = match new_c {
                (x, 0) => (x, self.height - 2),
                (x, y) if y == self.height - 1 => (x, 1),
                (0, y) => (self.width - 2, y),
                (x, y) if x == self.width - 1 => (1, y),
                c => c,
            }
        }
    }

    fn walk(&mut self, orig: Coord, target: Coord, time_limit: usize) -> Option<usize> {
        let mut map = vec![false; self.width * self.height];
        let mut heads = HashSet::from([orig]);

        let moves = &[(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];

        for round in 1..=time_limit {
            self.move_blizzard();
            self.populate_map(&mut map);

            heads = heads
                .iter()
                .flat_map(|h| {
                    moves.iter().filter_map(|m| {
                        let (tx, ty) =
                            match (h.0.checked_add_signed(m.0), h.1.checked_add_signed(m.1)) {
                                (Some(x), Some(y)) => Some((x, y)),
                                _ => None,
                            }?;
                        if tx < self.width && ty < self.height && map[self.idx(tx, ty)] {
                            return Some((tx, ty));
                        }
                        None
                    })
                })
                .collect();

            if heads.is_empty() {
                break;
            }
            if heads.contains(&target) {
                return Some(round);
            }
        }
        None
    }
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Scene> {
    let (mut w, mut h) = (0, 0);
    let mut blizzard = Vec::new();
    let mut map = Vec::with_capacity(4096);

    for (y, line) in input.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        w = match w {
            0 => line.len(),
            w if w != line.len() => bail!("got variable line length at line {}", y + 1),
            _ => w,
        };

        for (x, byte) in line.bytes().enumerate() {
            let dir = match byte {
                b'#' => {
                    map.push(false);
                    continue;
                }
                b'.' => {
                    map.push(true);
                    continue;
                }
                b => {
                    map.push(true);
                    Dir::try_from(b).with_context(|| format!("parsing line {}", y + 1))?
                }
            };
            blizzard.push(((x, y), dir));
        }
        h += 1;
    }
    map.shrink_to_fit();

    Ok(Scene {
        width: w,
        height: h,
        blizzard,
        map,
    })
}
