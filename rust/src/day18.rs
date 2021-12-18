use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    left: EitherList,
    right: EitherList,
}

impl List {
    pub fn magnitude(&self) -> i32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            left: EitherList::List(Box::new(self)),
            right: EitherList::List(Box::new(other)),
        }
    }
    pub fn explode(mut self) -> Self {
        recurse_down(&mut self, 0, false).0
    }
}

pub fn recurse_down(
    l: &mut List,
    depth: i32,
    mut found: bool,
) -> (List, Option<i32>, Option<i32>, bool) {
    let mut new_list: List = l.clone();
    if found {
        return (new_list, None, None, true);
    }
    if depth >= 3 {
        // dbg!(l.clone());
        match (l.left.clone(), l.right.clone()) {
            (EitherList::Value(_), EitherList::Value(_)) => return (new_list, None, None, found),
            (EitherList::List(l2), EitherList::Value(rv)) => {
                // dbg!((l2.clone(), rv));
                found = true;
                new_list.right = EitherList::Value(rv + l2.right.magnitude());
                new_list.left = EitherList::Value(0);
                return (new_list, Some(l2.left.magnitude()), None, true);
            }
            (EitherList::Value(lv), EitherList::List(r2)) => {
                // dbg!((r2.clone(), lv));
                found = true;
                new_list.left = EitherList::Value(lv + r2.left.magnitude());
                new_list.right = EitherList::Value(0);
                return (new_list, None, Some(r2.right.magnitude()), true);
            }
            (EitherList::List(l2), EitherList::List(r2)) => {
                // Adjacent explosions
                // Do left first and break for recall - then right will be only one next call
                found = true;
                new_list.right = EitherList::List(Box::new(add_left(&r2, l2.right.magnitude())));
                new_list.left = EitherList::Value(0);
                return (new_list, Some(l2.left.magnitude()), None, true);
            }
        }
    } else {
        let mut new_left = match l.left.clone() {
            EitherList::List(mut l2) => {
                // dbg!((l2.clone(), depth + 1));
                let temp = recurse_down(&mut l2, depth + 1, found);
                (EitherList::List(Box::new(temp.0)), temp.1, temp.2, temp.3)
            }
            EitherList::Value(_) => (l.left.clone(), None, None, false),
        };
        found = found || new_left.3;
        let mut new_right = match l.right.clone() {
            EitherList::List(mut l2) => {
                // dbg!((l2.clone(), depth + 1));
                let temp = recurse_down(&mut l2, depth + 1, found);
                (EitherList::List(Box::new(temp.0)), temp.1, temp.2, temp.3)
            }
            EitherList::Value(_) => (l.right.clone(), None, None, false),
        };

        new_list.right = new_right.0.clone();
        new_list.left = new_left.0.clone();

        if let Some(leftadd) = new_left.2 {
            new_list.right = match new_right.0 {
                EitherList::Value(v) => EitherList::Value(v + leftadd),
                EitherList::List(l2) => EitherList::List(Box::new(add_left(&l2, leftadd))),
            }
        }
        if let Some(rightadd) = new_right.1 {
            new_list.left = match new_left.0 {
                EitherList::Value(v) => EitherList::Value(v + rightadd),
                EitherList::List(l2) => EitherList::List(Box::new(add_right(&l2, rightadd))),
            }
        }
        return (new_list, new_left.1, new_right.2, new_left.3 || new_right.3);
    }
}

pub fn split(l: &List, mut found: bool) -> (List, bool) {
    let mut new_list: List = l.clone();
    if found {
        return (new_list, true);
    };
    let new_left = match l.left.clone() {
        EitherList::List(l2) => {
            let res = split(&l2, found);
            if res.1 {
                found = true
            };
            EitherList::List(Box::new(res.0))
        }
        EitherList::Value(v) => {
            if v >= 10 {
                found = true;
                if v % 2 == 0 {
                    EitherList::List(Box::new(List {
                        left: EitherList::Value(v / 2),
                        right: EitherList::Value(v / 2),
                    }))
                } else {
                    EitherList::List(Box::new(List {
                        left: EitherList::Value(v / 2),
                        right: EitherList::Value((v / 2) + 1),
                    }))
                }
            } else {
                EitherList::Value(v)
            }
        }
    };
    new_list.left = new_left;
    if found {
        return (new_list, true);
    };
    let new_right = match l.right.clone() {
        EitherList::List(l2) => {
            let res = split(&l2, found);
            if res.1 {
                found = true
            };

            EitherList::List(Box::new(res.0))
        }
        EitherList::Value(v) => {
            if v >= 10 {
                found = true;
                if v % 2 == 0 {
                    EitherList::List(Box::new(List {
                        left: EitherList::Value(v / 2),
                        right: EitherList::Value(v / 2),
                    }))
                } else {
                    EitherList::List(Box::new(List {
                        left: EitherList::Value(v / 2),
                        right: EitherList::Value((v / 2) + 1),
                    }))
                }
            } else {
                EitherList::Value(v)
            }
        }
    };
    new_list.right = new_right;
    (new_list, found)
}

