use super::Solution;
use anyhow::{Context, Ok};
use std::{fmt::Display, io::BufRead};

pub struct Day14;

impl<R: BufRead> Solution<R> for Day14 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let guards = read_input(input)?;

        let mut quads = [0u64; 4];
        for g in guards {
            if let Some(q) = g.pos_quadrant_when(100) {
                quads[q as usize] += 1;
            }
        }
        let sum = quads[0] * quads[1] * quads[2] * quads[3];
        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut guards = read_input(input)?;
        let mut prev_score = 0;

        for i in 1.. {
            for g in &mut guards {
                *g = g.seconds_later(1);
            }
            let map: Map = (&guards[..]).into();
            let score = map.score();

            if prev_score > 0 && score / prev_score > 2 {
                return Ok(format!("{}\n{}", map, i));
            }
            prev_score = score;
        }
        panic!("Cannot find answer after eternity :(")
    }
}

const MAP_WIDTH: i64 = 101;
const MAP_HEIGHT: i64 = 103;

#[derive(Debug, Default)]
struct Guard {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Guard {
    fn pos_when(&self, seconds: u8) -> (i64, i64) {
        let s = seconds as i64;
        let x = (self.position.0 + self.velocity.0 * s) % MAP_WIDTH;
        let y = (self.position.1 + self.velocity.1 * s) % MAP_HEIGHT;

        ((x + MAP_WIDTH) % MAP_WIDTH, (y + MAP_HEIGHT) % MAP_HEIGHT)
    }

    /// 0 1
    /// 2 3
    fn pos_quadrant_when(&self, seconds: u8) -> Option<u8> {
        let (x, y) = self.pos_when(seconds);
        let mid = (MAP_WIDTH / 2, MAP_HEIGHT / 2);

        let x_part: u8 = match x.cmp(&mid.0) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => return None,
            std::cmp::Ordering::Greater => 1,
        };
        let y_part: u8 = match y.cmp(&mid.1) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => return None,
            std::cmp::Ordering::Greater => 1,
        };
        Some(y_part * 2 + x_part)
    }

    fn seconds_later(&self, seconds: u8) -> Self {
        Self {
            position: self.pos_when(seconds),
            velocity: self.velocity,
        }
    }
}

struct Map(Vec<bool>);

impl Map {
    fn score(&self) -> u64 {
        let mut sum = 0u64;
        let mut continuity: u64 = 0;
        for g in &self.0 {
            match g {
                true => continuity += 1,
                false => {
                    sum += continuity.pow(2);
                    continuity = 0;
                }
            }
        }
        sum
    }
}

impl From<&[Guard]> for Map {
    fn from(value: &[Guard]) -> Self {
        let mut data = vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize];
        for g in value {
            data[coord_to_idx(g.position.0, g.position.1)] = true;
        }
        Map(data)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity((MAP_WIDTH * MAP_HEIGHT + MAP_HEIGHT) as usize);
        for (i, c) in self.0.iter().enumerate() {
            s.push(match c {
                true => 'o',
                false => ' ',
            });
            if i as i64 % MAP_WIDTH == 0 && i > 0 {
                s.push('\n');
            };
        }
        write!(f, "{}", s)
    }
}

fn coord_to_idx(x: i64, y: i64) -> usize {
    (y * MAP_WIDTH + x) as usize
}

impl TryFrom<&str> for Guard {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ret = Self::default();

        let parts = value.split_once(' ').context("expected two parts")?;
        let pos = (parts.0.trim()[2..])
            .split_once(',')
            .context("invalid position")?;
        ret.position = (pos.0.trim().parse()?, pos.1.trim().parse()?);

        let vel = (parts.1.trim()[2..])
            .split_once(',')
            .context("invalid velocity")?;
        ret.velocity = (vel.0.trim().parse()?, vel.1.trim().parse()?);
        Ok(ret)
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<Guard>, anyhow::Error> {
    let mut ret = Vec::<Guard>::with_capacity(512);

    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        ret.push(line.as_str().try_into()?);
    }
    Ok(ret)
}
