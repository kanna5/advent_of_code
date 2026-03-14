//! Implements a solution for https://adventofcode.com/2022/day/8

use core::panic;
use std::{
    cmp,
    io::BufRead,
    ops::{Index, IndexMut},
};

use crate::{answer, solutions::Solution};
use anyhow::anyhow;

pub struct Day08;

impl<R: BufRead> Solution<R> for Day08 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let m = read_input(input)?;
        let mut visible = Map {
            data: vec![false; m.data.len()],
            w: m.w,
            h: m.h,
        };

        // top -> bottom
        let mut max_heights = vec![-1i8; cmp::max(m.w, m.h)];
        for y in 0..m.h {
            for x in 0..m.w {
                let h = m[(x, y)];
                if h > max_heights[x] {
                    max_heights[x] = h;
                    visible[(x, y)] = true;
                }
            }
        }

        // bottom -> top
        max_heights.fill(-1);
        for y in (0..m.h).rev() {
            for x in 0..m.w {
                let h = m[(x, y)];
                if h > max_heights[x] {
                    max_heights[x] = h;
                    visible[(x, y)] = true;
                }
            }
        }

        // left -> right
        max_heights.fill(-1);
        for x in 0..m.w {
            for y in 0..m.h {
                let h = m[(x, y)];
                if h > max_heights[y] {
                    max_heights[y] = h;
                    visible[(x, y)] = true;
                }
            }
        }

        // right -> left
        max_heights.fill(-1);
        for x in (0..m.w).rev() {
            for y in 0..m.h {
                let h = m[(x, y)];
                if h > max_heights[y] {
                    max_heights[y] = h;
                    visible[(x, y)] = true;
                }
            }
        }

        answer!(visible.data.iter().filter(|&i| *i).count())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let m = read_input(input)?;

        // A naive implementation, but it's good enough for the input size
        let score = |idx: (usize, usize)| {
            let mut ret = 1;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (mut x, mut y) = (idx.0 as isize, idx.1 as isize);
                let mut ret_t = 0;

                loop {
                    (x, y) = (x + dx, y + dy);
                    if x < 0 || y < 0 || x >= m.w as isize || y >= m.h as isize {
                        break;
                    }
                    ret_t += 1;
                    if m[(x as usize, y as usize)] >= m[idx] {
                        break;
                    }
                }
                ret *= ret_t
            }
            ret
        };

        let mut max_score = 0;
        for y in 1..m.h - 1 {
            for x in 1..m.w - 1 {
                max_score = cmp::max(max_score, score((x, y)))
            }
        }
        answer!(max_score)
    }
}

struct Map<T> {
    data: Vec<T>,
    w: usize,
    h: usize,
}

impl<T> Map<T> {
    fn idx(&self, x: usize, y: usize) -> usize {
        if x >= self.w || y >= self.h {
            panic!(
                "index out of range. w={}, h={}, requested x={}, y={}",
                self.w, self.h, x, y
            )
        }
        x + y * self.w
    }
}

impl<T> Index<(usize, usize)> for Map<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.idx(index.0, index.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Map<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let i = self.idx(index.0, index.1);
        &mut self.data[i]
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Map<i8>, anyhow::Error> {
    let mut m = Map {
        data: Vec::new(),
        w: 0,
        h: 0,
    };

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let w = line.len();
        if m.w == 0 {
            m.w = w
        } else if w != m.w {
            return Err(anyhow!("invalid input: got variable line length"));
        }

        m.data.reserve(w);
        for c in line.as_bytes() {
            if !c.is_ascii_digit() {
                return Err(anyhow!(
                    "invalid input \"{}\": got invalid character {}",
                    line,
                    c
                ));
            }
            m.data.push((*c - b'0') as i8);
        }
        m.h += 1;
    }
    Ok(m)
}
