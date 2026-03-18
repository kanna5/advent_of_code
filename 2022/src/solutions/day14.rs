//! Implements a solution for https://adventofcode.com/2022/day/14

use std::{
    cmp::{max, min},
    fmt::{Display, Write},
    io::BufRead,
    ops::{Index, IndexMut},
};

use crate::{
    answer,
    solutions::{Options, Solution},
};
use anyhow::Context;

pub struct Day14 {
    pub opts: Options,
}

impl<R: BufRead> Solution<R> for Day14 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (mut map, start_x) = read_input(input, false)?;
        let mut n_dropped: usize = 0;

        'outer: loop {
            let (mut x, mut y) = (start_x, 0);

            // Simulate fall
            loop {
                if y + 1 >= map.h {
                    break 'outer; // falled out of map
                }
                if map[y + 1][x] == Cell::Air {
                    y += 1; // fall straight
                    continue;
                }

                if x == 0 {
                    break 'outer;
                }
                if map[y + 1][x - 1] == Cell::Air {
                    (x, y) = (x - 1, y + 1); // fall to the left
                    continue;
                }

                if x + 1 >= map.w {
                    break 'outer;
                }
                if map[y + 1][x + 1] == Cell::Air {
                    (x, y) = (x + 1, y + 1); // fall to the right
                    continue;
                }

                // stops
                map[y][x] = Cell::Sand;
                n_dropped += 1;
                break;
            }
        }

        if self.opts.try_get("render", 0)? == 1 {
            println!("{}", map);
        }
        answer!(n_dropped)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (mut map, start_x) = read_input(input, true)?;

        map[0][start_x] = Cell::Sand;
        let mut amount = 1;

        for y in 1..map.h - 1 {
            for x in start_x - y..=start_x + y {
                if map[y][x] != Cell::Air {
                    continue;
                }

                let (l, r) = (x.saturating_sub(1), min(map.w - 1, x + 1));
                if map[y - 1][l..=r].contains(&Cell::Sand) {
                    map[y][x] = Cell::Sand;
                    amount += 1
                }
            }
        }

        if self.opts.try_get("render", 0)? == 1 {
            println!("{}", map);
        }
        answer!(amount)
    }
}

struct Coord(usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

struct Map<T> {
    data: Vec<T>,
    w: usize,
    h: usize,
}

impl Map<Cell> {
    fn new(w: usize, h: usize) -> Self {
        Self {
            data: vec![Cell::Air; w * h],
            w,
            h,
        }
    }
}

impl<T> Index<usize> for Map<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.w..(index + 1) * self.w]
    }
}

impl<T> IndexMut<usize> for Map<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.w..(index + 1) * self.w]
    }
}

impl Display for Map<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[ w = {}, h = {} ]", self.w, self.h)?;
        writeln!(f, "╭{}╮", "─".repeat(self.w))?;

        for r in (0..self.h).step_by(2) {
            f.write_char('│')?;

            for c in 0..self.w {
                f.write_str(if r + 1 < self.h {
                    match (self[r][c], self[r + 1][c]) {
                        (Cell::Air, Cell::Air) => " ",
                        (Cell::Air, Cell::Rock) => "▄",
                        (Cell::Air, Cell::Sand) => "\x1b[33m▄\x1b[0m",
                        (Cell::Rock, Cell::Air) => "▀",
                        (Cell::Rock, Cell::Rock) => "█",
                        (Cell::Rock, Cell::Sand) => "\x1b[43m▀\x1b[0m",
                        (Cell::Sand, Cell::Air) => "\x1b[33m▀\x1b[0m",
                        (Cell::Sand, Cell::Rock) => "\x1b[43m▄\x1b[0m",
                        (Cell::Sand, Cell::Sand) => "\x1b[33m█\x1b[0m",
                    }
                } else {
                    match self[r][c] {
                        Cell::Air => " ",
                        Cell::Rock => "▀",
                        Cell::Sand => "\x1b[33m▀\x1b[0m",
                    }
                })?;
            }
            writeln!(f, "│")?;
        }
        writeln!(f, "╰{}╯", "─".repeat(self.w))
    }
}

fn read_input<R: BufRead>(input: &mut R, expand: bool) -> anyhow::Result<(Map<Cell>, usize)> {
    let mut min_x: usize = usize::MAX;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    let mut paths = vec![];
    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let path = line
            .split("->")
            .map(|s| -> anyhow::Result<Coord> {
                let mut s = s.trim_ascii().splitn(2, ',');
                let x: usize = s
                    .next()
                    .context("missing x value")?
                    .parse()
                    .context("parse x")?;
                let y: usize = s
                    .next()
                    .context("missing y value")?
                    .parse()
                    .context("parse y")?;

                min_x = min(min_x, x);
                max_x = max(max_x, x);
                max_y = max(max_y, y);
                Ok(Coord(x, y))
            })
            .collect::<Result<Vec<_>, _>>()?;
        paths.push(path);
    }

    let mut x_shift = min_x as isize;
    let (mut w, mut h) = (max_x.strict_sub_signed(x_shift) + 1, max_y + 1);

    if expand {
        // As per part 2 description
        max_y += 2;
        x_shift = min(500 - (max_y as isize - 1), min_x as isize);
        w = max(max_x.strict_sub_signed(x_shift) + 1, max_y * 2 - 1);
        h = max_y + 1;
    }

    let mut map: Map<Cell> = Map::new(w, h);

    // Draw rocky paths
    for path in &paths {
        let (mut last_x, mut last_y) = (path[0].0.strict_sub_signed(x_shift), path[0].1);
        map[last_y][last_x] = Cell::Rock;

        for p in &path[1..] {
            let (next_x, next_y) = (p.0.strict_sub_signed(x_shift), p.1);

            for ty in min(next_y, last_y)..=max(next_y, last_y) {
                for tx in min(next_x, last_x)..=max(next_x, last_x) {
                    map[ty][tx] = Cell::Rock;
                }
            }
            (last_x, last_y) = (next_x, next_y);
        }
    }
    if expand {
        for x in 0..w {
            map[h - 1][x] = Cell::Rock
        }
    }

    Ok((map, 500usize.strict_sub_signed(x_shift)))
}
