use super::{InvalidInput, Solution};
use std::{collections::HashSet, io::BufRead};

pub struct Day6;

impl<R: BufRead> Solution<R> for Day6 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (map, guard) = read_input(input)?;
        let trail = get_trail(&map, &guard);
        let uniq_places: HashSet<(i32, i32)> = HashSet::from_iter(trail.data.iter().cloned());
        Ok(uniq_places.len().to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (map, guard) = read_input(input)?;
        let trail = get_trail(&map, &guard);
        let uniq_places: HashSet<(i32, i32)> = HashSet::from_iter(trail.data.iter().cloned());

        let count: i32 = uniq_places
            .iter()
            .map(|&(x, y)| -> i32 {
                if (x, y) == (guard.x, guard.y) {
                    return 0;
                }
                let map_t = map.with_obstacle_at(x, y);
                let trail_t = get_trail(&map_t, &guard);
                trail_t.is_loop as i32
            })
            .sum();
        Ok(count.to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, thiserror::Error)]
#[error("Not a valid direction: '{0}'")]
struct ParseDirectionError(char);

impl TryFrom<&char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            _ => Err(ParseDirectionError(*value)),
        }
    }
}

impl Direction {
    /// Returns (x, y)
    fn get_move(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GuardState {
    x: i32,
    y: i32,
    d: Direction,
}

impl GuardState {
    fn do_move(&self) -> Self {
        let (dx, dy) = self.d.get_move();
        Self {
            x: self.x + dx,
            y: self.y + dy,
            d: self.d,
        }
    }

    fn do_turn(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: self.d.turn_right(),
        }
    }

    fn front(&self) -> (i32, i32) {
        let (dx, dy) = self.d.get_move();
        (self.x + dx, self.y + dy)
    }
}

#[derive(Debug, Clone)]
enum MapTile {
    Empty,
    Occupied,
}

#[derive(Debug, thiserror::Error)]
#[error("Not a valid tile: '{0}'")]
struct ParseMapTileError(char);

impl TryFrom<&char> for MapTile {
    type Error = ParseMapTileError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Occupied),
            '.' => Ok(Self::Empty),
            _ => Err(ParseMapTileError(*value)),
        }
    }
}

#[derive(Default, Debug)]
struct Map {
    data: Vec<MapTile>,
    width: i32,
    height: i32,
}

impl Map {
    fn get(&self, x: i32, y: i32) -> Option<&MapTile> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(&self.data[(y * self.width + x) as usize])
    }

    fn with_obstacle_at(&self, x: i32, y: i32) -> Self {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            panic!("coordinate ({}, {}) out of bound", x, y)
        }
        let mut data = self.data.clone();
        data[(y * self.width + x) as usize] = MapTile::Occupied;
        Self {
            data,
            width: self.width,
            height: self.height,
        }
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<(Map, GuardState), InvalidInput> {
    let mut map_data: Vec<MapTile> = Vec::with_capacity(4096);
    let mut guard: Option<GuardState> = None;

    let (mut cx, mut cy) = (0i32, 0i32);
    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        cy += 1;
        if cx > 0 && line.len() as i32 != cx {
            return Err(InvalidInput("Got variable line length.".to_string()));
        }
        cx = 0;
        for c in line.chars() {
            cx += 1;
            if let Ok(t) = MapTile::try_from(&c) {
                map_data.push(t);
                continue;
            }
            if let Ok(d) = Direction::try_from(&c) {
                map_data.push(MapTile::Empty);
                guard = Some(GuardState {
                    x: cx - 1,
                    y: cy - 1,
                    d,
                });
                continue;
            }
            return Err(InvalidInput(format!("Unrecognized input: '{}'", c)));
        }
    }

    if let Some(guard) = guard {
        return Ok((
            Map {
                data: map_data,
                width: cx,
                height: cy,
            },
            guard,
        ));
    }
    Err(InvalidInput(
        "Cannot find initial state of the guard.".to_string(),
    ))
}

struct Trail {
    is_loop: bool,
    data: Vec<(i32, i32)>,
}

fn get_trail(map: &Map, guard: &GuardState) -> Trail {
    let mut guard = guard.clone();
    let mut states: HashSet<GuardState> = HashSet::with_capacity(4096);
    states.insert(guard.clone());

    let mut trail = Trail {
        is_loop: false,
        data: Vec::with_capacity(4096),
    };
    trail.data.push((guard.x, guard.y));

    loop {
        let (x, y) = guard.front();
        let next_state = match map.get(x, y) {
            None => break,
            Some(MapTile::Empty) => guard.do_move(),
            Some(MapTile::Occupied) => guard.do_turn(),
        };

        if states.contains(&next_state) {
            trail.is_loop = true;
            break;
        }
        states.insert(next_state.clone());
        trail.data.push((next_state.x, next_state.y));
        guard = next_state
    }
    trail
}
