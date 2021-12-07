use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut sorted = input.iter().map(|x| *x).collect::<Vec<i32>>();
    sorted.sort();
    // dbg!(&sorted);
    let median = sorted[sorted.len() / 2];
    sorted.iter().map(|x| (x - median).abs()).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut sorted = input.iter().map(|x| *x).collect::<Vec<i32>>();
    sorted.sort();
    let mut min_fuel = i32::MAX;
    for i in sorted[0]..sorted[sorted.len() - 1] {
        let fuel = calc_fuel(&sorted, i);
        if fuel < min_fuel {
            min_fuel = fuel;
            // dbg!(i);
            // dbg!(fuel);
        }
    }
    min_fuel
}

pub fn calc_fuel(input: &[i32], target: i32) -> i32 {
    input
        .iter()
        .map(|x| (1..=(x - target).abs()).sum::<i32>())
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "16,1,2,0,4,2,7,1,2,14";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 37);
    }
    #[test]
    fn test_2() {
        let inp = "16,1,2,0,4,2,7,1,2,14";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 168);
    }
}
