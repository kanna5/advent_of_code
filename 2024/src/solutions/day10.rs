use super::{InvalidInput, Solution};
use std::{collections::HashSet, io::BufRead};

pub struct Day10;

impl<R: BufRead> Solution<R> for Day10 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let map = read_input(input)?;
        let sum: usize = map.zeros.iter().map(|v| map.score(v)).sum();
        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let map = read_input(input)?;
        let sum = map.rate_all();
        Ok(sum.to_string())
    }
}

type Coordinate = (usize, usize);
type MapCell = u8;

#[derive(Debug)]
struct QuizMap {
    width: usize,
    height: usize,
    data: Vec<MapCell>,
    zeros: Vec<Coordinate>,
}

impl QuizMap {
    fn get(&self, coord: &Coordinate) -> Option<&MapCell> {
        match self.contains(coord) {
            true => Some(&self.data[coord.1 * self.width + coord.0]),
            false => None,
        }
    }

    fn contains(&self, coord: &Coordinate) -> bool {
        (0..self.width).contains(&coord.0) && (0..self.height).contains(&coord.1)
    }

    fn neighbors(&self, coord: &Coordinate) -> Vec<Coordinate> {
        let mut ret = Vec::<Coordinate>::with_capacity(4);
        let &(x, y) = coord;
        if x > 0 {
            ret.push((x - 1, y));
        }
        if y > 0 {
            ret.push((x, y - 1));
        }
        if x < self.width - 1 {
            ret.push((x + 1, y));
        }
        if y < self.height - 1 {
            ret.push((x, y + 1));
        }
        ret
    }

    fn score(&self, coord: &Coordinate) -> usize {
        let mut search: Vec<Coordinate> = Vec::with_capacity(128);
        let mut cnt = 0usize;
        let mut visited: HashSet<Coordinate> = HashSet::with_capacity(128);
        search.push(*coord);
        visited.insert(*coord);

        while let Some(coord) = search.pop() {
            let val = self.get(&coord).unwrap();
            if *val == 9 {
                cnt += 1;
                continue;
            }

            self.neighbors(&coord).iter().for_each(|neighbor| {
                if visited.contains(neighbor) {
                    return;
                }
                match self.get(neighbor) {
                    Some(v) if *v == *val + 1 => {
                        search.push(*neighbor);
                        visited.insert(*neighbor);
                    }
                    _ => (),
                }
            });
        }
        cnt
    }

    fn rate_all(&self) -> usize {
        let mut scores = vec![0usize; self.height * self.width];
        let mut search: Vec<Coordinate> = Vec::with_capacity(128);
        search.extend(&self.zeros);
        let mut nines = HashSet::<Coordinate>::with_capacity(128);
        let tr = |coord: &Coordinate| coord.1 * self.width + coord.0;

        while let Some(coord) = search.pop() {
            let val = self.get(&coord).unwrap();

            if *val == 0 {
                scores[tr(&coord)] = 1
            } else {
                let new_score: usize = self
                    .neighbors(&coord)
                    .iter()
                    .filter(|c| *self.get(c).unwrap() == val - 1)
                    .map(|c| scores[tr(c)])
                    .sum();
                let old_score = &mut scores[tr(&coord)];
                if *old_score == new_score {
                    continue;
                }
                *old_score = new_score
            }

            match val {
                9 => {
                    nines.insert(coord);
                }
                val => self
                    .neighbors(&coord)
                    .iter()
                    .filter(|c| *self.get(c).unwrap() == val + 1)
                    .for_each(|c| search.push(*c)),
            }
        }

        nines.iter().map(|c| scores[tr(c)]).sum()
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<QuizMap, anyhow::Error> {
    let mut data = Vec::<MapCell>::with_capacity(4096);
    let (mut width, mut height) = (0, 0);
    let mut zeros = Vec::<Coordinate>::with_capacity(128);

    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let line = line.as_bytes();

        if width == 0 {
            width = line.len();
        } else if line.len() != width {
            return Err(InvalidInput("Got variable line length.".to_string()).into());
        }
        for (x, b) in line.iter().enumerate() {
            if !b.is_ascii_digit() {
                return Err(InvalidInput(format!("Got invalid character 0x{:2X}", b)).into());
            }
            let num = b - b'0';
            data.push(num);
            if num == 0 {
                zeros.push((x, height));
            }
        }
        height += 1;
    }

    Ok(QuizMap {
        width,
        height,
        data,
        zeros,
    })
}
