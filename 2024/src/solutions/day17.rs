use super::Solution;
use anyhow::{anyhow, Context};
use std::{fmt::Write, io::BufRead};

pub struct Day17;

impl<R: BufRead> Solution<R> for Day17 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut executor = read_input(input)?;
        executor.run_to_end();
        Ok(executor
            .output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","))
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let mut executor = read_input(input)?;
        match find_a(&mut executor, 0, 0) {
            Some(v) => Ok(v.to_string()),
            None => Err(anyhow!("No solution could be found for this input.")),
        }
    }
}

type Register = i64;

#[derive(Debug)]
struct Executor {
    program: Vec<u8>,
    reg_a: Register,
    reg_b: Register,
    reg_c: Register,
    ip: usize,
    output: Vec<u8>,
}

impl Executor {
    fn _readable(&self) -> String {
        let mut ret = String::with_capacity(4096);
        let mut prog_iter = self.program.iter().enumerate();
        while let Some((i, &v)) = prog_iter.next() {
            let (op, is_combo) = match v {
                0 => ("adv", true),
                1 => ("bxl", false),
                2 => ("bst", true),
                3 => ("jnz", false),
                4 => ("bxc", true),
                5 => ("out", true),
                6 => ("bdv", true),
                7 => ("cdv", true),
                _ => panic!("invalid op_code {v}"),
            };
            let (_, &operand) = prog_iter.next().unwrap();
            let operand = match is_combo {
                true => match operand {
                    0..=3 => operand.to_string(),
                    4 => "reg_a".into(),
                    5 => "reg_b".into(),
                    6 => "reg_c".into(),
                    _ => panic!("invalid combo operand {operand}"),
                },
                false => operand.to_string(),
            };
            match op {
                "bxc" => writeln!(ret, "{:2}: {}", i, op).unwrap(),
                _ => writeln!(ret, "{:2}: {} {}", i, op, operand).unwrap(),
            }
        }
        ret
    }

    fn combo_resolve(&self, val: u8) -> Register {
        match val {
            0..=3 => val as Register,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            v => panic!("Found invalid operand {v}"),
        }
    }

    fn xdv(&self, operand: u8) -> Register {
        self.reg_a / (2 as Register).pow(self.combo_resolve(operand) as u32)
    }

    fn adv(&mut self, operand: u8) {
        self.reg_a = self.xdv(operand)
    }

    fn bdv(&mut self, operand: u8) {
        self.reg_b = self.xdv(operand)
    }

    fn cdv(&mut self, operand: u8) {
        self.reg_c = self.xdv(operand)
    }

    fn out(&mut self, operand: u8) {
        let val = self.combo_resolve(operand).rem_euclid(8) as u8;
        self.output.push(val);
    }

    fn bxl(&mut self, operand: u8) {
        self.reg_b ^= operand as Register
    }

    fn bst(&mut self, operand: u8) {
        self.reg_b = self.combo_resolve(operand).rem_euclid(8)
    }

    fn jnz(&mut self, operand: u8) {
        if self.reg_a != 0 {
            self.ip = operand as usize
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.reg_b ^= self.reg_c;
    }

    fn read_prog(&mut self) -> Option<u8> {
        let ret = self.program.get(self.ip).cloned();
        if ret.is_some() {
            self.ip += 1
        }
        ret
    }

    fn run_op(&mut self, opcode: u8, operand: u8) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            v => panic!("Invalid opcode {v}"),
        }
    }

    fn run_to_end(&mut self) {
        loop {
            let Some(opcode) = self.read_prog() else {
                break;
            };
            let Some(operand) = self.read_prog() else {
                break;
            };
            self.run_op(opcode, operand);
        }
    }

    fn run_until(&mut self, opcode: u8) {
        loop {
            let Some(current) = self.read_prog() else {
                break;
            };
            let Some(operand) = self.read_prog() else {
                break;
            };
            if current == opcode {
                break;
            }
            self.run_op(current, operand);
        }
    }

    fn reset(&mut self, reg_a: Register) {
        (self.reg_a, self.reg_b, self.reg_c) = (reg_a, 0, 0);
        self.output.clear();
        self.ip = 0;
    }
}

fn find_a(exe: &mut Executor, current_a: Register, current_pos: usize) -> Option<Register> {
    if current_pos == exe.program.len() {
        return Some(current_a);
    }
    let &expected = exe
        .program
        .get(exe.program.len() - 1 - current_pos)
        .unwrap();
    let a = current_a << 3;
    for da in 0..8 {
        exe.reset(a + da);
        exe.run_until(3); // JNZ
        let &output = exe.output.first()?;
        if output == expected {
            if let Some(v) = find_a(exe, a + da, current_pos + 1) {
                return Some(v);
            }
        }
    }
    None
}

fn read_input<R: BufRead>(input: &mut R) -> Result<Executor, anyhow::Error> {
    let mut ret = Executor {
        program: Vec::with_capacity(128),
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        ip: 0,
        output: Vec::with_capacity(128),
    };

    for line in input
        .lines()
        .map_while(Result::ok)
        .filter(|v| !v.is_empty())
    {
        let (k, v) = line
            .split_once(':')
            .with_context(|| format!("Invalid input: {line}"))?;
        let v = v.trim();

        match k {
            "Register A" => ret.reg_a = v.parse()?,
            "Register B" => ret.reg_b = v.parse()?,
            "Register C" => ret.reg_c = v.parse()?,
            "Program" => ret.program = v.split(',').map(|v| v.parse()).collect::<Result<_, _>>()?,
            _ => return Err(anyhow!("Unknown key {k}")),
        }
    }
    Ok(ret)
}
