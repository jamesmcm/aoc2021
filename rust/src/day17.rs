use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> ((i32, i32), (i32, i32)) {
    let re =
        Regex::new(r"target area: x=([0-9\-]+)..([0-9\-]+), y=([0-9\-]+)..([0-9\-]+)").unwrap();
    let c = re.captures(input).unwrap();
    (
        (
            c.get(1).unwrap().as_str().parse().unwrap(),
            c.get(2).unwrap().as_str().parse().unwrap(),
        ),
        (
            c.get(3).unwrap().as_str().parse().unwrap(),
            c.get(4).unwrap().as_str().parse().unwrap(),
        ),
    )
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &((i32, i32), (i32, i32))) -> i32 {
    let mut max_y = -999999;
    for xvel in 0..=1000 {
        for yvel in -1000..=1000 {
            let mut vx = xvel;
            let mut vy = yvel;
            let mut xpos = 0;
            let mut ypos = 0;
            let mut local_max_y = -9999999;
            loop {
                if ypos > local_max_y {
                    local_max_y = ypos;
                }
                if xpos >= input.0 .0
                    && xpos <= input.0 .1
                    && ypos >= input.1 .0
                    && ypos <= input.1 .1
                {
                    if local_max_y > max_y {
                        max_y = local_max_y;
                        // println!("{}, {}", xvel, yvel);
                    }
                }

                if xpos > input.0 .1 || (ypos < input.1 .0 && vy < 0) {
                    break;
                }

                xpos += vx;
                ypos += vy;
                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }
                vy -= 1;
            }
        }
    }
    max_y
}
#[aoc(day17, part2)]
pub fn solve_part2(input: &((i32, i32), (i32, i32))) -> usize {
    let mut velset = HashSet::new();
    for xvel in 0..=1000 {
        for yvel in -1000..=1000 {
            let mut vx = xvel;
            let mut vy = yvel;
            let mut xpos = 0;
            let mut ypos = 0;
            loop {
                if xpos >= input.0 .0
                    && xpos <= input.0 .1
                    && ypos >= input.1 .0
                    && ypos <= input.1 .1
                {
                    velset.insert((xvel, yvel));
                }

                if xpos > input.0 .1 || (ypos < input.1 .0 && vy < 0) {
                    break;
                }

                xpos += vx;
                ypos += vy;
                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }
                vy -= 1;
            }
        }
    }
    velset.len()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let inp = "target area: x=20..30, y=-10..-5";
        let parsed = input_generator(inp);
        assert_eq!(parsed, ((20, 30), (-10, -5)));
    }
    #[test]
    fn test_part1() {
        let inp = "target area: x=20..30, y=-10..-5";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 45);
    }
    #[test]
    fn test_part2() {
        let inp = "target area: x=20..30, y=-10..-5";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 112);
    }
}
