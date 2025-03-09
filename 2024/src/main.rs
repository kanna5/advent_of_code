use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, stdin, BufRead, BufReader},
};

use anyhow::Context;
use aoc2024::solutions::{get_solution, Options};

#[derive(Debug, thiserror::Error)]
#[error("Invalid usage: {0}")]
struct UsageError(String);

enum Part {
    One,
    Two,
}

impl TryFrom<String> for Part {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            v => Err(format!(
                "Part \"{}\" is not available. Valid options are 1, 2",
                v
            )),
        }
    }
}

fn from_file(path: &str) -> io::Result<Box<dyn BufRead>> {
    let f = File::open(path)?;
    Ok(Box::new(BufReader::new(f)))
}

fn main() -> Result<(), anyhow::Error> {
    let mut args = env::args().skip(1);
    let day = args
        .next()
        .ok_or(UsageError("Missing argument: day".into()))?
        .parse::<u8>()
        .map_err(|_| UsageError("Invalid value: day".into()))?;
    let part: Part = args
        .next()
        .ok_or(UsageError("Missing argument: part".into()))?
        .try_into()
        .map_err(UsageError)?;

    let mut options: HashMap<String, String> = HashMap::with_capacity(4);
    let mut input_path: Option<String> = None;
    for opt in args {
        let Some(parts) = opt.split_once('=') else {
            match input_path {
                Some(_) => {
                    return Err(UsageError(format!(
                        "Invalid option \"{}\". Options should be specified in key=value format",
                        opt
                    ))
                    .into())
                }
                None => input_path = Some(opt),
            };
            continue;
        };

        if parts.0.is_empty() {
            return Err(
                UsageError(format!("Invalid option \"{}\". Key cannot be empty.", opt)).into(),
            );
        }
        options.insert(parts.0.to_string(), parts.1.to_string());
    }

    let sol = get_solution(day, Options::new(options))
        .with_context(|| format!("No solution implemented for day {}", day))?;

    let mut input: Box<dyn BufRead> = match input_path.as_deref() {
        None => {
            let path = format!("input/day-{:02}.txt", day);
            from_file(&path).with_context(|| format!("Cannot read input from {}", path))?
        }
        Some("-") => {
            eprintln!("Reading from stdin");
            Box::new(stdin().lock())
        }
        Some(pth) => {
            from_file(pth).with_context(|| format!("Cannot read from input file \"{pth}\""))?
        }
    };

    let answer = match part {
        Part::One => sol.part1(&mut input),
        Part::Two => sol.part2(&mut input),
    }?;

    println!("{}", answer);
    Ok(())
}
