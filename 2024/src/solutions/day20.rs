use super::{InvalidInput, Options, Solution};
use std::{collections::VecDeque, io::BufRead};

pub struct Day20 {
    pub options: Options,
}

impl<R: BufRead> Solution<R> for Day20 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let min_save: usize = self.options.try_get("MIN_SAVE", 100)?;
        let mut map = read_input(input)?;

        map.find_distances();
        Ok(map.find_n_cheats_p1(min_save).to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let min_save: usize = self.options.try_get("MIN_SAVE", 100)?;
        let mut map = read_input(input)?;

        map.find_distances();
        Ok(map.find_n_cheats_p2(min_save).to_string())
    }
}

const UNREACHABLE: isize = isize::MAX;
const DIRECTIONS: [Coord; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Clone)]
struct Cell {
    dist_from_start: isize,
    dist_from_end: isize,
    is_wall: bool,
}

type Coord = (isize, isize);

fn distance(a: &Coord, b: &Coord) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    data: Vec<Cell>,
    start: Coord,
    end: Coord,
}

impl Map {
    fn find_distances(&mut self) {
        let mut queue = VecDeque::<Coord>::with_capacity(1024);
        // distance from start
        queue.push_back(self.start);
        while let Some(c) = queue.pop_front() {
            let next_dist = self.data[self.idx(&c).unwrap()].dist_from_start + 1;
            for next in DIRECTIONS.iter().map(|d| (c.0 + d.0, c.1 + d.1)) {
                let Some(idx) = self.idx(&next) else {
                    continue;
                };
                let cell = &mut self.data[idx];
                if cell.is_wall || cell.dist_from_start <= next_dist {
                    continue;
                }

                cell.dist_from_start = next_dist;
                queue.push_back(next);
            }
        }
        // distance from end
        queue.clear();
        queue.push_back(self.end);
        while let Some(c) = queue.pop_front() {
            let next_dist = self.data[self.idx(&c).unwrap()].dist_from_end + 1;
            for next in DIRECTIONS.iter().map(|d| (c.0 + d.0, c.1 + d.1)) {
                let Some(idx) = self.idx(&next) else {
                    continue;
                };
                let cell = &mut self.data[idx];
                if cell.is_wall || cell.dist_from_end <= next_dist {
                    continue;
                }

                cell.dist_from_end = next_dist;
                queue.push_back(next);
            }
        }
    }

    fn find_n_cheats_p1(&self, min_save: usize) -> usize {
        let target = self.data[self.idx(&self.start).unwrap()].dist_from_end - min_save as isize;
        let mut cnt = 0usize;

        let mut neighbors = Vec::<&Cell>::with_capacity(4);
        self.data
            .iter()
            .enumerate()
            .filter(|v| v.1.is_wall)
            .for_each(|(idx, _)| {
                let c = (idx as isize % self.width, idx as isize / self.width);
                neighbors.clear();
                neighbors.extend(
                    DIRECTIONS
                        .iter()
                        .map(|d| (c.0 + d.0, c.1 + d.1))
                        .filter_map(|n| {
                            let idx = self.idx(&n)?;
                            let cell = &self.data[idx];
                            match cell.is_wall {
                                true => None,
                                false => Some(cell),
                            }
                        }),
                );
                for (i, ic) in neighbors.iter().enumerate() {
                    for (j, jc) in neighbors.iter().enumerate() {
                        if i == j {
                            continue;
                        }
                        if ic.dist_from_start < target
                            && jc.dist_from_end < target
                            && ic.dist_from_start + jc.dist_from_end + 2 <= target
                        {
                            cnt += 1;
                        }
                    }
                }
            });
        cnt
    }

    fn find_n_cheats_p2(&self, min_save: usize) -> usize {
        let target = self.data[self.idx(&self.start).unwrap()].dist_from_end - min_save as isize;

        let mut jump_from = Vec::<(Coord, isize)>::with_capacity(1024);
        let mut jump_to = Vec::<(Coord, isize)>::with_capacity(1024);

        for (idx, cell) in self.data.iter().enumerate() {
            let c = (idx as isize % self.width, idx as isize / self.width);
            if cell.is_wall {
                continue;
            }
            if cell.dist_from_start < target {
                jump_from.push((c, cell.dist_from_start));
            }
            if cell.dist_from_end < target {
                jump_to.push((c, cell.dist_from_end));
            }
        }

        let mut cnt = 0usize;
        for (ic, it) in jump_from {
            for (jc, jt) in jump_to.iter() {
                let dist = distance(&ic, jc);
                if dist > 20 {
                    continue;
                }
                if it + jt + dist as isize <= target {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn contains(&self, c: &Coord) -> bool {
        (0..self.width).contains(&c.0) && (0..self.height).contains(&c.1)
    }

    fn idx(&self, c: &Coord) -> Option<usize> {
        match self.contains(c) {
            true => Some((c.1 * self.width + c.0) as usize),
            false => None,
        }
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Map, anyhow::Error> {
    let mut map = Map {
        width: 0,
        height: 0,
        data: Vec::with_capacity(4096),
        start: (0, 0),
        end: (0, 0),
    };

    for line in input
        .lines()
        .map_while(Result::ok)
        .take_while(|v| !v.is_empty())
    {
        map.height += 1;
        match map.width {
            0 => map.width = line.len() as isize,
            v if line.len() as isize != v => {
                return Err(InvalidInput("Got variable line length".to_string()).into())
            }
            _ => (),
        }

        for (x, b) in line.bytes().enumerate() {
            map.data.push(match b {
                b'#' => Cell {
                    dist_from_start: UNREACHABLE,
                    dist_from_end: UNREACHABLE,
                    is_wall: true,
                },
                b'.' => Cell {
                    dist_from_start: UNREACHABLE,
                    dist_from_end: UNREACHABLE,
                    is_wall: false,
                },
                b'S' => {
                    map.start = (x as isize, map.height - 1);
                    Cell {
                        dist_from_start: 0,
                        dist_from_end: UNREACHABLE,
                        is_wall: false,
                    }
                }
                b'E' => {
                    map.end = (x as isize, map.height - 1);
                    Cell {
                        dist_from_start: UNREACHABLE,
                        dist_from_end: 0,
                        is_wall: false,
                    }
                }
                v => return Err(InvalidInput(format!("Invalid character {:02x}", v)).into()),
            });
        }
    }

    Ok(map)
}
