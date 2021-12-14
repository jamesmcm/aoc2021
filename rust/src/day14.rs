use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = input.lines();
    let mut map = HashMap::new();
    let f = lines.next().unwrap();
    let start: Vec<char> = f.trim().chars().collect();
    lines.next();
    for l in lines {
        let mut s = l.split(" -> ").collect::<Vec<&str>>();
        let f = s[0].chars().collect::<Vec<char>>();
        map.insert((f[0], f[1]), s[1].chars().next().unwrap());
    }
    (start, map)
}

pub fn count_chars(input: &Vec<char>) -> Vec<(char, usize)> {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in input {
        map.entry(*c).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut out: Vec<(char, usize)> = Vec::new();
    for (k, v) in map.iter() {
        out.push((*k, *v));
    }
    out.sort_by(|x, y| x.1.cmp(&y.1));
    out
}

pub fn count_chars2(input: &HashMap<(char, char), usize>, last_char: char) -> Vec<(char, usize)> {
    let mut map: HashMap<char, usize> = HashMap::new();

    for (k, v) in input.iter() {
        map.entry(k.0).and_modify(|x| *x += v).or_insert(*v);
        // map.entry(k.1).and_modify(|x| *x += v).or_insert(*v);
    }
    *map.get_mut(&last_char).unwrap() += 1;
    let mut out: Vec<(char, usize)> = Vec::new();
    for (k, v) in map.iter() {
        out.push((*k, *v));
    }
    out.sort_by(|x, y| x.1.cmp(&y.1));
    out
}

pub fn count_pairs(input: &Vec<char>) -> HashMap<(char, char), usize> {
    let mut map: HashMap<(char, char), usize> = HashMap::new();

    for c in input.windows(2) {
        map.entry((c[0], c[1])).and_modify(|x| *x += 1).or_insert(1);
    }
    map
}

pub fn process_pairs(
    pairs: HashMap<(char, char), usize>,
    map: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut new_map: HashMap<(char, char), usize> = HashMap::new();
    for (k, v) in pairs.iter() {
        if let Some(c) = map.get(k) {
            new_map
                .entry((k.0, *c))
                .and_modify(|x| *x += v)
                .or_insert(*v);
            new_map
                .entry((*c, k.1))
                .and_modify(|x| *x += v)
                .or_insert(*v);
        } else {
            new_map
                .entry((k.0, k.1))
                .and_modify(|x| *x += v)
                .or_insert(*v);
        }
    }
    new_map
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    let mut src = input.0.clone();

    for _i in 0..10 {
        let mut out: Vec<char> = Vec::new();
        for s in src.windows(2) {
            out.push(s[0]);
            input.1.get(&(s[0], s[1])).iter().for_each(|x| {
                out.push(**x);
            });
        }
        out.push(src[src.len() - 1]);
        src = out;
        dbg!(&src);
    }
    let counts = count_chars(&src);
    dbg!(&counts);
    counts[counts.len() - 1].1 - counts[0].1
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    let mut pairs = count_pairs(&input.0);

    for _i in 0..40 {
        pairs = process_pairs(pairs, &input.1);
    }
    let counts = count_chars2(&pairs, input.0[input.0.len() - 1]);
    dbg!(&counts);
    counts[counts.len() - 1].1 - counts[0].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 1588);
    }
    #[test]
    fn test_2() {
        let inp = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 2188189693529);
    }
}
