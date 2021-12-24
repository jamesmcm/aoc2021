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
    Val(i64),
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
    pub fn process<'a, 'b, I>(&self, registers: &mut HashMap<Register, i64>, input: &'a mut I)
    where
        I: Iterator<Item = &'b i64>,
    {
        use Op::*;
        use RArg::*;
        use Register::*;
        match self {
            Inp(r) => {
                *registers.get_mut(r).unwrap() = *(input.next().unwrap());
            }
            Add(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *registers.get(x).unwrap(),
                };
                registers.entry(*r).and_modify(|v| *v += a);
            }
            Mul(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *registers.get(x).unwrap(),
                };

                registers.entry(*r).and_modify(|v| *v *= a);
            }
            Div(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *registers.get(x).unwrap(),
                };
                registers.entry(*r).and_modify(|v| *v /= a);
            }
            Mod(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *registers.get(x).unwrap(),
                };
                registers.entry(*r).and_modify(|v| *v = v.rem_euclid(a));
            }
            Eql(r, a) => {
                let a = match a {
                    Val(x) => *x,
                    Reg(x) => *registers.get(x).unwrap(),
                };

                registers
                    .entry(*r)
                    .and_modify(|v| *v = if *v == a { 1 } else { 0 });
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
                    n => RArg::Val(n.parse::<i64>().unwrap()),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    registers: HashMap<Register, i64>,
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &[Op]) -> usize {
    use Register::*;
    let mut registers: HashMap<Register, i64> = HashMap::new();
    registers.insert(W, 0);
    registers.insert(X, 0);
    registers.insert(Y, 0);
    registers.insert(Z, 0);
    let mut highest: usize = 0;

    let range: Rev<RangeInclusive<i64>> = (11111111111111..=99999999999999).rev();

    for inp in range
        .map(|x| {
            x.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .filter(|v| !v.contains(&0))
    {
        registers.insert(W, 0);
        registers.insert(X, 0);
        registers.insert(Y, 0);
        registers.insert(Z, 0);

        let mut inp_it = inp.iter();
        for op in input {
            op.process(&mut registers, &mut inp_it);
        }
        if *registers.get(&Z).unwrap() == 0 {
            let mut n = inp.iter().map(|x| x.to_string()).collect::<Vec<String>>();
            highest = n.join("").parse().unwrap();
            break;
        }
    }

    highest
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
