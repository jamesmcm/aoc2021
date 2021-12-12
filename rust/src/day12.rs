use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;

type Routes = HashMap<String, HashSet<String>>;
#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Routes {
    let mut out: HashMap<String, HashSet<String>> = HashMap::new();
    input.lines().for_each(|l| {
        let mut parts = l.split('-');
        let src = parts.next().unwrap();
        let dest = parts.next().unwrap();
        if dest != "start" {
            out.entry(src.to_string())
                .and_modify(|x| {
                    x.insert(dest.to_string());
                })
                .or_insert({
                    let mut x = HashSet::new();
                    x.insert(dest.to_string());
                    x
                });
        }
        if src != "start" {
            out.entry(dest.to_string())
                .and_modify(|x| {
                    x.insert(src.to_string());
                })
                .or_insert({
                    let mut x = HashSet::new();
                    x.insert(src.to_string());
                    x
                });
        }
    });

    out
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Routes) -> usize {
    let mut total: usize = 0;
    let mut seen = HashSet::new();
    let mut path = Vec::new();
    let mut found_paths = Vec::new();
    dbg!(&input);

    recurse(input, "start", &mut total, seen, path, &mut found_paths);
    dbg!(&found_paths);
    total
}

pub fn recurse(
    routes: &Routes,
    node: &str,
    total: &mut usize,
    mut seen: HashSet<String>,
    mut path: Vec<String>,
    found_paths: &mut Vec<Vec<String>>,
) {
    path.push(node.to_string());
    if node.to_ascii_lowercase() == node {
        seen.insert(node.to_string());
    }
    if node == "end" {
        *total += 1;
        found_paths.push(path);
        return;
    }

    routes.get(node).iter().for_each(|s| {
        s.iter()
            .filter(|x| !seen.contains(*x))
            .for_each(|d| recurse(routes, d, total, seen.clone(), path.clone(), found_paths))
    });
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Routes) -> usize {
    let mut total: usize = 0;
    let mut seen = HashMap::new();
    let mut path = Vec::new();
    let mut found_paths = Vec::new();
    // dbg!(&input);

    recurse2(
        input,
        "start",
        &mut total,
        seen,
        false,
        path,
        &mut found_paths,
    );
    // dbg!(&found_paths);
    total
}

pub fn recurse2(
    routes: &Routes,
    node: &str,
    total: &mut usize,
    mut seen: HashMap<String, usize>,
    revisit_used: bool,
    mut path: Vec<String>,
    found_paths: &mut Vec<Vec<String>>,
) {
    path.push(node.to_string());
    if node.to_ascii_lowercase() == node {
        seen.entry(node.to_string())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    if node == "end" {
        if !found_paths.contains(&path) {
            *total += 1;
            found_paths.push(path);
        }
        return;
    }

    routes.get(node).iter().for_each(|s| {
        s.iter()
            .filter(|x| match seen.get(*x) {
                Some(z) if *z >= 2 => false,
                Some(z) if revisit_used && *z >= 1 => false,
                _ => true,
            })
            .for_each(|d| {
                if revisit_used {
                    recurse2(
                        routes,
                        d,
                        total,
                        seen.clone(),
                        revisit_used,
                        path.clone(),
                        found_paths,
                    )
                } else {
                    match seen.get(&*d) {
                        Some(z) if *z == 1 => {
                            recurse2(
                                routes,
                                d,
                                total,
                                seen.clone(),
                                true,
                                path.clone(),
                                found_paths,
                            );
                        }
                        _ => {
                            recurse2(
                                routes,
                                d,
                                total,
                                seen.clone(),
                                revisit_used,
                                path.clone(),
                                found_paths,
                            );
                        }
                    }
                }
            })
    });
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 10);
    }
    #[test]
    fn test_2() {
        let inp = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 19);
    }
    #[test]
    fn test_3() {
        let inp = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 226);
    }
    #[test]
    fn test_4() {
        let inp = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 36);
    }
    #[test]
    fn test_5() {
        let inp = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 103);
    }
    #[test]
    fn test_6() {
        let inp = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 3509);
    }
}
