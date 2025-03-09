use super::Solution;
use anyhow::anyhow;
use std::{collections::HashSet, fmt::Display, io::BufRead};

pub struct Day15;

impl<R: BufRead> Solution<R> for Day15 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (mut map, mvmt) = read_input(input)?;
        for m in mvmt {
            map.move_bot(m);
        }
        Ok(map.sum_box_coordinates().to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (map, mvmt) = read_input(input)?;
        let mut map = map.widen();
        for m in mvmt {
            map.move_bot(m);
        }
        Ok(map.sum_box_coordinates().to_string())
    }
}

#[derive(Debug)]
enum Movement {
    Up,
    Left,
    Down,
    Right,
}

impl Movement {
    fn vec(&self) -> (i64, i64) {
        match self {
            Movement::Up => (0, -1),
            Movement::Left => (-1, 0),
            Movement::Down => (0, 1),
            Movement::Right => (1, 0),
        }
    }
}

impl TryFrom<u8> for Movement {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Self::Up),
            b'>' => Ok(Self::Right),
            b'v' => Ok(Self::Down),
            b'<' => Ok(Self::Left),
            v => Err(anyhow!("invalid movement '{}'", v as char)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Box,
    WideBoxL,
    WideBoxR,
}

#[derive(Debug)]
struct Map {
    data: Vec<Tile>,
    width: i64,
    height: i64,
    bot_pos: (i64, i64),
    is_wide: bool,
}

impl Map {
    fn idx(&self, x: i64, y: i64) -> usize {
        (y * self.width + x) as usize
    }

    fn coord(&self, idx: usize) -> (i64, i64) {
        (idx as i64 % self.width, idx as i64 / self.width)
    }

    fn contains(&self, x: i64, y: i64) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    fn move_bot(&mut self, mvmt: Movement) {
        let (dx, dy) = mvmt.vec();

        let mut boxes = Vec::<((i64, i64), Tile)>::with_capacity(256);
        let mut visited = HashSet::<(i64, i64)>::with_capacity(256);
        let mut queue = Vec::<(i64, i64)>::with_capacity(256);
        queue.push((self.bot_pos.0 + dx, self.bot_pos.1 + dy));

        while let Some(c) = queue.pop() {
            if visited.contains(&c) {
                continue;
            }
            visited.insert(c);
            if !self.contains(c.0, c.1) {
                return;
            }
            match self.data[self.idx(c.0, c.1)] {
                Tile::Empty => continue,
                Tile::Wall => return,
                Tile::Box => boxes.push((c, Tile::Box)),
                Tile::WideBoxL => {
                    boxes.push((c, Tile::WideBoxL));
                    queue.push((c.0 + 1, c.1));
                }
                Tile::WideBoxR => {
                    boxes.push((c, Tile::WideBoxR));
                    queue.push((c.0 - 1, c.1));
                }
            }
            queue.push((c.0 + dx, c.1 + dy));
        }

        // clear old
        for (c, _) in &boxes {
            let idx = self.idx(c.0, c.1);
            self.data[idx] = Tile::Empty;
        }
        // put new
        for (c, typ) in boxes {
            let idx = self.idx(c.0 + dx, c.1 + dy);
            self.data[idx] = typ;
        }
        self.bot_pos = (self.bot_pos.0 + dx, self.bot_pos.1 + dy)
    }

    fn sum_box_coordinates(&self) -> i64 {
        self.data
            .iter()
            .enumerate()
            .filter(|v| *v.1 == Tile::Box || *v.1 == Tile::WideBoxL)
            .map(|(i, _)| {
                let (x, y) = self.coord(i);
                100 * y + x
            })
            .sum()
    }

    fn widen(&self) -> Self {
        assert!(!self.is_wide, "Cannot expand the map: already expanded");

        let mut ret = Self {
            data: Vec::with_capacity(self.data.len() * 2),
            width: self.width * 2,
            height: self.height,
            bot_pos: (self.bot_pos.0 * 2, self.bot_pos.1),
            is_wide: true,
        };
        ret.data.extend(self.data.iter().flat_map(|t| match t {
            Tile::Box => [Tile::WideBoxL, Tile::WideBoxR],
            v => [*v, *v],
        }));
        ret
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity((self.width * self.height + self.height) as usize);
        for (i, v) in self.data.iter().enumerate() {
            if i > 0 && i % self.width as usize == 0 {
                res.push('\n');
            }
            if self.coord(i) == self.bot_pos {
                res.push('@');
                continue;
            };

            res.push(match v {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::Box => 'O',
                Tile::WideBoxL => '[',
                Tile::WideBoxR => ']',
            });
        }
        write!(f, "{res}")
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<(Map, Vec<Movement>), anyhow::Error> {
    let mut map = Map {
        data: Vec::with_capacity(4096),
        width: 0,
        height: 0,
        bot_pos: (0, 0),
        is_wide: false,
    };
    for (y, line) in input.lines().map_while(Result::ok).enumerate() {
        if line.is_empty() {
            break;
        }
        map.width = line.len() as i64;
        map.height += 1;
        for (x, b) in line.bytes().enumerate() {
            match b {
                b'#' => map.data.push(Tile::Wall),
                b'O' => map.data.push(Tile::Box),
                b'.' => map.data.push(Tile::Empty),
                b'@' => {
                    map.data.push(Tile::Empty);
                    map.bot_pos = (x as i64, y as i64)
                }
                _ => return Err(anyhow!("invalid input at ({}, {})", x, y)),
            }
        }
    }

    let mvmt = input
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.into_bytes())
        .map(TryInto::<Movement>::try_into)
        .collect::<Result<_, _>>()?;

    Ok((map, mvmt))
}
