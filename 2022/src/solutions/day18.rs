//! Implements a solution for https://adventofcode.com/2022/day/18

use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
    ops::{Add, Sub},
};

use crate::{answer, solutions::Solution};
use anyhow::Context;

pub struct Day18;

impl<R: BufRead> Solution<R> for Day18 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let cubes = read_input(input)?;

        let mut exposed = cubes.len() * 6;
        let mut faces = HashSet::new();

        for cube in &cubes {
            for f in cube.cube_faces() {
                if !faces.insert(f) {
                    exposed -= 2
                }
            }
        }
        answer!(exposed)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let cubes = read_input(input)?;
        let (mut coord_min, mut coord_max) = (cubes[0], cubes[0]);

        let mut faces = HashSet::new();
        for cube in &cubes {
            for f in cube.cube_faces() {
                if !faces.insert(f) {
                    faces.remove(&f);
                }
                coord_min = coord_min.comp_min(cube);
                coord_max = coord_max.comp_max(cube);
            }
        }
        coord_min = coord_min - Coord(1, 1, 1);
        coord_max = coord_max + Coord(1, 1, 1);

        let mapper = Mapper {
            xlen: coord_max.0 - coord_min.0 + 1,
            ylen: coord_max.1 - coord_min.1 + 1,
            zlen: coord_max.2 - coord_min.2 + 1,
            base: coord_min,
        };
        let mut visited = vec![false; (mapper.xlen * mapper.ylen * mapper.zlen) as usize];

        for cube in &cubes {
            visited[mapper.ctoi(cube)] = true;
        }

        let mut exposed = 0;
        let mut queue = VecDeque::<usize>::with_capacity(512);
        queue.push_back(0);
        visited[0] = true;
        while let Some(i) = queue.pop_front() {
            let c = mapper.itoc(i);
            for n in c.neighbors() {
                let i_n = mapper.ctoi(&n);
                if !mapper.contains(&n) || visited[i_n] {
                    continue;
                }
                for face in n.cube_faces() {
                    if faces.contains(&face) {
                        exposed += 1
                    }
                }

                visited[i_n] = true;
                queue.push_back(i_n);
            }
        }
        answer!(exposed)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coord(i32, i32, i32);

impl Coord {
    fn cube_faces(&self) -> [Face; 6] {
        [
            (*self, 0),
            (*self, 1),
            (*self, 2),
            (Coord(self.0 - 1, self.1, self.2), 0),
            (Coord(self.0, self.1 - 1, self.2), 1),
            (Coord(self.0, self.1, self.2 - 1), 2),
        ]
    }

    fn comp_min(&self, another: &Coord) -> Self {
        Coord(
            self.0.min(another.0),
            self.1.min(another.1),
            self.2.min(another.2),
        )
    }

    fn comp_max(&self, another: &Coord) -> Self {
        Coord(
            self.0.max(another.0),
            self.1.max(another.1),
            self.2.max(another.2),
        )
    }

    fn neighbors(&self) -> [Coord; 6] {
        [
            Coord(self.0 + 1, self.1, self.2),
            Coord(self.0, self.1 + 1, self.2),
            Coord(self.0, self.1, self.2 + 1),
            Coord(self.0 - 1, self.1, self.2),
            Coord(self.0, self.1 - 1, self.2),
            Coord(self.0, self.1, self.2 - 1),
        ]
    }
}

struct Mapper {
    xlen: i32,
    ylen: i32,
    zlen: i32,
    base: Coord,
}

impl Mapper {
    fn ctoi(&self, c: &Coord) -> usize {
        let c = *c - self.base;
        (c.0 * self.ylen * self.zlen + c.1 * self.zlen + c.2) as usize
    }

    fn itoc(&self, i: usize) -> Coord {
        let i = i as i32;
        let yz = self.ylen * self.zlen;
        let x = i / yz;
        let y = i % yz / self.zlen;
        let z = i % self.zlen;
        Coord(x, y, z) + self.base
    }

    fn contains(&self, c: &Coord) -> bool {
        let c = *c - self.base;
        (0..self.xlen).contains(&c.0)
            && (0..self.ylen).contains(&c.1)
            && (0..self.zlen).contains(&c.2)
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

type Face = (Coord, u8);

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Coord>> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let line = line?;
        let mut parts = line.splitn(3, ',');
        let x: i32 = parts
            .next()
            .context("missing x")?
            .parse()
            .context("failed to parse x")?;
        let y: i32 = parts
            .next()
            .context("missing y")?
            .parse()
            .context("failed to parse y")?;
        let z: i32 = parts
            .next()
            .context("missing z")?
            .parse()
            .context("failed to parse z")?;
        ret.push(Coord(x, y, z));
    }
    Ok(ret)
}
