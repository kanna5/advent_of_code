use super::{InvalidInput, Options, Solution};
use anyhow::anyhow;
use std::{collections::VecDeque, io::BufRead};

pub struct Day18 {
    pub options: Options,
}

impl<R: BufRead> Solution<R> for Day18 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let drops = read_input(input)?;
        let width: usize = self.options.try_get("WIDTH", 71)?;
        let height: usize = self.options.try_get("HEIGHT", 71)?;
        let n_drops: usize = self.options.try_get("DROPS", 1024)?;

        let mut map = Map::new(width, height);
        drops[0..n_drops]
            .iter()
            .map(|c| map.block(c))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(map.walk().to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let drops = read_input(input)?;
        let width: usize = self.options.try_get("WIDTH", 71)?;
        let height: usize = self.options.try_get("HEIGHT", 71)?;
        let skip: usize = self.options.try_get("SKIP", 1024)?;

        let mut map = Map::new(width, height);
        for (i, drop) in drops.iter().enumerate() {
            map.block(drop)?;
            if i < skip {
                continue;
            }
            if map.walk() == UNREACHABLE {
                return Ok(format!("{},{}", drop.0, drop.1));
            }
            map.reset_cost();
        }
        Err(anyhow!("Not blocked"))
    }
}

const UNREACHABLE: u64 = u64::MAX;
type Coord = (i64, i64);

#[derive(Debug, Clone)]
struct Cell {
    cost: u64,
    blocked: bool,
}

impl Cell {
    fn new(blocked: bool) -> Self {
        Self {
            cost: UNREACHABLE,
            blocked,
        }
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![Cell::new(false); width * height],
            width,
            height,
        }
    }

    fn reset_cost(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.cost = UNREACHABLE;
        }
    }

    fn block(&mut self, c: &Coord) -> Result<(), anyhow::Error> {
        let Some(idx) = self.idx(c) else {
            return Err(anyhow!(
                "Cell out of bound: {:?}. Map size: ({}, {})",
                c,
                self.width,
                self.height
            ));
        };
        self.cells[idx].blocked = true;
        Ok(())
    }

    fn contains(&self, c: &Coord) -> bool {
        (0..self.width as i64).contains(&c.0) && (0..self.height as i64).contains(&c.1)
    }

    fn idx(&self, c: &Coord) -> Option<usize> {
        match self.contains(c) {
            true => Some(c.1 as usize * self.width + c.0 as usize),
            false => None,
        }
    }

    fn walk(&mut self) -> u64 {
        self.cells[0].cost = 0;
        let mut queue = VecDeque::<Coord>::with_capacity(256);
        queue.push_back((0, 0));

        while let Some(c) = queue.pop_front() {
            let next_cost = self.cells[self.idx(&c).unwrap()].cost + 1;
            for next in [(0i64, 1i64), (1, 0), (0, -1), (-1, 0)]
                .iter()
                .map(|d| (c.0 + d.0, c.1 + d.1))
            {
                if !self.contains(&next) {
                    continue;
                }
                let idx = self.idx(&next).unwrap();
                let cell = &mut self.cells[idx];
                if cell.blocked {
                    continue;
                }
                if next_cost < cell.cost {
                    cell.cost = next_cost;
                    queue.push_back(next);
                }
            }
        }

        self.cells[self
            .idx(&(self.width as i64 - 1, self.height as i64 - 1))
            .unwrap()]
        .cost
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<Coord>, anyhow::Error> {
    let mut ret: Vec<Coord> = Vec::with_capacity(256);

    for line in input
        .lines()
        .map_while(Result::ok)
        .take_while(|v| !v.is_empty())
    {
        let Some(parts) = line.split_once(',') else {
            return Err(InvalidInput("expected two numbers".into()).into());
        };
        ret.push((parts.0.parse()?, parts.1.parse()?));
    }
    Ok(ret)
}
