use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        match s {
            "forward" => Ok(Forward),
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(()),
        }
    }
}

pub struct Move {
    direction: Direction,
    distance: i32,
}

pub struct Position {
    pub horizontal: i32,
    pub vertical: i32,
    pub aim: i32,
}

impl Move {
    fn process(&self, position: &mut Position) {
        use Direction::*;
        match self.direction {
            Forward => position.horizontal += self.distance,
            Down => position.vertical += self.distance,
            Up => {
                position.vertical -= self.distance;
                if position.vertical < 0 {
                    position.vertical = 0
                }
            }
        }
    }

    fn process2(&self, position: &mut Position) {
        use Direction::*;
        match self.direction {
            Forward => {
                position.horizontal += self.distance;
                position.vertical += position.aim * self.distance
            }
            Down => position.aim += self.distance,
            Up => position.aim -= self.distance,
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let dir = split.next().ok_or(())?;
        let distance = split.next().ok_or(())?;
        Ok(Self {
            direction: Direction::from_str(dir)?,
            distance: distance.parse().map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input.lines().map(|l| Move::from_str(l).unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Move]) -> i32 {
    let mut pos = Position {
        horizontal: 0,
        vertical: 0,
        aim: 0,
    };
    input.iter().for_each(|x| x.process(&mut pos));
    pos.horizontal * pos.vertical
}
#[aoc(day2, part2)]
pub fn solve_part2(input: &[Move]) -> i32 {
    let mut pos = Position {
        horizontal: 0,
        vertical: 0,
        aim: 0,
    };
    input.iter().for_each(|x| x.process2(&mut pos));
    pos.horizontal * pos.vertical
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 150);
    }
    #[test]
    fn test_2() {
        let inp = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 900);
    }
}
