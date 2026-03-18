//! Implements a solution for https://adventofcode.com/2022/day/17

use std::io::BufRead;

use crate::{answer, solutions::Solution};
use anyhow::anyhow;

const SHAPES: &[&[(i32, i32)]] = &[
    &[(0, 0), (1, 0), (2, 0), (3, 0)],         // -
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // +
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // J
    &[(0, 0), (0, 1), (0, 2), (0, 3)],         // I
    &[(0, 0), (1, 0), (0, 1), (1, 1)],         // O
];

type Coord = (i32, i32);

pub struct Day17;

impl<R: BufRead> Solution<R> for Day17 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let jet_dirs = read_input(input)?;
        let mut c = Chamber::new(jet_dirs);

        for _ in 0..2022 {
            c.drop_next();
        }
        answer!(c.height)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let jet_dirs = read_input(input)?;
        let cycle_len = SHAPES.len() * jet_dirs.len();

        let target_stone: i64 = 1_000_000_000_000 - 1;
        let mut pos_record: Vec<Coord> = Vec::with_capacity(cycle_len * 2);
        let mut h_record: Vec<i32> = Vec::with_capacity(cycle_len * 2);

        let mut c = Chamber::new(jet_dirs);
        for _ in 0..cycle_len {
            pos_record.push(c.drop_next());
            h_record.push(c.height);
        }

        let (mut loop_start, mut loop_len) = (0, 0);
        'outer: for i in 1.. {
            for _ in 0..cycle_len {
                pos_record.push(c.drop_next());
                h_record.push(c.height);
            }

            for j in 0..i {
                if is_repeat(
                    &pos_record[j * cycle_len..(j + 1) * cycle_len],
                    &pos_record[i * cycle_len..],
                ) {
                    loop_start = j * cycle_len;
                    loop_len = i * cycle_len - loop_start;
                    break 'outer;
                }
            }
        }

        let h_base = match loop_start {
            0 => 0,
            st => h_record[st - 1] as i64,
        };
        let loop_h_diff = h_record[loop_start + loop_len - 1] as i64 - h_base;

        let offset = ((target_stone - loop_start as i64) % loop_len as i64) as usize;
        let n_loops = (target_stone - loop_start as i64) / loop_len as i64;

        let h = h_base + n_loops * loop_h_diff + (h_record[loop_start + offset] as i64 - h_base);
        answer!(h)
    }
}

fn is_repeat(a: &[Coord], b: &[Coord]) -> bool {
    let offset_y = b[0].1 - a[0].1;
    a.iter()
        .zip(b)
        .all(|(&(x1, y1), &(x2, y2))| x1 == x2 && y1 == y2 - offset_y)
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Right,
    Down,
    Left,
}

const VECTS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0)];

impl Dir {
    fn vect(self) -> &'static (i32, i32) {
        &VECTS[self as usize]
    }
}

fn read_input<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Dir>> {
    let mut line = String::new();
    input.read_line(&mut line)?;

    line.trim_ascii()
        .as_bytes()
        .iter()
        .map(|&c| match c {
            b'<' => Ok(Dir::Left),
            b'>' => Ok(Dir::Right),
            s => Err(anyhow!("invalid character {:?}", s)),
        })
        .collect()
}

struct Chamber {
    data: Vec<bool>,
    height: i32,
    next_shape: usize,
    dirs: Vec<Dir>,
    next_dir: usize,
}

impl Chamber {
    fn drop_next(&mut self) -> Coord {
        let shape = SHAPES[self.next_shape];
        self.next_shape = (self.next_shape + 1) % SHAPES.len();
        let mut pos = (2, self.height + 3);

        loop {
            // Blow sideways
            let dir = self.dirs[self.next_dir];
            self.next_dir = (self.next_dir + 1) % self.dirs.len();

            let (dx, dy) = dir.vect();
            let next_pos = (pos.0 + dx, pos.1 + dy);
            if self.can_place(shape, next_pos) {
                pos = next_pos
            }

            // Fall down
            let (dx, dy) = Dir::Down.vect();
            let next_pos = (pos.0 + dx, pos.1 + dy);
            if self.can_place(shape, next_pos) {
                pos = next_pos
            } else {
                // Come to rest
                self.place(shape, pos);
                return pos;
            }
        }
    }

    fn can_place(&self, shape: &[Coord], pos: Coord) -> bool {
        shape.iter().all(|offset| {
            let (x, y) = (pos.0 + offset.0, pos.1 + offset.1);
            (0..7).contains(&x) && y >= 0 && (y >= self.height || !self.data[(y * 7 + x) as usize])
        })
    }

    fn place(&mut self, shape: &[Coord], pos: Coord) {
        let new_h = shape.iter().map(|o| o.1 + pos.1).max().unwrap() + 1;
        if new_h > self.height {
            self.data.resize((new_h * 7) as usize, false);
            self.height = new_h;
        }

        for offset in shape {
            let (x, y) = (pos.0 + offset.0, pos.1 + offset.1);
            self.data[(y * 7 + x) as usize] = true;
        }
    }

    fn new(dirs: Vec<Dir>) -> Self {
        Chamber {
            data: Vec::with_capacity(4096),
            height: 0,
            next_shape: 0,
            dirs,
            next_dir: 0,
        }
    }
}
