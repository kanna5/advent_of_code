use super::Solution;
use anyhow::anyhow;
use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

pub struct Day16;

impl<R: BufRead> Solution<R> for Day16 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut map = read_input(input)?;
        Ok(map.solve().to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut map = read_input(input)?;
        _ = map.solve();
        Ok(map.best_spots().len().to_string())
    }
}

type Coord = (i64, i64);

#[derive(Debug)]
struct Map {
    width: i64,
    height: i64,
    start: Coord,
    end: Coord,
    data: Vec<Tile>,
}

impl Map {
    fn idx(&self, c: Coord) -> usize {
        (c.1 * self.width + c.0) as usize
    }

    fn contains(&self, c: &Coord) -> bool {
        (0..self.width).contains(&c.0) && (0..self.height).contains(&c.1)
    }

    fn get_score(&self, c: Coord, dir: Dir) -> Option<u64> {
        if !self.contains(&c) {
            return None;
        }
        let Tile::Path(v) = &self.data[self.idx(c)] else {
            return None;
        };
        Some(v[dir as usize])
    }

    fn update_score(&mut self, coord: Coord, facing: Dir, score: u64) -> bool {
        if !self.contains(&coord) {
            return false;
        }
        let idx = self.idx(coord);
        let Tile::Path(p) = &mut self.data[idx] else {
            return false;
        };
        if score < p[facing as usize] {
            p[facing as usize] = score;
            return true;
        }
        false
    }

    fn solve(&mut self) -> u64 {
        let mut queue = VecDeque::<(Coord, Dir, u64)>::with_capacity(512);
        self.update_score(self.start, Dir::East, 0);
        queue.push_back((self.start, Dir::East, 0));
        let mut goal = u64::MAX;

        while let Some((c, dir, score)) = queue.pop_front() {
            if score >= goal {
                continue;
            }

            // go straight
            let (next, nscore) = (dir.mv(&c), score + 1);
            if self.update_score(next, dir, score + 1) {
                match next == self.end {
                    true => goal = goal.min(score + 1),
                    false => queue.push_back((next, dir, nscore)),
                };
            }
            // turn left
            let (next_dir, nscore) = (dir.turn_left(), score + 1000);
            if self.update_score(c, next_dir, nscore) {
                queue.push_back((c, next_dir, nscore));
            }
            // turn right
            let (next_dir, nscore) = (dir.turn_right(), score + 1000);
            if self.update_score(c, next_dir, nscore) {
                queue.push_back((c, next_dir, nscore));
            }
        }
        goal
    }

    fn best_spots(&self) -> HashSet<Coord> {
        let mut ret = HashSet::<Coord>::with_capacity(256);
        ret.extend([self.start, self.end]);

        let Tile::Path(end_scores) = &self.data[self.idx(self.end)] else {
            panic!("not possible")
        };
        let (dir, &best_score) = end_scores.iter().enumerate().min_by_key(|f| f.1).unwrap();
        let dir = Dir::ALL[dir];
        let (dx, dy) = dir.vec();
        let mut queue = VecDeque::<(Coord, Dir, u64)>::with_capacity(256);
        queue.push_back(((self.end.0 - dx, self.end.1 - dy), dir, best_score - 1));

        while let Some((c, dir, score_exp)) = queue.pop_front() {
            match self.get_score(c, dir) {
                Some(score) if score == score_exp => (),
                _ => continue,
            }
            if !ret.insert(c) {
                continue;
            }

            // search back
            let v = dir.vec();
            queue.push_back(((c.0 - v.0, c.1 - v.1), dir, score_exp - 1));

            if score_exp > 1000 {
                // search left
                let dir_l = dir.turn_left();
                let v = dir_l.vec();
                queue.push_back(((c.0 - v.0, c.1 - v.1), dir_l, score_exp - 1001));
                // search right
                let dir_r = dir.turn_right();
                let v = dir_r.vec();
                queue.push_back(((c.0 - v.0, c.1 - v.1), dir_r, score_exp - 1001));
            }
        }
        ret
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    fn vec(&self) -> Coord {
        match self {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        }
    }

    fn turn_right(&self) -> Self {
        Self::ALL[(*self as usize + 1) % 4]
    }

    fn turn_left(&self) -> Self {
        Self::ALL[(*self as usize + 4 - 1) % 4]
    }

    fn mv(&self, c: &Coord) -> Coord {
        let (dx, dy) = self.vec();
        (c.0 + dx, c.1 + dy)
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Path(Scores),
}

type Scores = [u64; 4];

impl Tile {
    fn path() -> Self {
        Self::Path([u64::MAX; 4])
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Map, anyhow::Error> {
    let (mut width, mut height) = (0i64, 0i64);
    let (mut start, mut end) = ((0i64, 0i64), (0i64, 0i64));
    let mut data = Vec::<Tile>::with_capacity(4096);

    for (y, line) in input
        .lines()
        .map_while(Result::ok)
        .take_while(|i| !i.is_empty())
        .enumerate()
    {
        height += 1;
        width = line.len() as i64;
        for (x, b) in line.bytes().enumerate() {
            data.push(match b {
                b'S' => {
                    start = (x as i64, y as i64);
                    Tile::path()
                }
                b'E' => {
                    end = (x as i64, y as i64);
                    Tile::path()
                }
                b'.' => Tile::path(),
                b'#' => Tile::Wall,
                v => return Err(anyhow!("Got invalid character 0x{v:02x} at ({x}, {y})")),
            });
        }
    }
    Ok(Map {
        width,
        height,
        start,
        end,
        data,
    })
}
