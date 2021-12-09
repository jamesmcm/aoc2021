use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Index = (usize, usize);

#[aoc_generator(day9)]
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

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    let mut output = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if is_pit(input, (y, x)) {
                // dbg!(input[y][x]);
                // dbg!((y, x));
                output += input[y][x] + 1;
            }
        }
    }

    output
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    let mut output = 1;
    let mut pits: HashMap<Index, Vec<Index>> = HashMap::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if is_pit(input, (y, x)) {
                pits.insert((y, x), vec![]);
            }
        }
    }

    // dbg!(&pits);
    let keys: Vec<Index> = pits.keys().map(|k| *k).collect();
    for pit in keys {
        recurse_dfs(&mut pits, pit, input, pit);
    }

    let mut out: Vec<(Index, usize)> = pits.iter().map(|x| (*x.0, x.1.len())).collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    out.iter().take(3).for_each(|x| output *= x.1);
    output
}

pub fn recurse_dfs(
    pits: &mut HashMap<Index, Vec<Index>>,
    pit: Index,
    array: &Vec<Vec<usize>>,
    i: Index,
) {
    let val = array.get(i.0).and_then(|v| v.get(i.1));
    if pits.get(&pit).unwrap().contains(&i) {
        return;
    }
    if val == Some(&9) || val.is_none() {
        return;
    }
    pits.get_mut(&pit).unwrap().push(i);
    [
        i.1.checked_sub(1).map(|z| (i.0, z)),
        Some((i.0, i.1 + 1)),
        i.0.checked_sub(1).map(|z| (z, i.1)),
        Some((i.0 + 1, i.1)),
    ]
    .iter()
    .filter(|x| x.is_some())
    .for_each(|x| recurse_dfs(pits, pit, array, x.unwrap()));
}

pub fn is_pit(array: &Vec<Vec<usize>>, i: Index) -> bool {
    [
        array
            .get(i.0)
            .and_then(|x| i.1.checked_sub(1).map(|z| x.get(z)))
            .flatten(),
        array.get(i.0).and_then(|x| x.get(i.1 + 1)),
        i.0.checked_sub(1)
            .map(|z| array.get(z).and_then(|x| x.get(i.1)))
            .flatten(),
        array.get(i.0 + 1).and_then(|x| x.get(i.1)),
    ]
    .iter()
    .filter(|x| x.is_some())
    .all(|x| *x.unwrap() > array[i.0][i.1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 15);
    }
    #[test]
    fn test_2() {
        let inp = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 1134);
    }
}
