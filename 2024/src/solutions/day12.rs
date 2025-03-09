use super::Solution;
use crate::solutions::InvalidInput;

use std::io::BufRead;

pub struct Day12;

impl<R: BufRead> Solution<R> for Day12 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let map = read_input(input)?;
        Ok(map.fencing_cost().to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let map = read_input(input)?;
        Ok(map.fencing_cost_discounted().to_string())
    }
}

type Coordinate = (usize, usize);
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

struct Map {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Map {
    fn tr(&self, c: &Coordinate) -> usize {
        c.1 * self.width + c.0
    }

    fn tr_rev(&self, idx: usize) -> Coordinate {
        (idx % self.width, idx / self.width)
    }

    fn contains(&self, c: &Coordinate) -> bool {
        (..self.width).contains(&c.0) && (..self.height).contains(&c.1)
    }

    fn fencing_cost(&self) -> u64 {
        let mut cost = 0u64;
        let mut visited: Vec<bool> = vec![false; self.data.len()];

        for idx in 0..self.data.len() {
            let coord = self.tr_rev(idx);
            cost += self.visit(&coord, &mut visited)
        }
        cost
    }

    fn visit(&self, coord: &Coordinate, visited: &mut [bool]) -> u64 {
        let idx = self.tr(coord);
        if visited[idx] {
            return 0;
        }
        let typ = self.data[idx];

        let (mut tiles, mut edges) = (0u64, 0u64);
        let mut queue: Vec<Coordinate> = Vec::with_capacity(256);
        queue.push(*coord);

        while let Some(c) = queue.pop() {
            let idx = self.tr(&c);
            visited[idx] = match visited[idx] {
                true => continue,
                false => true,
            };
            tiles += 1;
            let mut edge = 4;

            neighbors(c.0 as isize, c.1 as isize)
                .filter(|(c, _)| c.0 >= 0 && c.1 >= 0)
                .map(|(c, _)| (c.0 as usize, c.1 as usize))
                .filter(|c| self.contains(c) && self.data[self.tr(c)] == typ)
                .for_each(|c| {
                    edge -= 1;
                    if !visited[self.tr(&c)] {
                        queue.push(c);
                    }
                });
            edges += edge;
        }
        tiles * edges
    }

    fn fencing_cost_discounted(&self) -> u64 {
        let mut cost = 0u64;
        let mut visited: Vec<bool> = vec![false; self.data.len()];

        for idx in 0..self.data.len() {
            let coord = self.tr_rev(idx);
            cost += self.visit_discounted(&coord, &mut visited)
        }
        cost
    }

    fn visit_discounted(&self, coord: &Coordinate, visited: &mut [bool]) -> u64 {
        let idx = self.tr(coord);
        if visited[idx] {
            return 0;
        }
        let typ = self.data[idx];

        let (mut tiles, mut faces) = (0u64, 0u64);
        let mut edges: Vec<Vec<(usize, usize)>> = (0..4).map(|_| Vec::with_capacity(256)).collect();

        let mut queue: Vec<Coordinate> = Vec::with_capacity(256);
        queue.push(*coord);
        visited[idx] = true;
        while let Some(c) = queue.pop() {
            tiles += 1;
            let (x, y) = c;

            neighbors(c.0 as isize, c.1 as isize).for_each(|(c_raw, dir)| {
                let c = (c_raw.0 as usize, c_raw.1 as usize);
                if c_raw.0 >= 0
                    && c_raw.1 >= 0
                    && self.contains(&c)
                    && self.data[self.tr(&c)] == typ
                {
                    if !visited[self.tr(&c)] {
                        visited[self.tr(&c)] = true;
                        queue.push(c);
                    }
                } else {
                    // The neighbor is outside of the region, so there's an edge
                    match dir {
                        Direction::Top | Direction::Bottom => edges[dir as usize].push((y, x)),
                        Direction::Left | Direction::Right => edges[dir as usize].push((x, y)),
                    }
                }
            });
        }

        for mut v in edges {
            v.sort_by(|a, b| match a.0.cmp(&b.0) {
                std::cmp::Ordering::Equal => a.1.cmp(&b.1),
                v => v,
            });

            let mut prev: Option<(usize, usize)> = None;
            for edge in v {
                match prev {
                    Some(p) if edge.0 == p.0 && edge.1 == p.1 + 1 => (),
                    _ => faces += 1,
                }
                prev = Some(edge)
            }
        }
        tiles * faces
    }
}

fn neighbors(x: isize, y: isize) -> impl Iterator<Item = ((isize, isize), Direction)> {
    [
        ((x + 1, y), Direction::Right),
        ((x, y + 1), Direction::Bottom),
        ((x - 1, y), Direction::Left),
        ((x, y - 1), Direction::Top),
    ]
    .into_iter()
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Map, anyhow::Error> {
    let mut data = Vec::<u8>::with_capacity(4096);
    let (mut width, mut height) = (0usize, 0usize);
    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        height += 1;
        let line = line.as_bytes();
        if width != 0 && width != line.len() {
            return Err(InvalidInput("Got variable line length".to_string()).into());
        }
        width = line.len();
        data.extend(line);
    }
    Ok(Map {
        width,
        height,
        data,
    })
}
