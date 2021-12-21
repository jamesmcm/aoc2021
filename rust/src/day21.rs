use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::ops::RangeInclusive;
#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (u8, u8) {
    let re1 = Regex::new("Player 1 starting position: ([0-9]+)").unwrap();
    let re2 = Regex::new("Player 2 starting position: ([0-9]+)").unwrap();

    let mut it = input.lines();
    let l1 = it.next().unwrap();
    let l2 = it.next().unwrap();
    (
        re1.captures(l1)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        re2.captures(l2)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
    )
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &(u8, u8)) -> usize {
    let mut p1score: usize = 0;
    let mut p2score: usize = 0;
    let mut num_rolls = 0;
    let mut i = 0;

    let mut s1 = input.0;
    let mut s2 = input.1;
    let out = loop {
        // (i+1, i+2, i+3) = 3i+6
        //
        let mut place = if i % 2 == 0 {
            let p = (s1 as usize + (9 * i + 6)) % 10;
            s1 = p as u8;
            p
        } else {
            let p = (s2 as usize + (9 * i + 6)) % 10;
            s2 = p as u8;
            p
        };
        num_rolls += 3;
        if place == 0 {
            place = 10;
        }
        if i % 2 == 0 {
            p1score += place as usize;
            if p1score >= 1000 {
                // println!(
                //     "p1 wins score: {}, p2 score: {}, num_rolls: {}",
                //     p1score, p2score, num_rolls
                // );
                break p2score * num_rolls;
            }
        } else {
            // println!("i: {}, before p2score: {}, adding: {}", i, p2score, place);
            p2score += place as usize;
            if p2score >= 1000 {
                // println!(
                //     "p2 wins score: {}, p1 score: {}, num_rolls: {}",
                //     p2score, p1score, num_rolls
                // );
                break p1score * num_rolls;
            }
        }

        i += 1;
    };

    out
}

pub struct State {
    pub p1score: u8,
    pub p2score: u8,
    pub i: u8,
    pub s1: u8,
    pub s2: u8,
    pub p1multi: usize,
    pub p2multi: usize,
}

pub fn multiplier(x: u8) -> usize {
    match x {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => panic!("Bad number"),
    }
}

pub fn step(state: State) -> (Vec<State>, usize, usize) {
    let mut pending: Vec<State> = Vec::new();
    let mut p1wins: usize = 0;
    let mut p2wins: usize = 0;
    let range: RangeInclusive<u8> = (3..=9);
    range
        .map(|r| {
            if state.i % 2 == 0 {
                let s1 = (state.s1 + r) % 10;
                let p1score = state.p1score + if (s1 == 0) { 10 } else { s1 };
                let p1multi = state.p1multi * multiplier(r);
                let p2multi = state.p2multi * multiplier(r);
                if p1score >= 21 {
                    (None, p1multi, 0)
                } else {
                    (
                        Some(State {
                            p1score,
                            p2score: state.p2score,
                            i: state.i + 1,
                            s1,
                            s2: state.s2,
                            p1multi,
                            p2multi,
                        }),
                        0,
                        0,
                    )
                }
            } else {
                let s2 = (state.s2 + r) % 10;
                let p2score = state.p2score + if (s2 == 0) { 10 } else { s2 };
                let p1multi = state.p1multi * multiplier(r);
                let p2multi = state.p2multi * multiplier(r);
                if p2score >= 21 {
                    (None, 0, p2multi)
                } else {
                    (
                        Some(State {
                            p1score: state.p1score,
                            p2score,
                            i: state.i + 1,
                            s1: state.s1,
                            s2,
                            p1multi,
                            p2multi,
                        }),
                        0,
                        0,
                    )
                }
            }
        })
        .for_each(|x| match x {
            (Some(s), _, _) => {
                pending.push(s);
            }
            (None, x, y) => {
                p1wins += x;
                p2wins += y;
            }
        });

    (pending, p1wins, p2wins)
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &(u8, u8)) -> usize {
    let mut p1wins;
    let mut p2wins;

    let mut pending;
    let mut i = 0;

    let res = step(State {
        p1score: 0,
        p2score: 0,
        i: 0,
        s1: input.0,
        s2: input.1,
        p1multi: 1,
        p2multi: 1,
    });
    pending = res.0;
    p1wins = res.1;
    p2wins = res.2;

    while !pending.is_empty() {
        let state = pending.pop().unwrap();
        let mut res = step(state);
        p1wins += res.1;
        p2wins += res.2;
        pending.append(&mut res.0);
    }
    if p1wins >= p2wins {
        p1wins
    } else {
        p2wins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let inp = "Player 1 starting position: 4
Player 2 starting position: 8";
        assert_eq!(input_generator(inp), (4, 8));
    }
    #[test]
    fn test_part1() {
        let inp = "Player 1 starting position: 4
Player 2 starting position: 8";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 739785);
    }
    #[test]
    fn test_part2() {
        let inp = "Player 1 starting position: 4
Player 2 starting position: 8";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 444356092776315);
    }
}
