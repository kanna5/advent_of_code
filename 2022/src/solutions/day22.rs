//! Implements a solution for https://adventofcode.com/2022/day/22
//!
//! TODO: The part 2 solution is tailored to the shape of the input I got. If it's different,
//! adjustments to the `zip()` calls are needed.

use std::{collections::HashMap, io::BufRead};

use crate::{answer, solutions::Solution};
use anyhow::{Context, bail};

pub struct Day22;

impl<R: BufRead> Solution<R> for Day22 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (map, instructions) = read_input(input)?;

        // Compute jumps
        let mut jumps = HashMap::with_capacity((map.height + map.width) * 2);

        let mut col_ranges = vec![[usize::MAX, usize::MIN]; map.width];
        for y in 0..map.height {
            let mut row_range = [usize::MAX, usize::MIN];
            for (x, col_range) in col_ranges.iter_mut().enumerate() {
                if map.tiles[map.to_idx(x, y)] == Tile::Void {
                    continue;
                }
                row_range = [x.min(row_range[0]), x.max(row_range[1])];
                *col_range = [y.min(col_range[0]), y.max(col_range[1])];
            }

            jumps.insert(
                Cursor::new(row_range[0], y, Dir::Left),
                Cursor::new(row_range[1], y, Dir::Left),
            );
            jumps.insert(
                Cursor::new(row_range[1], y, Dir::Right),
                Cursor::new(row_range[0], y, Dir::Right),
            );
        }
        for (x, col_range) in col_ranges.iter().enumerate() {
            jumps.insert(
                Cursor::new(x, col_range[0], Dir::Up),
                Cursor::new(x, col_range[1], Dir::Up),
            );
            jumps.insert(
                Cursor::new(x, col_range[1], Dir::Down),
                Cursor::new(x, col_range[0], Dir::Down),
            );
        }

        let mut cursor = map.start;
        for inst in instructions {
            match inst {
                Instruction::Turn(turn_dir) => cursor.dir = cursor.dir.turn(turn_dir),
                Instruction::Walk(dist) => cursor = map.walk_cursor(cursor, dist, &jumps),
            }
        }
        answer!(cursor.password())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (map, instructions) = read_input(input)?;

        let expect_layout = vec![vec![0, 1, 1], vec![0, 1, 0], vec![1, 1, 0], vec![1, 0, 0]];
        if !map.matches_layout(&expect_layout) {
            bail!(
                "The input does not match the expected layout.\nInput:\n{}\nExpected:\n{}",
                draw_layout(&map.layout()),
                draw_layout(&expect_layout)
            );
        }

        let mut z = Zipper::new(gcd(map.width, map.height));
        z.zip((2, 0), Dir::Right, (1, 2), Dir::Left);
        z.zip((2, 0), Dir::Down, (1, 1), Dir::Left);
        z.zip((1, 2), Dir::Down, (0, 3), Dir::Left);
        z.zip((0, 2), Dir::Up, (1, 1), Dir::Right);
        z.zip((1, 0), Dir::Left, (0, 2), Dir::Right);
        z.zip((1, 0), Dir::Up, (0, 3), Dir::Right);
        z.zip((2, 0), Dir::Up, (0, 3), Dir::Up);

        let mut cursor = map.start;
        for inst in instructions {
            match inst {
                Instruction::Turn(turn_dir) => cursor.dir = cursor.dir.turn(turn_dir),
                Instruction::Walk(dist) => cursor = map.walk_cursor(cursor, dist, &z.jumps),
            }
        }
        answer!(cursor.password())
    }
}

type Face = (usize, usize);

struct Zipper {
    jumps: HashMap<Cursor, Cursor>,
    edge_len: usize,
}

impl Zipper {
    fn new(edge_len: usize) -> Self {
        Self {
            jumps: HashMap::with_capacity(edge_len * 14),
            edge_len,
        }
    }

