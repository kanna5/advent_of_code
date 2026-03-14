//! Implements a solution for https://adventofcode.com/2022/day/12

use std::{
    collections::VecDeque,
    io::BufRead,
    ops::{Index, IndexMut},
};

use crate::{answer, solutions::Solution};
use anyhow::bail;

pub struct Day12;

const DIRS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl<R: BufRead> Solution<R> for Day12 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut map = read_input(input)?;
        let (start_x, start_y) = map.start;
        let (end_x, end_y) = map.end;

        // x, y, steps
        let mut queue = VecDeque::<(usize, usize, usize)>::with_capacity(128);
        queue.push_back((start_x, start_y, 0));
        map[start_y][start_x].visited = true;
        while let Some((x, y, steps)) = queue.pop_front() {
            for (dx, dy) in DIRS {
                let (nx, ny) = (x as i64 + dx, y as i64 + dy);

                if nx < 0 || ny < 0 || nx as usize >= map.w || ny as usize >= map.h {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);

                if map[ny][nx].visited || map[ny][nx].height > map[y][x].height + 1 {
                    continue;
                }
                if (nx, ny) == (end_x, end_y) {
                    return answer!(steps + 1);
                }

                queue.push_back((nx, ny, steps + 1));
                map[ny][nx].visited = true;
            }
        }
        answer!(-1)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut map = read_input(input)?;
        let (end_x, end_y) = map.end;

        // Reverse walk, find the first spot with elevation 0
        let mut queue = VecDeque::<(usize, usize, usize)>::with_capacity(128);
        queue.push_back((end_x, end_y, 0));
        map[end_y][end_x].visited = true;
        while let Some((x, y, steps)) = queue.pop_front() {
            for (dx, dy) in DIRS {
                let (nx, ny) = (x as i64 + dx, y as i64 + dy);

                if nx < 0 || ny < 0 || nx as usize >= map.w || ny as usize >= map.h {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);

                if map[ny][nx].visited || map[y][x].height > map[ny][nx].height + 1 {
                    continue;
                }
                if map[ny][nx].height == 0 {
                    return answer!(steps + 1);
                }

                queue.push_back((nx, ny, steps + 1));
                map[ny][nx].visited = true;
            }
        }
        answer!(-1)
    }
}

struct Map<T> {
    data: Vec<T>,
    w: usize,
    h: usize,
    start: (usize, usize),
    end: (usize, usize),
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

struct Cell {
    height: u8,
    visited: bool,
}

impl Cell {
    fn new(height: u8) -> Self {
        Self {
            height,
            visited: false,
        }
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Map<Cell>, anyhow::Error> {
    let mut m = Map::<Cell> {
        data: Vec::with_capacity(1024),
        w: 0,
        h: 0,
        start: (0, 0),
        end: (0, 0),
    };
    let (mut has_start, mut has_end) = (false, false);

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        if m.w > 0 && line.len() != m.w {
            bail!("got variable line length")
        } else if m.w == 0 {
            m.w = line.len();
        }

        for (x, c) in line.bytes().enumerate() {
            match c {
                b'S' => {
                    m.start = (x, m.h);
                    m.data.push(Cell::new(0));
                    has_start = true;
                }
                b'E' => {
                    m.end = (x, m.h);
                    m.data.push(Cell::new(b'z' - b'a'));
                    has_end = true;
                }
                c if c.is_ascii_lowercase() => {
                    m.data.push(Cell::new(c - b'a'));
                }
                c => bail!("invalid character {:?}", c as char),
            }
        }
        m.h += 1;
    }
    if !has_start || !has_end {
        bail!("incomplete input: start or end not found")
    }
    Ok(m)
}
