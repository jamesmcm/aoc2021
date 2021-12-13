use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;

type Position = (usize, usize);
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Fold {
    axis: Axis,
    value: usize,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (HashSet<Position>, Vec<Fold>) {
    let mut split = input.split("\n\n");
    let mut pos_set: HashSet<Position> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    for line in split.next().unwrap().lines() {
        let p = line
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();
        pos_set.insert((p[0], p[1]));
    }
    for line in split.next().unwrap().lines() {
        let mut splits = line.split_whitespace().nth(2).unwrap().split("=");
        let axis = match splits.next().unwrap() {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("Invalid char"),
        };
        folds.push(Fold {
            axis,
            value: splits.next().unwrap().parse().unwrap(),
        });
    }
    (pos_set, folds)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(HashSet<Position>, Vec<Fold>)) -> usize {
    let fold = input.1[0];

    let mut new_set: HashSet<Position> = HashSet::new();
    input.0.iter().for_each(|x| match fold.axis {
        Axis::X => {
            if x.0 > fold.value {
                new_set.insert((fold.value - (x.0 - fold.value), x.1));
            } else if x.0 < fold.value {
                new_set.insert((x.0, x.1));
            }
        }
        Axis::Y => {
            if x.1 > fold.value {
                new_set.insert((x.0, fold.value - (x.1 - fold.value)));
            } else if x.1 < fold.value {
                new_set.insert((x.0, x.1));
            }
        }
    });
    new_set.len()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(HashSet<Position>, Vec<Fold>)) -> String {
    let mut old_set = input.0.clone();
    for fold in &input.1 {
        let mut new_set: HashSet<Position> = HashSet::new();
        old_set.iter().for_each(|x| match fold.axis {
            Axis::X => {
                if x.0 > fold.value {
                    new_set.insert((fold.value - (x.0 - fold.value), x.1));
                } else if x.0 < fold.value {
                    new_set.insert((x.0, x.1));
                }
            }
            Axis::Y => {
                if x.1 > fold.value {
                    new_set.insert((x.0, fold.value - (x.1 - fold.value)));
                } else if x.1 < fold.value {
                    new_set.insert((x.0, x.1));
                }
            }
        });
        old_set = new_set;
    }

    let mut max_x = 0;
    let mut max_y = 0;
    for p in old_set.iter() {
        if p.0 > max_x {
            max_x = p.0
        }
        if p.1 > max_y {
            max_y = p.1
        }
    }

    let mut v = Vec::new();
    for _i in 0..=max_y {
        v.push(vec!['.'; max_x + 1]);
    }
    for p in old_set.iter() {
        v[p.1][p.0] = '#'
    }

    let out = v
        .iter()
        .map(|l| String::from_iter(l))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 17);
    }
    #[test]
    fn test_2() {
        let inp = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let parsed = input_generator(inp);
        assert_eq!(
            solve_part2(&parsed),
            "#####
#...#
#...#
#...#
#####"
        );
    }
}
