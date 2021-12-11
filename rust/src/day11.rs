use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Index = (usize, usize);

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn one_step(array: &mut Vec<Vec<usize>>, total: &mut usize) {
    let mut flashed: HashSet<Index> = HashSet::new();
    array
        .iter_mut()
        .for_each(|l| l.iter_mut().for_each(|x| *x += 1));
    let mut to_flash: Vec<Index> = array
        .iter()
        .enumerate()
        .flat_map(|a| {
            a.1.iter()
                .enumerate()
                .filter(|b| *b.1 > 9)
                .map(|b| (a.0, b.0))
                .collect::<Vec<Index>>()
        })
        .collect::<Vec<Index>>();
    to_flash.retain(|x| !flashed.contains(&x));

    while !to_flash.is_empty() {
        for (y, x) in &to_flash {
            flashed.insert((*y, *x));
            array[*y].get_mut(x + 1).iter_mut().for_each(|v| **v += 1);
            array
                .get_mut(*y)
                .and_then(|q| x.checked_sub(1).map(|z| q.get_mut(z)))
                .flatten()
                .iter_mut()
                .for_each(|v| **v += 1);
            y.checked_sub(1)
                .map(|z| array.get_mut(z).and_then(|v| v.get_mut(*x)))
                .flatten()
                .iter_mut()
                .for_each(|v| **v += 1);
            y.checked_sub(1)
                .map(|z| array.get_mut(z))
                .flatten()
                .map(|q| x.checked_sub(1).map(|z| q.get_mut(z)))
                .flatten()
                .flatten()
                .iter_mut()
                .for_each(|v| **v += 1);
            y.checked_sub(1)
                .map(|z| array.get_mut(z).and_then(|v| v.get_mut(*x + 1)))
                .flatten()
                .iter_mut()
                .for_each(|v| **v += 1);
            array
                .get_mut(*y + 1)
                .and_then(|q| x.checked_sub(1).map(|z| q.get_mut(z)))
                .flatten()
                .iter_mut()
                .for_each(|v| **v += 1);
            array
                .get_mut(*y + 1)
                .and_then(|q| q.get_mut(*x))
                .iter_mut()
                .for_each(|v| **v += 1);
            array
                .get_mut(*y + 1)
                .and_then(|q| q.get_mut(x + 1))
                .iter_mut()
                .for_each(|v| **v += 1);
        }
        to_flash = array
            .iter()
            .enumerate()
            .flat_map(|a| {
                a.1.iter()
                    .enumerate()
                    .filter(|b| *b.1 > 9)
                    .map(|b| (a.0, b.0))
                    .collect::<Vec<Index>>()
            })
            .collect::<Vec<Index>>();
        to_flash.retain(|x| !flashed.contains(&x));
    }

    for f in flashed.iter() {
        *total += 1;
        array[f.0][f.1] = 0;
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    let mut array: Vec<Vec<usize>> = input.clone();
    let mut total: usize = 0;

    for _i in 0..100 {
        one_step(&mut array, &mut total);
    }

    total
}
#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    let mut array: Vec<Vec<usize>> = input.clone();
    let mut total: usize = 0;
    let mut output: usize = 0;

    for i in 0..5000 {
        let old_total = total;
        one_step(&mut array, &mut total);
        if total - old_total == array.len() * array[0].len() {
            output = i + 1;
            break;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 1656);
    }
    #[test]
    fn test_one_step() {
        let inp = "11111
19991
19191
19991
11111";
        let mut parsed = input_generator(inp);
        let mut total: usize = 0;
        one_step(&mut parsed, &mut total);
        assert_eq!(total, 9);
    }
    #[test]
    fn test_2() {
        let inp = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 195);
    }
}
