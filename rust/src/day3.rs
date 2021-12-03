use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<u32>, u32) {
    (
        input
            .lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect(),
        input.lines().next().unwrap().len() as u32,
    )
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &(Vec<u32>, u32)) -> u32 {
    let bits = input.1;
    let input = &input.0;
    let l = input.len() as u32;
    let boolpart = (0..bits)
        .rev()
        .map(|x: u32| {
            let num_ones = input.iter().map(|i| (*i >> x) & 1).sum::<u32>();
            let num_zeros = l - num_ones;
            let most_common = (num_ones >= num_zeros) as u32;
            most_common.to_string()
        })
        .collect::<Vec<String>>();
    let boolpart: String = boolpart.concat();

    let gamma = u32::from_str_radix(&boolpart, 2).unwrap();
    let epsilon = (!gamma) & (u32::pow(2, bits) - 1);
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &(Vec<u32>, u32)) -> u32 {
    let bits = input.1;
    let input = &input.0;

    // oxygen generator
    let oxy: u32 = (0..bits).rev().fold(input.clone(), |acc, b| {
        let newl = acc.len() as u32;
        if newl > 1 {
            let num_ones = acc.iter().map(|i| (*i >> b) & 1).sum::<u32>();
            let num_zeros = newl - num_ones;
            let most_common = (num_ones >= num_zeros) as u32;
            acc.iter()
                .filter(|i| ((*i >> b) & 1) == most_common)
                .map(|x| *x)
                .collect()
        } else {
            acc
        }
    })[0];

    // co2
    let co: u32 = (0..bits).rev().fold(input.clone(), |acc, b| {
        let newl = acc.len() as u32;
        if newl > 1 {
            let num_ones = acc.iter().map(|i| (*i >> b) & 1).sum::<u32>();
            let num_zeros = newl - num_ones;
            let least_common = !(num_ones >= num_zeros) as u32;
            acc.iter()
                .filter(|i| (*i >> b) & 1 == least_common)
                .map(|x| *x)
                .collect()
        } else {
            acc
        }
    })[0];

    oxy * co
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 198);
    }
    #[test]
    fn test_2() {
        let inp = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 230);
    }
}
