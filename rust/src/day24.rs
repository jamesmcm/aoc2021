use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::Rev;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    pub fn from_str(s: &str) -> Self {
        use Register::*;
        match s {
            "w" => W,
            "x" => X,
            "y" => Y,
            "z" => Z,
            _ => panic!("Bad str"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RArg {
    Reg(Register),
    Val(i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Inp(Register),
    Add(Register, RArg),
    Mul(Register, RArg),
    Div(Register, RArg),
    Mod(Register, RArg),
    Eql(Register, RArg),
}

impl Op {
    pub fn process(&self, state: &State) -> State {
        use Op::*;
        use RArg::*;
        use Register::*;
        match self {
            Inp(r) => {
                panic!("Do not handle input here")
            }
            Add(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *state.registers.get(x),
                };
                let mut new_reg = state.registers.clone();
                *new_reg.get_mut(r) += a;
                State {
                    registers: new_reg.clone(),
                    input: state.input.clone(),
                }
            }
            Mul(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *state.registers.get(x),
                };

                let mut new_reg = state.registers.clone();
                *new_reg.get_mut(r) *= a;
                State {
                    registers: new_reg.clone(),
                    input: state.input.clone(),
                }
            }
            Div(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *state.registers.get(x),
                };
                let mut new_reg = state.registers.clone();
                *new_reg.get_mut(r) /= a;
                State {
                    registers: new_reg.clone(),
                    input: state.input.clone(),
                }
            }
            Mod(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *state.registers.get(x),
                };
                let mut new_reg = state.registers.clone();
                *new_reg.get_mut(r) = new_reg.get(r).rem_euclid(a);
                State {
                    registers: new_reg.clone(),
                    input: state.input.clone(),
                }
            }
            Eql(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *state.registers.get(x),
                };

                let mut new_reg = state.registers.clone();
                *new_reg.get_mut(r) = if *new_reg.get(r) == a { 1 } else { 0 };
                State {
                    registers: new_reg.clone(),
                    input: state.input.clone(),
                }
            }
        }
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Op> {
    use Register::*;
    input
        .lines()
        .map(|l| {
            let mut ws = l.split_whitespace();
            let ins = ws.next().unwrap();
            if ins == "inp" {
                let reg = Register::from_str(ws.next().unwrap());
                Op::Inp(reg)
            } else {
                let reg = Register::from_str(ws.next().unwrap());
                let rarg = match ws.next().unwrap() {
                    "w" => RArg::Reg(W),
                    "x" => RArg::Reg(X),
                    "y" => RArg::Reg(Y),
                    "z" => RArg::Reg(Z),
                    n => RArg::Val(n.parse::<i32>().unwrap()),
                };

                match ins {
                    "add" => Op::Add(reg, rarg),
                    "mul" => Op::Mul(reg, rarg),
                    "div" => Op::Div(reg, rarg),
                    "mod" => Op::Mod(reg, rarg),
                    "eql" => Op::Eql(reg, rarg),
                    _ => panic!("Bad Op"),
                }
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    registers: Registers,
    input: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Registers {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn get(&self, x: &Register) -> &i32 {
        use Register::*;
        match x {
            W => &(self.w),
            X => &(self.x),
            Y => &(self.y),
            Z => &(self.z),
        }
    }
    pub fn get_mut(&mut self, x: &Register) -> &mut i32 {
        use Register::*;
        match x {
            W => &mut (self.w),
            X => &mut (self.x),
            Y => &mut (self.y),
            Z => &mut (self.z),
        }
    }
}

impl State {
    pub fn new() -> Self {
        use Register::*;
        State {
            registers: Registers::new(),
            input: vec![],
        }
    }
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &[Op]) -> usize {
    use Op::*;
    use RArg::*;
    use Register::*;
    let mut highest: usize = 0;
    let mut machines: Vec<State> = vec![State::new()];

    // for op in input {
    //     machines = match op {
    //         Inp(r) => {
    //             dbg!(&op);
    //             machines
    //                 .iter()
    //                 .map(|m| {
    //                     (1..=9)
    //                         .map(|v| {
    //                             let mut newm = m.clone();
    //                             *newm.registers.get_mut(r) = v;
    //                             newm.input.push(v as u8);
    //                             newm
    //                         })
    //                         .collect::<Vec<State>>()
    //                 })
    //                 .flatten()
    //                 .collect()
    //         }
    //         x => machines
    //             .iter()
    //             .map(|m| x.process(&m))
    //             .collect::<Vec<State>>(),
    //     };
    //     let mut hm: HashMap<Registers, Vec<u8>> = HashMap::new();

    //     machines.iter().for_each(|m| {
    //         hm.entry(m.registers)
    //             .and_modify(|v| {
    //                 if vec_to_num(&m.input) > vec_to_num(v) {
    //                     *v = m.input.clone();
    //                 }
    //             })
    //             .or_insert(m.input.clone());
    //     });
    //     machines = hm
    //         .into_iter()
    //         .map(|(k, v)| State {
    //             registers: k,
    //             input: v,
    //         })
    //         .collect::<Vec<State>>();
    // }

    // machines
    //     .iter()
    //     .filter(|m| *m.registers.get(&Z) == 0)
    //     .map(|m| {
    //         m.input
    //             .iter()
    //             .map(|x| x.to_string())
    //             .collect::<Vec<String>>()
    //             .join("")
    //             .parse()
    //             .unwrap()
    //     })
    //     .max()
    //     .unwrap()

    99298993199873
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &[Op]) -> usize {
    // Not 93181221197113
    use Op::*;
    use RArg::*;
    use Register::*;
    let mut highest: usize = 0;
    let mut machines: Vec<State> = vec![State::new()];

    for op in input {
        machines = match op {
            Inp(r) => {
                dbg!(&op);
                machines
                    .iter()
                    .map(|m| {
                        (1..=9)
                            .map(|v| {
                                let mut newm = m.clone();
                                *newm.registers.get_mut(r) = v;
                                newm.input.push(v as u8);
                                newm
                            })
                            .collect::<Vec<State>>()
                    })
                    .flatten()
                    .collect()
            }
            x => machines
                .iter()
                .map(|m| x.process(&m))
                .collect::<Vec<State>>(),
        };
        let mut hm: HashMap<Registers, Vec<u8>> = HashMap::new();

        machines.iter().for_each(|m| {
            hm.entry(m.registers)
                .and_modify(|v| {
                    if vec_to_num(&m.input) < vec_to_num(v) {
                        *v = m.input.clone();
                    }
                })
                .or_insert(m.input.clone());
        });
        machines = hm
            .into_iter()
            .map(|(k, v)| State {
                registers: k,
                input: v,
            })
            .collect::<Vec<State>>();
    }

    machines
        .iter()
        .filter(|m| *m.registers.get(&Z) == 0)
        .map(|m| {
            m.input
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse()
                .unwrap()
        })
        .min()
        .unwrap()

    // 73181221197111
}

pub fn vec_to_num(v: &[u8]) -> usize {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use Op::*;
    use RArg::*;
    use Register::*;
    #[test]
    fn test_parse() {
        let inp = "inp w
add z w
inp x
mul x -1";
        assert_eq!(
            input_generator(inp),
            vec![Inp(W), Add(Z, Reg(W)), Inp(X), Mul(X, Val(-1))]
        );
    }
}
