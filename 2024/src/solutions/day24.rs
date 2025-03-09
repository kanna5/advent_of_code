use super::Solution;
use anyhow::{anyhow, Context};
use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
    iter,
    rc::Rc,
};

/// Got some inspirations for part 2 from Reddit:
/// First check each bit (full-adder) separately to find the misbehaving ones, and collect
/// candidates (output wires) for swapping (this greatly shrinks the search space). Then, iterate
/// through the candidates to find a pair that, when swapped, reduces the most error bits. Apply
/// that swap and repeat until there are no errors left.
pub struct Day24;

impl<R: BufRead> Solution<R> for Day24 {
    fn part1(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (wires, mut state, gates) = read_input(input)?;

        let c = Circuit::new(Rc::from(wires), gates);
        Ok(c.run(&mut state).to_string())
    }

    fn part2(&self, input: &mut R) -> Result<String, anyhow::Error> {
        let (wires, _, gates) = read_input(input)?;
        let wires = Rc::from(wires);

        let mut c = Circuit::new(Rc::clone(&wires), gates);
        let mut swapped = vec![false; wires.len()];

        loop {
            let candidates = find_swap_candidates(&c);
            let errs = check_err_bits(&c);
            let errs_n = errs.count_ones();
            if errs_n == 0 {
                break;
            }
            let mut max_reduction: Option<(usize, (usize, usize), Circuit)> = None;

            for (i, a) in candidates.iter().enumerate() {
                for b in candidates.iter().skip(i + 1) {
                    let Some(cc) = c.with_swap(*a, *b) else {
                        continue;
                    };
                    let errs_swapped = check_err_bits(&cc);
                    let errs_swapped_n = errs_swapped.count_ones();
                    if errs | errs_swapped == errs && errs_swapped_n < errs_n {
                        let r = (errs_n - errs_swapped_n) as usize;
                        match max_reduction {
                            Some((v, _, _)) if r <= v => (),
                            _ => max_reduction = Some((r, (*a, *b), cc)),
                        }
                    }
                }
            }
            let Some((_, (a, b), circuit)) = max_reduction else {
                return Err(anyhow!("Could not find solution."));
            };
            (c, swapped[a], swapped[b]) = (circuit, true, true);
        }

        let mut names: Vec<_> = swapped
            .iter()
            .enumerate()
            .filter(|(_, &v)| v)
            .map(|(i, _)| wires.id_to_name.get(&i).unwrap().as_str())
            .collect();
        names.sort();
        Ok(names.join(","))
    }
}

type WireStates = [bool];
type WireId = usize;

#[derive(Debug)]
struct WireSet {
    next_id: usize,
    id_to_name: HashMap<WireId, String>,
    name_to_id: HashMap<String, WireId>,
    lengths: [usize; 3],
}

impl WireSet {
    fn new() -> Self {
        Self {
            next_id: 64 * 3,
            id_to_name: HashMap::with_capacity(64),
            name_to_id: HashMap::with_capacity(64),
            lengths: [0; 3],
        }
    }

    fn register(&mut self, name: &str) -> WireId {
        if let Some(id) = self.name_to_id.get(name) {
            return *id;
        }

        let offset = match name.as_bytes()[0] {
            b'x' => Some(0usize),
            b'y' => Some(1),
            b'z' => Some(2),
            _ => None,
        };
        let mut id = self.next_id;
        if let Some(offset) = offset {
            if let Ok(idx) = name[1..].parse::<usize>() {
                self.lengths[offset] = (idx + 1).max(self.lengths[offset]);
                id = offset * 64 + idx
            }
        } else {
            self.next_id += 1;
        }
        self.id_to_name.insert(id, name.to_string());
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    fn z_bits<'a>(&self, state: &'a WireStates) -> &'a WireStates {
        &state[64 * 2..64 * 2 + self.lengths[2]]
    }

    fn len(&self) -> usize {
        self.next_id
    }
}

#[derive(Debug, Clone)]
struct LogicGate {
    iw: (WireId, WireId),
    ow: WireId,
    op: LogicOp,
}

impl LogicGate {
    fn output(&self, wire_states: &WireStates) -> (WireId, bool) {
        let a = wire_states[self.iw.0];
        let b = wire_states[self.iw.1];
        (self.ow, self.op.calc(a, b))
    }
}

#[derive(Debug, Clone)]
enum LogicOp {
    And,
    Or,
    Xor,
}

impl LogicOp {
    fn calc(&self, a: bool, b: bool) -> bool {
        match self {
            LogicOp::And => a & b,
            LogicOp::Or => a | b,
            LogicOp::Xor => a ^ b,
        }
    }
}

impl TryFrom<&str> for LogicOp {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            v => Err(anyhow!("invalid operation {v:?}"))?,
        })
    }
}

fn pack(bits: &WireStates) -> u64 {
    bits.iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .fold(0u64, |s, (i, _)| s | (1 << i))
}

struct Circuit {
    wires: Rc<WireSet>,
    gates: Vec<LogicGate>,

    wire_gates: Rc<Vec<Vec<usize>>>,
}

impl Circuit {
    fn new(wires: Rc<WireSet>, gates: Vec<LogicGate>) -> Self {
        let mut wire_gates: Vec<Vec<usize>> = vec![Vec::new(); wires.len()];
        for (i, g) in gates.iter().enumerate() {
            for j in [g.iw.0, g.iw.1] {
                wire_gates[j].push(i);
            }
        }

        Self {
            wires,
            gates,
            wire_gates: Rc::from(wire_gates),
        }
    }