pub fn reduce(l: &List) -> List {
    let mut new_list = l.clone();
    loop {
        loop {
            let prev_list = new_list.clone();
            new_list = new_list.explode();
            if prev_list == new_list {
                break;
            }
        }

        let res = split(&new_list, false);
        new_list = res.0;
        if !res.1 {
            break;
        }
    }
    new_list
}

pub fn add_left(l: &List, v: i32) -> List {
    let mut new_l = l.clone();
    match l.left.clone() {
        EitherList::Value(val) => {
            new_l.left = EitherList::Value(val + v);
        }
        EitherList::List(l2) => {
            new_l.left = EitherList::List(Box::new(add_left(&l2, v)));
        }
    }
    new_l
}
pub fn add_right(l: &List, v: i32) -> List {
    let mut new_l = l.clone();
    match l.right.clone() {
        EitherList::Value(val) => {
            new_l.right = EitherList::Value(val + v);
        }
        EitherList::List(l2) => {
            new_l.right = EitherList::List(Box::new(add_right(&l2, v)));
        }
    }
    new_l
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EitherList {
    List(Box<List>),
    Value(i32),
}

impl EitherList {
    pub fn magnitude(&self) -> i32 {
        match self {
            Self::List(l) => l.magnitude(),
            Self::Value(v) => *v,
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<List> {
    input
        .lines()
        .map(|l| {
            // Skip first char - we know it must be '[' so we start inside List
            let mut it = l.chars();
            it.next();
            recurse_list(&mut it)
        })
        .collect()
}

pub fn recurse_list<I>(input: &mut I) -> List
where
    I: Iterator<Item = char>,
{
    let left = iter_chars(input).expect("Left was none");
    let right = iter_chars(input).expect("Right was none");

    List { left, right }
}
pub fn iter_chars<I>(input: &mut I) -> Option<EitherList>
where
    I: Iterator<Item = char>,
{
    let mut num: Option<i32> = None;
    loop {
        match input.next() {
            None => break None,
            Some('[') => break Some(EitherList::List(Box::new(recurse_list(input)))),
            Some(']') => {
                if let Some(n) = num {
                    break Some(EitherList::Value(n));
                }
            }
            Some(',') => {
                if let Some(n) = num {
                    break Some(EitherList::Value(n));
                }
            }
            Some(c) => {
                let mut temp_num = num.unwrap_or(0);
                temp_num *= 10;
                temp_num += c.to_digit(10).expect("char not digit") as i32;
                num = Some(temp_num)
            }
        }
    }
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[List]) -> i32 {
    let input_vec: Vec<List> = input.iter().map(|x| x.clone()).collect();
    input_vec
        .into_iter()
        .reduce(|x, y| reduce(&x.add(y)))
        .unwrap()
        .magnitude()
}
#[aoc(day18, part2)]
pub fn solve_part2(input: &[List]) -> i32 {
    let input_vec: Vec<List> = input.iter().map(|x| x.clone()).collect();
    let mut max_magnitude: i32 = 0;
    for i in 0..input_vec.len() {
        for j in 0..input_vec.len() {
            if i != j {
                let mag = reduce(&input_vec[i].clone().add(input_vec[j].clone())).magnitude();
                if mag > max_magnitude {
                    max_magnitude = mag;
                }
            }
        }
    }
    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let inp = "[1,2]";
        let parsed = input_generator(inp);
        assert_eq!(
            parsed[0],
            List {
                left: EitherList::Value(1),
                right: EitherList::Value(2)
            }
        );
    }
    #[test]
    fn test_parse2() {
        let inp = "[[1,2],3]";
        let parsed = input_generator(inp);
        assert_eq!(
            parsed[0],
            List {
                left: EitherList::List(Box::new(List {
                    left: EitherList::Value(1),
                    right: EitherList::Value(2)
                })),
                right: EitherList::Value(3)
            }
        );
    }
    #[test]
    fn test_parse3() {
        let inp = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let parsed = input_generator(inp);
        assert_eq!(
            parsed[0],
            List {
                left: EitherList::List(Box::new(List {
                    left: EitherList::List(Box::new(List {
                        left: EitherList::List(Box::new(List {
                            left: EitherList::Value(1),
                            right: EitherList::Value(2)
                        })),
                        right: EitherList::List(Box::new(List {
                            left: EitherList::Value(3),
                            right: EitherList::Value(4)
                        }))
                    })),
                    right: EitherList::List(Box::new(List {
                        left: EitherList::List(Box::new(List {
                            left: EitherList::Value(5),
                            right: EitherList::Value(6)
                        })),
                        right: EitherList::List(Box::new(List {
                            left: EitherList::Value(7),
                            right: EitherList::Value(8),
                        }))
                    }))
                })),
                right: EitherList::Value(9)
            }
        );
    }
    #[test]
    fn test_parse4() {
        let inp = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]";
        let parsed = input_generator(inp);
        assert_eq!(
            parsed[0],
            List {
                left: EitherList::List(Box::new(List {
                    left: EitherList::List(Box::new(List {
                        left: EitherList::Value(9),
                        right: EitherList::List(Box::new(List {
                            left: EitherList::Value(3),
                            right: EitherList::Value(8)
                        })),
                    })),
                    right: EitherList::List(Box::new(List {
                        left: EitherList::List(Box::new(List {
                            left: EitherList::Value(0),
                            right: EitherList::Value(9)
                        })),
                        right: EitherList::Value(6)
                    }))
                })),
                right: EitherList::List(Box::new(List {
                    left: EitherList::List(Box::new(List {
                        left: EitherList::List(Box::new(List {
                            left: EitherList::Value(3),
                            right: EitherList::Value(7)
                        })),
                        right: EitherList::List(Box::new(List {
                            left: EitherList::Value(4),
                            right: EitherList::Value(9)
                        }))
                    })),
                    right: EitherList::Value(3),
                }))
            }
        );
    }
    #[test]
    fn test_magnitude() {
        let inp = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let parsed = input_generator(inp);
        assert_eq!(parsed[0].magnitude(), 3488);
    }
    #[test]
    fn test_explode() {
        let inp = "[[[[[9,8],1],2],3],4]";
        let parsed = input_generator(inp);
        assert_eq!(
            parsed[0].clone().explode(),
            List {
                left: EitherList::List(Box::new(List {
                    left: EitherList::List(Box::new(List {
                        left: EitherList::List(Box::new(List {
                            left: EitherList::Value(0),
                            right: EitherList::Value(9),
                        })),
                        right: EitherList::Value(2),
                    })),
                    right: EitherList::Value(3),
                })),
                right: EitherList::Value(4),
            }
        );
    }
    #[test]
    fn test_explode2() {
        let inp = "[7,[6,[5,[4,[3,2]]]]]";
        let parsed = input_generator(inp);
        let out = "[7,[6,[5,[7,0]]]]";
        let parsedout = input_generator(out);
        assert_eq!(parsed[0].clone().explode(), parsedout[0]);
    }
    #[test]
    fn test_explode3() {
        let inp = "[[6,[5,[4,[3,2]]]],1]";
        let parsed = input_generator(inp);
        let out = "[[6,[5,[7,0]]],3]";
        let parsedout = input_generator(out);
        assert_eq!(parsed[0].clone().explode(), parsedout[0]);
    }
    #[test]
    fn test_explode4() {
        let inp = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let parsed = input_generator(inp);
        let out = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        // let out = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        let parsedout = input_generator(out);
        assert_eq!(parsed[0].clone().explode(), parsedout[0]);
    }
    #[test]
    fn test_explode5() {
        let inp = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let parsed = input_generator(inp);
        let out = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        let parsedout = input_generator(out);
        assert_eq!(parsed[0].clone().explode(), parsedout[0]);
    }
    #[test]
    fn test_add() {
        let inp1 = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let parsed1 = input_generator(inp1);
        let inp2 = "[1,1]";
        let parsed2 = input_generator(inp2);
        let out = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let parsedout = input_generator(out);
        assert_eq!(parsed1[0].clone().add(parsed2[0].clone()), parsedout[0]);
    }
    #[test]
    fn test_reduce() {
        let inp1 = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let parsed1 = input_generator(inp1);
        let inp2 = "[1,1]";
        let parsed2 = input_generator(inp2);
        let out = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let parsedout = input_generator(out);
        assert_eq!(
            reduce(&parsed1[0].clone().add(parsed2[0].clone())),
            parsedout[0]
        );
    }
    #[test]
    fn test_p1() {
        let inp1 = "[1,1]
[2,2]
[3,3]
[4,4]";
        let parsed1 = input_generator(inp1);
        let out = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        let parsedout = input_generator(out);
        assert_eq!(solve_part1(&parsed1), parsedout[0].magnitude());
    }
    #[test]
    fn test_add_medium() {
        let inp1 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
        let inp2 = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        let out = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]";

        let parsed1 = input_generator(inp1);
        let parsed2 = input_generator(inp2);
        let parsedout = input_generator(out);

        assert_eq!(
            reduce(&parsed1[0].clone().add(parsed2[0].clone())),
            parsedout[0]
        );
    }
    #[test]
    fn test_p1_medium() {
        let inp1 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let parsed1 = input_generator(inp1);
        let out = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let parsedout = input_generator(out);
        assert_eq!(
            solve_part1(&parsed1),
            reduce(&parsedout[0].clone()).magnitude()
        );
    }
    #[test]
    fn test_p1_big() {
        let inp1 = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let parsed1 = input_generator(inp1);
        assert_eq!(solve_part1(&parsed1), 4140);
    }
    #[test]
    fn test_p2() {
        let inp1 = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let parsed1 = input_generator(inp1);
        assert_eq!(solve_part2(&parsed1), 3993);
    }
}
