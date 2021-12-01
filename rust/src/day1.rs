use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input
        .windows(2)
        .fold(0, |x, y| if y[1] > *y.first().unwrap() { x + 1 } else { x })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |x, y| if y[1] > *y.first().unwrap() { x + 1 } else { x })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve_part1(&inp), 7);
    }
    #[test]
    fn test_2() {
        let inp = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve_part2(&inp), 5);
    }
}
