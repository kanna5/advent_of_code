use super::{InvalidInput, Solution};
use std::{
    collections::HashSet,
    io::{BufRead, Read},
};

pub struct Day9;

impl<R: BufRead> Solution<R> for Day9 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let dm = read_input(input)?;
        let compacted = dm.compact_frag();
        Ok(compacted.checksum().to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let dm = read_input(input)?;
        let compacted = dm.compact_nofrag();
        Ok(compacted.checksum().to_string())
    }
}

#[derive(Debug)]
struct DiskMap(Vec<Slice>);

impl DiskMap {
    fn checksum(&self) -> u64 {
        let (mut sum, mut left) = (0, 0);
        for s in &self.0 {
            if let Some(id) = s.file_id {
                sum += ((left * 2 + s.length - 1) * s.length / 2 * id) as u64;
            }
            left += s.length;
        }
        sum
    }

    fn compact_frag(self) -> Self {
        let fwd = self.0.iter().enumerate();
        let mut bwd = self.0.iter().enumerate().rev();
        let mut compacted = Vec::<Slice>::with_capacity(self.0.len());

        let mut right = self.0.len();
        let mut partial_file: Option<Slice> = None;

        'outer: for (left, s) in fwd {
            if left >= right {
                break;
            }
            if s.has_file() {
                compacted.push(s.clone());
                continue;
            }
            let mut free = s.length; // the slice is free space
            while free > 0 {
                let mut filler = partial_file.take();
                if filler.is_none() {
                    for (r, v) in bwd.by_ref() {
                        right = r;
                        if left >= right {
                            break;
                        }
                        if v.has_file() {
                            filler = Some(v.clone());
                            break;
                        }
                    }
                }
                match filler {
                    Some(f) => {
                        let flen = f.length;
                        if flen > free {
                            compacted.push(Slice {
                                length: free,
                                file_id: f.file_id,
                            });
                            partial_file = Some(Slice {
                                length: flen - free,
                                file_id: f.file_id,
                            })
                        } else {
                            compacted.push(f);
                        }
                        free = free.saturating_sub(flen);
                    }
                    None => break 'outer, // free space left but no more file
                }
            }
        }

        if let Some(f) = partial_file {
            compacted.push(f);
        }
        DiskMap(compacted)
    }

    fn compact_nofrag(self) -> Self {
        let mut free_spaces: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(pos, s)| match s.has_file() {
                true => None,
                false => Some((pos, s.length, Option::<Vec<&Slice>>::None)),
            })
            .collect();
        let mut moved = HashSet::<usize>::with_capacity(512);

        for (pos, s) in self.0.iter().enumerate().rev() {
            if !s.has_file() {
                continue;
            }
            for free in free_spaces.iter_mut() {
                if free.0 >= pos {
                    break;
                }
                if free.1 < s.length {
                    continue;
                }
                let fillers = free.2.get_or_insert_with(|| Vec::with_capacity(9));
                fillers.push(s);
                free.1 -= s.length;
                moved.insert(pos);
                break;
            }
        }

        let mut ret: Vec<Slice> = Vec::with_capacity(self.0.len() + moved.len());
        let mut nth_free: usize = 0;
        for (pos, s) in self.0.iter().enumerate() {
            match s.file_id {
                Some(_) => match moved.contains(&pos) {
                    true => ret.push(Slice::empty(s.length)),
                    false => ret.push(s.clone()),
                },
                None => {
                    let (_, len, fillers) = &free_spaces[nth_free];
                    for filler in fillers.as_deref().unwrap_or_default() {
                        ret.push(Slice::clone(filler));
                    }
                    if *len > 0 {
                        ret.push(Slice::empty(*len));
                    }
                    nth_free += 1
                }
            }
        }
        DiskMap(ret)
    }
}

#[derive(Debug, Clone)]
struct Slice {
    length: u32,
    file_id: Option<u32>,
}

impl Slice {
    fn has_file(&self) -> bool {
        self.file_id.is_some()
    }

    fn empty(len: u32) -> Self {
        Self {
            length: len,
            file_id: None,
        }
    }
}

fn read_input<R: Read>(input: &mut R) -> Result<DiskMap, anyhow::Error> {
    let mut next_id = 0u32..;
    let mut next_is_file = true;
    let mut ret = Vec::with_capacity(4096);

    for b in input.bytes().map_while(|i| match i {
        Ok(b'\r') | Ok(b'\n') | Err(_) => None,
        Ok(v) => Some(v),
    }) {
        if !b.is_ascii_digit() {
            return Err(InvalidInput(format!("Found invalid byte 0x{:X}", b)).into());
        }
        let len = b - b'0';

        let slice = Slice {
            length: len as u32,
            file_id: match next_is_file {
                true => next_id.next(),
                false => None,
            },
        };
        if slice.length > 0 {
            ret.push(slice);
        }
        next_is_file = !next_is_file
    }
    Ok(DiskMap(ret))
}
