//! Implements a solution for https://adventofcode.com/2022/day/7

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::BufRead,
};

use crate::{answer, solutions::Solution};
use anyhow::{Context, anyhow};

pub struct Day07;

impl<R: BufRead> Solution<R> for Day07 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let size_threshold = 100_000;
        let index = read_input(input)?;

        let mut sum = 0usize;
        fn chk(
            pth: String,
            idx: &HashMap<String, Dir>,
            sum: &mut usize,
            threshold: usize,
        ) -> usize {
            let dir = idx.get(&pth).unwrap();
            let mut sz = dir.files.values().sum::<usize>();

            // Descend
            for next in &dir.dirs {
                let mut npth = dir.path.clone();
                npth.push(next);
                sz += chk(npth.to_string(), idx, sum, threshold)
            }
            if sz <= threshold {
                *sum += sz;
            }
            sz
        }

        chk("/".into(), &index, &mut sum, size_threshold);
        answer!(sum)
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let max_usage: usize = 40_000_000;
        let index = read_input(input)?;

        let mut dir_usages: Vec<(String, usize)> = Vec::with_capacity(index.len());
        fn chk(
            pth: String,
            idx: &HashMap<String, Dir>,
            usages: &mut Vec<(String, usize)>,
        ) -> usize {
            let dir = idx.get(&pth).unwrap();
            let mut sz = dir.files.values().sum::<usize>();

            // Descend
            for next in &dir.dirs {
                let mut npth = dir.path.clone();
                npth.push(next);
                sz += chk(npth.to_string(), idx, usages)
            }
            usages.push((pth, sz));
            sz
        }

        chk("/".into(), &index, &mut dir_usages);
        let (_, total_usage) = dir_usages.last().unwrap();
        let needed_delete = total_usage.saturating_sub(max_usage);
        if needed_delete == 0 {
            return answer!(0);
        }

        dir_usages.sort_by_key(|a| a.1);
        for (_, sz) in &dir_usages {
            if *sz >= needed_delete {
                return answer!(sz);
            }
        }
        answer!(-1)
    }
}

#[derive(Clone, Debug)]
struct Path(Vec<String>);

impl Path {
    fn new() -> Self {
        Path(Vec::new())
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn push(&mut self, path: &str) {
        self.0.push(path.into());
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return f.write_str("/");
        }

        for part in &self.0 {
            f.write_str("/")?;
            f.write_str(part)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Dir {
    path: Path,
    dirs: HashSet<String>,
    files: HashMap<String, usize>,
}

impl Dir {
    fn new(path: Path) -> Self {
        Self {
            path,
            dirs: HashSet::new(),
            files: HashMap::new(),
        }
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<HashMap<String, Dir>, anyhow::Error> {
    let mut index = HashMap::<String, Dir>::new();
    let mut cwd = Path::new();
    index.insert("/".into(), Dir::new(Path::new()));

    for line in input.lines() {
        let line = line?;

        if let Some(cmdline) = line.strip_prefix('$') {
            let cmdline: Vec<&str> = cmdline.trim().splitn(2, ' ').map(|s| s.trim()).collect();
            if cmdline.is_empty() {
                return Err(anyhow!("empty command"));
            }

            if cmdline[0] == "cd" {
                if cmdline.len() != 2 {
                    return Err(anyhow!("invalid number of args for `cd`: {}", line));
                }

                let mut new_cwd = cwd.clone();
                match cmdline[1] {
                    ".." => new_cwd.pop(),
                    "/" => new_cwd = Path::new(),
                    s => new_cwd.push(s),
                }

                if !index.contains_key(&new_cwd.to_string()) {
                    let parent_dir = index.get_mut(&cwd.to_string()).unwrap();
                    parent_dir.dirs.insert(cmdline[1].to_string());

                    index.insert(new_cwd.to_string(), Dir::new(new_cwd.clone()));
                }
                cwd = new_cwd
            }
            continue;
        }

        // Parse output of `ls`
        let parts: Vec<&str> = line.splitn(2, ' ').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(anyhow!("invalid output of ls: {}", line));
        }
        match parts[0] {
            "dir" => {
                let cur_dir = index.get_mut(&cwd.to_string()).unwrap();
                cur_dir.dirs.insert(parts[1].into());

                let mut new_pth = cwd.clone();
                new_pth.push(parts[1]);
                index
                    .entry(new_pth.to_string())
                    .or_insert_with(|| Dir::new(new_pth));
            }
            s => {
                let sz: usize = s.parse().with_context(|| {
                    format!("invalid output of ls: failed to parse file size: {}", line)
                })?;

                let cur_dir = index.get_mut(&cwd.to_string()).unwrap();
                cur_dir.files.insert(parts[1].into(), sz);
            }
        }
    }
    Ok(index)
}
