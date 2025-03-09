use super::Solution;
use std::{collections::HashMap, io::BufRead};

pub struct Day21;

impl<R: BufRead> Solution<R> for Day21 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let nums = read_input(input)?;
        let sum: i64 = nums
            .iter()
            .map(|(seq, num)| {
                let moves = expand_dirpad(&expand_dirpad(&expand_numpad(seq)));
                moves.len() as i64 * num
            })
            .sum();
        Ok(sum.to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let nums = read_input(input)?;
        let mut sum = 0;
        for (seq, num) in nums {
            let mut mem = HashMap::<(u8, u8, usize), usize>::with_capacity(4096);
            let seq = expand_numpad(&seq);
            let mut len = 0;

            let positions = get_btn_coords(&DIRPAD, 3);
            let mut last = b'A';
            for b in seq.as_bytes() {
                len += find_n_steps(last, *b, 25, &positions, &mut mem);
                last = *b;
            }
            sum += len as i64 * num;
        }
        Ok(sum.to_string())
    }
}

type Coord = (isize, isize);
const DIRPAD: [u8; 6] = [b' ', b'^', b'A', b'<', b'v', b'>'];
const NUMPAD: [u8; 12] = [
    b'7', b'8', b'9', b'4', b'5', b'6', b'1', b'2', b'3', b' ', b'0', b'A',
];

fn write_moves(c1: &Coord, c2: &Coord, positions: &HashMap<u8, Coord>, buf: &mut String) {
    let (dx, dy) = (c2.0 - c1.0, c2.1 - c1.1);
    let gap_pos = positions.get(&b' ').cloned().unwrap();

    // idk why but this is the specific priority: "<^v>" or "<v^>"
    let mut pending_moves = Vec::<(char, isize, Coord)>::with_capacity(2);
    if dx < 0 {
        pending_moves.push(('<', dx.abs(), (dx, 0)));
    }
    if dy < 0 {
        pending_moves.push(('^', dy.abs(), (0, dy)));
    }
    if dy > 0 {
        pending_moves.push(('v', dy.abs(), (0, dy)));
    }
    if dx > 0 {
        pending_moves.push(('>', dx, (dx, 0)));
    }
    if let Some((_, _, d)) = pending_moves.first() {
        if (c1.0 + d.0, c1.1 + d.1) == gap_pos {
            pending_moves.reverse(); // avoid the gap
        }
    }
    for (c, n, _) in pending_moves {
        (0..n).for_each(|_| buf.push(c));
    }
}

fn get_btn_coords(layout: &[u8], width: usize) -> HashMap<u8, Coord> {
    layout
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, ((i % width) as isize, (i / width) as isize)))
        .collect()
}

fn expand(layout: &[u8], width: usize, seq: &str) -> String {
    let btn_coords = get_btn_coords(layout, width);

    let mut ret = String::with_capacity(256);
    let mut last_pos = btn_coords.get(&b'A').unwrap();
    seq.as_bytes().iter().for_each(|b| {
        let pos = btn_coords
            .get(b)
            .unwrap_or_else(|| panic!("Got unexpected character 0x{:02x}", b));
        write_moves(last_pos, pos, &btn_coords, &mut ret);
        ret.push('A');
        last_pos = pos;
    });
    ret
}

fn expand_numpad(seq: &str) -> String {
    expand(&NUMPAD, 3, seq)
}

fn expand_dirpad(seq: &str) -> String {
    expand(&DIRPAD, 3, seq)
}

fn find_n_steps(
    a: u8,
    b: u8,
    level: usize,
    positions: &HashMap<u8, Coord>,
    mem: &mut HashMap<(u8, u8, usize), usize>,
) -> usize {
    if level == 0 {
        return 1;
    }
    if let Some(v) = mem.get(&(a, b, level)) {
        return *v;
    }

    let mut buf = String::with_capacity(8);
    write_moves(
        positions.get(&a).unwrap(),
        positions.get(&b).unwrap(),
        positions,
        &mut buf,
    );
    buf.push('A');
    let mut last = b'A';
    let mut sum = 0;
    for b in buf.as_bytes() {
        sum += find_n_steps(last, *b, level - 1, positions, mem);
        last = *b;
    }

    mem.insert((a, b, level), sum);
    sum
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Vec<(String, i64)>, anyhow::Error> {
    input
        .lines()
        .map_while(Result::ok)
        .take_while(|s| !s.is_empty())
        .map(|line| {
            let num: i64 = line.trim_start_matches('0').trim_end_matches('A').parse()?;
            Ok((line, num))
        })
        .collect()
}
