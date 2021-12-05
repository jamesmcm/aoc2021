use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Position = (i32, i32);
pub struct Line {
    pub start: Position,
    pub end: Position,
}

impl Line {
    pub fn get_points(&self) -> Vec<Position> {
        if self.start.0 == self.end.0 {
            // Vertical line from start to end
            (self.start.1..=self.end.1)
                .map(|y| (self.start.0, y))
                .collect()
        } else if self.start.1 == self.end.1 {
            // Horizontal line from start to end
            (self.start.0..=self.end.0)
                .map(|x| (x, self.start.1))
                .collect()
        } else {
            // Diagonal
            let grad = (self.end.1 - self.start.1) / (self.end.0 - self.start.0);

            (self.start.0..=self.end.0)
                .enumerate()
                .map(|x| (x.1, self.start.1 + (x.0 as i32 * grad)))
                .collect()
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    let mut out = vec![];
    for line in input.lines() {
        let mut it = line.split_whitespace();
        let first = it.next().unwrap();
        it.next();
        let second = it.next().unwrap();
        let first: Vec<i32> = first.split(',').map(|x| x.parse().unwrap()).collect();
        let second: Vec<i32> = second.split(',').map(|x| x.parse().unwrap()).collect();

        let l = match ((first[0], first[1]), (second[0], second[1])) {
            (f, s) if f.0 == s.0 => {
                if f.1 <= s.1 {
                    (f, s)
                } else {
                    (s, f)
                }
            }
            (f, s) if f.0 < s.0 => (f, s),
            (f, s) if f.0 > s.0 => (s, f),
            (f, s) => (f, s),
        };
        out.push(Line {
            start: (l.0 .0, l.0 .1),
            end: (l.1 .0, l.1 .1),
        });
    }
    out
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Line]) -> i32 {
    let mut world: HashMap<(i32, i32), i32> = HashMap::new();

    for l in input
        .iter()
        .filter(|x| x.start.0 == x.end.0 || x.start.1 == x.end.1)
    {
        for p in l.get_points() {
            world.entry(p).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    world.values().filter(|&x| *x >= 2).count() as i32
}
#[aoc(day5, part2)]
pub fn solve_part2(input: &[Line]) -> i32 {
    let mut world: HashMap<(i32, i32), i32> = HashMap::new();

    for l in input {
        for p in l.get_points() {
            world.entry(p).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    world.values().filter(|&x| *x >= 2).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 5);
    }
    #[test]
    fn test_2() {
        let inp = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 12);
    }
}