    fn zip(&mut self, f1: Face, out_dir: Dir, f2: Face, in_dir: Dir) {
        let elen = self.edge_len - 1;

        let get_init_pos = |f: Face, d: Dir| -> (usize, usize, Dir) {
            let topleft = (f.0 * self.edge_len, f.1 * self.edge_len);

            let pos = match d {
                Dir::Right => (topleft.0 + elen, topleft.1),
                Dir::Down => (topleft.0 + elen, topleft.1 + elen),
                Dir::Left => (topleft.0, topleft.1 + elen),
                Dir::Up => (topleft.0, topleft.1),
            };
            (pos.0, pos.1, d.turn(TurnDir::Right))
        };

        let (p1x, p1y, d1) = get_init_pos(f1, out_dir);
        let (p2x, p2y, d2) = get_init_pos(f2, in_dir.rev());

        // Move p2 to the other end
        let vec_t = d2.vec();
        let (p2x, p2y) = (
            p2x.wrapping_add_signed(vec_t.0 * elen as isize),
            p2y.wrapping_add_signed(vec_t.1 * elen as isize),
        );

        let (v1, v2) = (d1.vec(), d2.rev().vec());
        for i in 0..self.edge_len as isize {
            let c1 = Cursor::new(
                p1x.wrapping_add_signed(i * v1.0),
                p1y.wrapping_add_signed(i * v1.1),
                out_dir,
            );
            let c2 = Cursor::new(
                p2x.wrapping_add_signed(i * v2.0),
                p2y.wrapping_add_signed(i * v2.1),
                in_dir,
            );
            self.jumps.insert(c1, c2);
            self.jumps.insert(
                Cursor::new(c2.x, c2.y, c2.dir.rev()),
                Cursor::new(c1.x, c1.y, c1.dir.rev()),
            );
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a.max(b), a.min(b));
    loop {
        match a % b {
            0 => return b,
            c => (a, b) = (b, c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Void,
    Empty,
    Wall,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    start: Cursor,
}

impl Map {
    fn to_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn walk_cursor(
        &self,
        cursor: Cursor,
        distance: usize,
        jumps: &HashMap<Cursor, Cursor>,
    ) -> Cursor {
        let mut c = cursor;

        for _ in 0..distance {
            let (dx, dy) = c.dir.vec();

            let mut jump = false;
            let (nx, ny) = match (c.x.checked_add_signed(dx), c.y.checked_add_signed(dy)) {
                (Some(x), Some(y)) => (x, y),
                _ => {
                    jump = true;
                    (0, 0) // won't be used
                }
            };

            if !(0..self.width).contains(&nx)
                || !(0..self.height).contains(&ny)
                || self.tiles[self.to_idx(nx, ny)] == Tile::Void
            {
                jump = true
            }

            let next_c = match jump {
                true => match jumps.get(&c) {
                    Some(n) => *n,
                    None => break,
                },
                false => Cursor {
                    x: nx,
                    y: ny,
                    dir: c.dir,
                },
            };

            if self.tiles[self.to_idx(next_c.x, next_c.y)] == Tile::Wall {
                break;
            }
            c = next_c;
        }
        c
    }

    fn layout(&self) -> Vec<Vec<u8>> {
        let edge_len = gcd(self.width, self.height);
        (0..self.height / edge_len)
            .map(|y| {
                (0..self.width / edge_len)
                    .map(|x| {
                        match self.tiles[self.to_idx(x * edge_len, y * edge_len)] == Tile::Void {
                            true => 0,
                            false => 1,
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn matches_layout(&self, layout: &[Vec<u8>]) -> bool {
        let edge_len = gcd(self.width, self.height);
        if layout.len() != self.height / edge_len {
            return false;
        }
        for (y, l_row) in layout.iter().enumerate() {
            if l_row.len() != self.width / edge_len {
                return false;
            }
            for (x, &expect) in l_row.iter().enumerate() {
                let actual = match self.tiles[self.to_idx(x * edge_len, y * edge_len)] {
                    Tile::Void => 0,
                    _ => 1,
                };
                if actual != expect {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, Clone, Copy)]
enum TurnDir {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl From<u8> for Dir {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }
}

impl Dir {
    fn turn(self, dir: TurnDir) -> Self {
        match dir {
            TurnDir::Left => (self as u8 + 3) % 4,
            TurnDir::Right => (self as u8 + 1) % 4,
        }
        .into()
    }

    fn rev(self) -> Self {
        ((self as u8 + 2) % 4).into()
    }

    fn vec(self) -> (isize, isize) {
        match self {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Turn(TurnDir),
    Walk(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cursor {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Cursor {
    fn new(x: usize, y: usize, dir: Dir) -> Self {
        Self { x, y, dir }
    }

    fn password(&self) -> usize {
        1000 * (self.y + 1) + 4 * (self.x + 1) + self.dir as usize
    }
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<(Map, Vec<Instruction>)> {
    let map_raw: Vec<_> = input
        .lines()
        .map_while(|line| match &line {
            Ok(l) => match l.is_empty() {
                true => None,
                false => Some(line),
            },
            Err(_) => Some(line),
        })
        .collect::<Result<_, _>>()?;

    let w: usize = map_raw.iter().map(|l| l.len()).max().context("empty map")?;
    let h: usize = map_raw.len();

    let mut map = Map {
        tiles: vec![Tile::Void; w * h],
        start: Cursor::new(0, 0, Dir::Right),
        width: w,
        height: h,
    };

    for (row, line) in map_raw.iter().enumerate() {
        for (col, byt) in line.bytes().enumerate() {
            if byt == b' ' {
                continue;
            }

            map.tiles[row * w + col] = match byt {
                b'.' => Tile::Empty,
                b'#' => Tile::Wall,
                b => bail!("invalid tile {:?} at {}, {}", b as char, row, col),
            };
        }
    }
    match (0..w).find(|&x| map.tiles[x] == Tile::Empty) {
        Some(x) => map.start.x = x,
        None => bail!("cannot find a suitable starting position"),
    }

    let mut inst_raw = String::new();
    input.read_line(&mut inst_raw)?;

    let mut instructions = Vec::new();
    let mut cur_num = 0usize;
    for b in inst_raw.trim_ascii().bytes() {
        if b.is_ascii_digit() {
            cur_num = cur_num * 10 + (b - b'0') as usize
        } else {
            if cur_num > 0 {
                instructions.push(Instruction::Walk(cur_num));
                cur_num = 0
            }
            instructions.push(Instruction::Turn(match b {
                b'L' => TurnDir::Left,
                b'R' => TurnDir::Right,
                _ => bail!("invalid turn direction {:?}", b as char),
            }));
        }
    }
    if cur_num > 0 {
        instructions.push(Instruction::Walk(cur_num));
    }

    Ok((map, instructions))
}

/// For debugging purposes only. Print the layout of the map in a more readable way.
fn draw_layout(layout: &[Vec<u8>]) -> String {
    let mut ret = String::new();
    for (y, row) in layout.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            let block = match y.abs_diff(x) % 2 == 0 {
                true => "▒▒",
                false => "██",
            };
            match tile {
                1 => ret.push_str(block),
                _ => ret.push_str("  "),
            }
        }
        ret.push('\n');
    }
    ret
}
