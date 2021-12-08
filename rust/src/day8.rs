use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Problem {
    pub configs: Vec<HashSet<char>>,
    pub output: Vec<HashSet<char>>,
}

lazy_static! {
    static ref STANDARDS: HashMap<u32, HashSet<char>> = {
        let mut m = HashMap::new();
        [
            (0, vec!['a', 'b', 'c', 'e', 'f', 'g']),
            (1, vec!['c', 'f']),
            (2, vec!['a', 'c', 'd', 'e', 'g']),
            (3, vec!['a', 'c', 'd', 'f', 'g']),
            (4, vec!['b', 'c', 'd', 'f']),
            (5, vec!['a', 'b', 'd', 'f', 'g']),
            (6, vec!['a', 'b', 'd', 'e', 'f', 'g']),
            (7, vec!['a', 'c', 'f']),
            (8, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            (9, vec!['a', 'b', 'c', 'd', 'f', 'g']),
        ]
        .into_iter()
        .for_each(|l| {
            let mut s = HashSet::new();
            l.1.into_iter().for_each(|x| {
                s.insert(x);
            });
            m.insert(l.0, s);
        });
        m
    };
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Problem> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split('|');
            let configs = parts
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| {
                    let mut hs = HashSet::new();
                    s.chars().for_each(|c| {
                        hs.insert(c);
                    });
                    hs
                })
                .collect();
            let output = parts
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| {
                    let mut hs = HashSet::new();
                    s.chars().for_each(|c| {
                        hs.insert(c);
                    });
                    hs
                })
                .collect();
            Problem { configs, output }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Problem]) -> usize {
    // dbg!(&input);
    input
        .iter()
        .map(|x| {
            x.output
                .iter()
                .filter(|hs| hs.len() == 2 || hs.len() == 4 || hs.len() == 3 || hs.len() == 7)
                .count()
        })
        .sum()
}

pub fn solve_one_output(input: &Problem) -> usize {
    let mut sols: HashMap<char, Vec<char>> = HashMap::new();
    ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .into_iter()
        .for_each(|c| {
            sols.insert(c, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        });

    let mut solved: HashMap<char, char> = HashMap::new();

    // 1 - 2 segments
    let segs = input.configs.iter().find(|i| i.len() == 2).unwrap();
    ['c', 'f'].iter().for_each(|c| {
        sols.get_mut(c).unwrap().retain(|x| segs.contains(&x));
    });

    // 4 - 4 segments
    let segs = input.configs.iter().find(|i| i.len() == 4).unwrap();
    ['b', 'c', 'd', 'f'].iter().for_each(|c| {
        sols.get_mut(c).unwrap().retain(|x| segs.contains(&x));
    });

    // 7 - 3 segments
    let segs = input.configs.iter().find(|i| i.len() == 3).unwrap();
    ['a', 'c', 'f'].iter().for_each(|c| {
        sols.get_mut(c).unwrap().retain(|x| segs.contains(&x));
    });

    // 8 - 7 segments - tells us nothing?
    let segs = input.configs.iter().find(|i| i.len() == 7).unwrap();
    ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().for_each(|c| {
        sols.get_mut(c).unwrap().retain(|x| segs.contains(&x));
    });

    // At this point C and F are indistinguishable
    // 0, 6, 9 have 6 segments each
    // But 6 will be missing C
    let segs = input
        .configs
        .iter()
        .find(|c| c.len() == 6 && !(c.contains(&sols[&'c'][0]) && c.contains(&sols[&'c'][1])))
        .unwrap();
    ['a', 'b', 'd', 'e', 'f', 'g'].iter().for_each(|c| {
        sols.get_mut(c).unwrap().retain(|x| segs.contains(&x));
        sols.get_mut(&'c').unwrap().retain(|x| !segs.contains(&x));
    });

    // C + F solved so remove
    solved.insert('c', sols[&'c'][0]);
    solved.insert('f', sols[&'f'][0]);
    for (_k, v) in sols.iter_mut() {
        if v.len() > 1 {
            v.retain(|x| *x != solved[&'c'] && *x != solved[&'f']);
        }
    }
    // A solved
    solved.insert('a', sols[&'a'][0]);
    for (_k, v) in sols.iter_mut() {
        if v.len() > 1 {
            v.retain(|x| *x != solved[&'a']);
        }
    }
    // At this point B and D are indistinguishable
    // 0, 6, 9 have 6 segments each
    // But 0 will be missing D
    let segs = input
        .configs
        .iter()
        .find(|c| c.len() == 6 && !(c.contains(&sols[&'d'][0]) && c.contains(&sols[&'d'][1])))
        .unwrap();
    ['a', 'b', 'c', 'e', 'f', 'g'].iter().for_each(|c| {
        sols.get_mut(c).unwrap().retain(|x| segs.contains(&x));
        sols.get_mut(&'d').unwrap().retain(|x| !segs.contains(&x));
    });
    // D solved
    solved.insert('d', sols[&'d'][0]);
    for (_k, v) in sols.iter_mut() {
        if v.len() > 1 {
            v.retain(|x| *x != solved[&'d']);
        }
    }
    // B solved
    solved.insert('b', sols[&'b'][0]);
    for (_k, v) in sols.iter_mut() {
        if v.len() > 1 {
            v.retain(|x| *x != solved[&'b']);
        }
    }
    // At this point E and G are indistinguishable
    // 0, 6, 9 have 6 segments each
    // But 9 will be missing E
    let segs = input
        .configs
        .iter()
        .find(|c| c.len() == 6 && !(c.contains(&sols[&'e'][0]) && c.contains(&sols[&'e'][1])))
        .unwrap();
    ['a', 'b', 'c', 'd', 'f', 'g'].iter().for_each(|c| {
        sols.get_mut(c).unwrap().retain(|x| segs.contains(&x));
        sols.get_mut(&'e').unwrap().retain(|x| !segs.contains(&x));
    });
    // E solved
    solved.insert('e', sols[&'e'][0]);
    for (_k, v) in sols.iter_mut() {
        if v.len() > 1 {
            v.retain(|x| *x != solved[&'e']);
        }
    }
    // G solved
    solved.insert('g', sols[&'g'][0]);
    for (_k, v) in sols.iter_mut() {
        if v.len() > 1 {
            v.retain(|x| *x != solved[&'g']);
        }
    }

    let mut solved_reverse: HashMap<char, char> = HashMap::new();
    for (k, v) in &solved {
        solved_reverse.insert(*v, *k);
    }

    let mut converted: HashMap<u32, HashSet<char>> = HashMap::new();
    for (k, v) in STANDARDS.iter() {
        let mut s: HashSet<char> = HashSet::new();
        v.iter().for_each(|x| {
            s.insert(*(solved.get(&x).unwrap()));
        });
        converted.insert(*k, s);
    }

    // dbg!(&solved_reverse);
    // dbg!(&converted);
    input
        .output
        .iter()
        .map(|x| converted.iter().find(|k| k.1 == x).unwrap().0.to_string())
        .collect::<Vec<String>>()
        .concat()
        .parse::<usize>()
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Problem]) -> usize {
    input.iter().map(|x| solve_one_output(x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 26);
    }
    #[test]
    fn test_2() {
        let inp =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 61229);
    }
    #[test]
    fn test_one_line() {
        let inp =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let parsed = input_generator(inp);
        assert_eq!(solve_one_output(&parsed[0]), 5353);
    }
}
