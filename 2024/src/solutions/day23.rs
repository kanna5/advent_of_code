use super::Solution;
use anyhow::{anyhow, Context};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    io::BufRead,
};

pub struct Day23;

impl<R: BufRead> Solution<R> for Day23 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let pairs = read_input(input)?;
        let conns = ConnMap::new(&pairs[..]);
        Ok(conns.find_n_trio_with_t().to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let pairs = read_input(input)?;
        let conns = ConnMap::new(&pairs[..]);

        let max_grp = (0..conns.name_to_id.len())
            .map(|i| conns.find_largest_group(i))
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();
        Ok(conns.group_password(&max_grp[..]))
    }
}

struct ConnPair(String, String);

impl TryFrom<&str> for ConnPair {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pair = value
            .split_once('-')
            .with_context(|| format!("invalid pair {:?}", value))?;
        if pair.0.len() != 2 || pair.1.len() != 2 {
            return Err(anyhow!("invalid pair {:?}: invalid length", value));
        };
        match pair.0.cmp(pair.1) {
            std::cmp::Ordering::Less => Ok(Self(pair.0.to_string(), pair.1.to_string())),
            _ => Ok(Self(pair.1.to_string(), pair.0.to_string())),
        }
    }
}

struct ConnMap {
    name_to_id: HashMap<String, usize>,
    connections: Vec<bool>,
}

impl ConnMap {
    fn new(pairs: &[ConnPair]) -> Self {
        let mut next_id: usize = 0;
        let mut name_to_id = HashMap::<String, usize>::with_capacity(1024);
        for pair in pairs {
            for name in [&pair.0, &pair.1] {
                if let Entry::Vacant(ent) = name_to_id.entry(name.clone()) {
                    ent.insert(next_id);
                    next_id += 1;
                };
            }
        }
        let len = name_to_id.len();
        let mut connections = vec![false; len * len];
        for pair in pairs {
            let (x, y) = (
                name_to_id.get(&pair.0).unwrap(),
                name_to_id.get(&pair.1).unwrap(),
            );
            connections[x * len + y] = true;
            connections[y * len + x] = true;
        }
        Self {
            name_to_id,
            connections,
        }
    }

    fn is_connected_id(&self, a: &usize, b: &usize) -> bool {
        self.connections[a * self.name_to_id.len() + b]
    }

    fn find_n_trio_with_t(&self) -> usize {
        let mut trios = HashSet::<[usize; 3]>::with_capacity(1024);

        for (_, a) in self.name_to_id.iter().filter(|v| v.0.starts_with('t')) {
            let connected: Vec<_> = (0..self.name_to_id.len())
                .filter(|v| self.is_connected_id(a, v))
                .collect();
            for (i, b) in connected.iter().enumerate() {
                for c in &connected[i + 1..] {
                    if self.is_connected_id(b, c) {
                        // a, b, c are interconnected
                        let mut trio = [*a, *b, *c];
                        trio.sort();
                        trios.insert(trio);
                    }
                }
            }
        }
        trios.len()
    }

    fn find_largest_group(&self, id: usize) -> Vec<usize> {
        let nodes: Vec<_> = (0..self.name_to_id.len())
            .filter(|v| self.is_connected_id(&id, v))
            .collect();

        let mut connected_groups = Vec::<Vec<usize>>::with_capacity(8192);
        for node in nodes.iter() {
            connected_groups.push(vec![*node]);
        }
        for node in nodes {
            for i in 0..connected_groups.len() {
                let g = &connected_groups[i][..];
                if g.iter().any(|b| !self.is_connected_id(&node, b)) {
                    continue;
                }
                let new_grp = [g, &[node][..]].concat();
                connected_groups.push(new_grp);
            }
        }
        let mut max_grp = connected_groups
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .clone();
        max_grp.push(id);
        max_grp
    }

    fn group_password(&self, group: &[usize]) -> String {
        let id_to_name: HashMap<usize, &str> = self
            .name_to_id
            .iter()
            .map(|(name, id)| (*id, name.as_str()))
            .collect();

        let mut names: Vec<_> = group
            .iter()
            .map(|v| id_to_name.get(v).cloned().unwrap_or(""))
            .collect();
        names.sort();
        names.join(",")
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<ConnPair>, anyhow::Error> {
    input
        .lines()
        .map_while(Result::ok)
        .take_while(|v| !v.is_empty())
        .map(|v| ConnPair::try_from(v.as_str()))
        .collect::<Result<Vec<_>, _>>()
}
