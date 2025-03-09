use super::Solution;
use std::io::BufRead;

pub struct Day25;

impl<R: BufRead> Solution<R> for Day25 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (locks, keys) = read_input(input)?;

        let mut sum = 0;
        for l in &locks {
            for k in &keys {
                if k.fits(l) {
                    sum += 1
                }
            }
        }
        Ok(sum.to_string())
    }

    fn part2(&self, _input: &mut R) -> Result<String, anyhow::Error> {
        Ok("Ho Ho Ho...".to_string())
    }
}

#[derive(Debug)]
struct Lock([u8; 5]);

#[derive(Debug)]
struct Key([u8; 5]);

impl Key {
    fn fits(&self, lock: &Lock) -> bool {
        !self.0.iter().zip(lock.0.iter()).any(|(k, l)| k + l > 5)
    }
}

fn read_input<R: BufRead>(input: &mut R) -> Result<(Vec<Lock>, Vec<Key>), anyhow::Error> {
    let mut keys = Vec::<Key>::with_capacity(1024);
    let mut locks = Vec::<Lock>::with_capacity(1024);

    let mut push = |buf: &mut [u8; 5], is_key: &mut Option<bool>| {
        match is_key {
            Some(true) => {
                buf.iter_mut().for_each(|v| *v -= 1);
                keys.push(Key(*buf))
            }
            Some(false) => locks.push(Lock(*buf)),
            None => (),
        }
        is_key.take();
    };

    let mut buf = [0u8; 5];
    let mut is_key: Option<bool> = None;
    for line in input.lines().map_while(Result::ok) {
        if line.is_empty() {
            push(&mut buf, &mut is_key);
            continue;
        }

        if is_key.is_none() {
            is_key = Some(!line.starts_with('#'));
            buf.iter_mut().for_each(|v| *v = 0);
            continue;
        }

        for (i, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                buf[i] += 1;
            }
        }
    }
    push(&mut buf, &mut is_key);
    Ok((locks, keys))
}