    /// Run the circuit and return the Z value
    fn run(&self, states: &mut [bool]) -> u64 {
        let mut queue: VecDeque<usize> = states
            .iter()
            .enumerate()
            .filter(|v| *v.1)
            .flat_map(|v| self.wire_gates[v.0].iter().cloned())
            .collect();

        while let Some(g) = queue.pop_front() {
            let (ow, val) = self.gates[g].output(states);
            if states[ow] != val {
                states[ow] = val;
                queue.extend(&self.wire_gates[ow]);
            }
        }

        pack(self.wires.z_bits(states))
    }

    /// Returns a new circuit with w1 and w2 swapped. Returns None when the swap creates a loop.
    fn with_swap(&self, w1: WireId, w2: WireId) -> Option<Self> {
        let new_gates: Vec<_> = self
            .gates
            .iter()
            .map(|v| LogicGate {
                iw: v.iw,
                ow: match v.ow {
                    x if x == w1 => w2,
                    x if x == w2 => w1,
                    x => x,
                },
                op: v.op.clone(),
            })
            .collect();

        // Detect loops
        let mut output_seen = vec![false; self.wires.len()];
        let mut queue = VecDeque::<usize>::with_capacity(new_gates.len());
        for w in [w1, w2] {
            output_seen[w] = true;
            queue.extend(&self.wire_gates[w]);
            while let Some(gate_idx) = queue.pop_front() {
                let ow = new_gates[gate_idx].ow;
                if output_seen[ow] {
                    return None;
                }
                queue.extend(&self.wire_gates[ow]);
            }
            if output_seen[w2] {
                break;
            }

            output_seen.fill(false);
            queue.clear();
        }

        Some(Self {
            wires: Rc::clone(&self.wires),
            gates: new_gates,
            wire_gates: Rc::clone(&self.wire_gates),
        })
    }
}

fn find_swap_candidates(c: &Circuit) -> Vec<WireId> {
    assert_eq!(c.wires.lengths[0], c.wires.lengths[1]);
    let input_len = c.wires.lengths[0];
    let mut state = vec![false; c.wires.len()];
    let mut candidate = vec![false; state.len()];

    for bit_idx in 0..input_len {
        for test_mode in [[true, false], [false, true], [true, true]] {
            for carry in [false, true] {
                if carry && bit_idx == 0 {
                    continue;
                }
                let carry_val = (carry as u64) << bit_idx.saturating_sub(1);
                let x = ((test_mode[0] as u64) << bit_idx) | carry_val;
                let y = ((test_mode[1] as u64) << bit_idx) | carry_val;
                let z_exp = x + y;
                for i in [0, 1] {
                    state[i * 64 + bit_idx] = test_mode[i];
                    if carry {
                        state[i * 64 + bit_idx - 1] = true;
                    }
                }
                if c.run(&mut state) != z_exp {
                    state
                        .iter()
                        .zip(candidate.iter_mut())
                        .skip(64 * 2)
                        .for_each(|(a, b)| *b |= a);
                }
                state.fill(false);
            }
        }
    }
    candidate
        .iter()
        .enumerate()
        .skip(64 * 2)
        .filter(|v| *v.1)
        .map(|v| v.0)
        .collect()
}

fn check_err_bits(c: &Circuit) -> u64 {
    let input_len = c.wires.lengths[0];
    let mut state = vec![false; c.wires.len()];
    let mut err_bits: u64 = 0;

    for bit_idx in 0..input_len {
        for test_mode in [[true, false], [false, true], [true, true]] {
            for carry in [false, true] {
                if carry && bit_idx == 0 {
                    continue;
                }
                let carry_val = (carry as u64) << bit_idx.saturating_sub(1);
                let x = ((test_mode[0] as u64) << bit_idx) | carry_val;
                let y = ((test_mode[1] as u64) << bit_idx) | carry_val;
                let z_exp = x + y;
                for i in [0, 1] {
                    state[i * 64 + bit_idx] = test_mode[i];
                    if carry {
                        state[i * 64 + bit_idx - 1] = true;
                    }
                }
                let z = c.run(&mut state);
                err_bits |= z ^ z_exp;
                state.fill(false);
            }
        }
    }
    err_bits
}

/// Returns wire set, initial state, and gate configurations
fn read_input<R: BufRead>(
    input: &mut R,
) -> Result<(WireSet, Vec<bool>, Vec<LogicGate>), anyhow::Error> {
    let mut wires = WireSet::new();
    let mut init_state = [false; 64 * 2];
    for line in input
        .lines()
        .map_while(Result::ok)
        .take_while(|v| !v.is_empty())
    {
        let parts = line
            .split_once(": ")
            .with_context(|| format!("invalid input: {:?}", line))?;
        let val = match parts.1 {
            "0" => false,
            "1" => true,
            v => return Err(anyhow!("invalid input {line:?}: invalid state:{v} ")),
        };
        let id = wires.register(parts.0);
        if id > 64 * 2 {
            return Err(anyhow!(
                "invalid initial state {line:?}: expected x or y, got offset {id}"
            ));
        }
        init_state[id] = val;
    }

    let mut gates = Vec::<LogicGate>::with_capacity(512);
    for line in input
        .lines()
        .map_while(Result::ok)
        .take_while(|v| !v.is_empty())
    {
        let parts: Vec<_> = line.split(' ').collect();
        if parts.len() != 5 || parts[3] != "->" {
            return Err(anyhow!("Invalid logic gate def: {line:?}"));
        }
        let op: LogicOp = parts[1]
            .try_into()
            .with_context(|| format!("invalid input: {line:?}"))?;
        gates.push(LogicGate {
            iw: (wires.register(parts[0]), wires.register(parts[2])),
            ow: wires.register(parts[4]),
            op,
        });
    }

    let init_state: Vec<_> = init_state
        .iter()
        .cloned()
        .chain(iter::repeat(false).take(wires.len() - init_state.len()))
        .collect();
    Ok((wires, init_state, gates))
}
