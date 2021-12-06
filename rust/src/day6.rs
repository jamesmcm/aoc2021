use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[i32]) -> u32 {
    let mut counts: HashMap<i32, u32> = HashMap::new();
    for i in input {
        counts.entry(*i).and_modify(|x| *x += 1).or_insert(1);
    }

    for _i in 1..=80 {
        let mut newcounts: HashMap<i32, u32> = HashMap::new();
        for (k, v) in counts.iter() {
            if *k > 0 {
                newcounts
                    .entry(k - 1)
                    .and_modify(|x| *x += *v)
                    .or_insert(*v);
            } else {
                newcounts.insert(8, *v);
                newcounts.entry(6).and_modify(|x| *x += *v).or_insert(*v);
            }
        }

        counts = newcounts;
    }

    counts.values().map(|x| *x).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[i32]) -> u64 {
    let mut counts: HashMap<i32, u64> = HashMap::new();
    for i in input {
        counts.entry(*i).and_modify(|x| *x += 1).or_insert(1);
    }

    for _i in 1..=256 {
        let mut newcounts: HashMap<i32, u64> = HashMap::new();
        for (k, v) in counts.iter() {
            if *k > 0 {
                newcounts
                    .entry(k - 1)
                    .and_modify(|x| *x += *v)
                    .or_insert(*v);
            } else {
                newcounts.insert(8, *v);
                newcounts.entry(6).and_modify(|x| *x += *v).or_insert(*v);
            }
        }

        counts = newcounts;
    }

    counts.values().map(|x| *x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "3,4,3,1,2";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 5934);
    }
    #[test]
    fn test_2() {
        let inp = "3,4,3,1,2";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 26984457539);
    }
}
