use super::{InvalidInput, Solution};
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

pub struct Day8;

impl<R: BufRead> Solution<R> for Day8 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let map = read_input(input)?;
        let mut antinodes_locs =
            HashSet::<Location>::with_capacity((map.width * map.height) as usize);

        for locs in map.locations.values() {
            for (i, loc1) in locs.iter().enumerate() {
                for loc2 in &locs[i + 1..] {
                    let nodes = loc1.antinodes_1(loc2);
                    antinodes_locs.extend(nodes);
                }
            }
        }
        let count = antinodes_locs.iter().filter(|l| map.contains(l)).count();
        Ok(count.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let map = read_input(input)?;
        let mut antinodes_locs =
            HashSet::<Location>::with_capacity((map.width * map.height) as usize);

        for locs in map.locations.values() {
            for (i, loc1) in locs.iter().enumerate() {
                for loc2 in &locs[i + 1..] {
                    let nodes = loc1.antinodes_2(loc2, &map);
                    antinodes_locs.extend(nodes);
                }
            }
        }
        let count = antinodes_locs.iter().filter(|l| map.contains(l)).count();
        Ok(count.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Location(i64, i64);

impl Location {
    fn antinodes_1(&self, other: &Self) -> [Self; 2] {
        let (diff_x, diff_y) = (other.0 - self.0, other.1 - self.1);
        [
            Location(self.0 - diff_x, self.1 - diff_y),
            Location(other.0 + diff_x, other.1 + diff_y),
        ]
    }

    fn antinodes_2(&self, other: &Self, map: &MapInfo) -> Vec<Location> {
        let mut ret = Vec::<Location>::with_capacity(map.width.max(map.height) as usize);
        let diff = (other.0 - self.0, other.1 - self.1);
        ret.push(self.clone());
        let mut next = self.clone();
        loop {
            next = Location(next.0 - diff.0, next.1 - diff.1);
            match map.contains(&next) {
                true => ret.push(next.clone()),
                false => break,
            }
        }
        let mut next = self.clone();
        loop {
            next = Location(next.0 + diff.0, next.1 + diff.1);
            match map.contains(&next) {
                true => ret.push(next.clone()),
                false => break,
            }
        }
        ret
    }
}

#[derive(Debug)]
struct MapInfo {
    width: i64,
    height: i64,
    locations: HashMap<char, Vec<Location>>,
}

impl MapInfo {
    fn contains(&self, loc: &Location) -> bool {
        (0..self.width).contains(&loc.0) && (0..self.height).contains(&loc.1)
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<MapInfo, anyhow::Error> {
    let mut map = MapInfo {
        width: 0,
        height: 0,
        locations: HashMap::with_capacity(256),
    };

    for (y, line) in input.lines().map_while(Result::ok).enumerate() {
        if line.is_empty() {
            break;
        }
        if map.width != 0 && map.width != line.len() as i64 {
            return Err(InvalidInput("Unexpected variable line length.".to_string()).into());
        }
        map.width = line.len() as i64;
        map.height += 1;

        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                _ => {
                    let locs = map
                        .locations
                        .entry(c)
                        .or_insert_with(|| Vec::with_capacity(256));
                    locs.push(Location(x as i64, y as i64));
                }
            }
        }
    }
    Ok(map)
}
