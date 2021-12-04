use aoc_runner_derive::{aoc, aoc_generator};

type RawBoard = Vec<Vec<u32>>;
type Position = (usize, usize);

#[derive(Debug)]
struct Board {
    pub raw: RawBoard,
    pub marked: Vec<Position>,
}

impl Board {
    pub fn from_raw(raw: RawBoard) -> Self {
        Board {
            raw,
            marked: vec![],
        }
    }

    pub fn find_and_mark(&mut self, val: u32) {
        let pos = self
            .raw
            .iter()
            .enumerate()
            .map(|v| (v.0, v.1.iter().enumerate().find(|x| *x.1 == val)))
            .find(|x| x.1.is_some());
        if let Some(pos) = pos {
            self.marked.push((pos.0, (pos.1.unwrap()).0));
        }
    }

    pub fn size(&self) -> usize {
        self.raw.len()
    }

    pub fn is_solved(&self) -> bool {
        let size = self.size();
        (0..size).any(|n| {
            (0..size).all(|i| self.marked.iter().any(|x| x.0 == i && x.1 == n))
                || (0..size).all(|i| self.marked.iter().any(|x| x.1 == i && x.0 == n))
        })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<RawBoard>) {
    let mut lines = input.lines();

    let nums: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    lines.next();

    let mut curboard: RawBoard = vec![];
    let mut boards: Vec<RawBoard> = vec![];
    for line in lines {
        if line.trim().is_empty() {
            boards.push(curboard);
            curboard = vec![];
        } else {
            curboard.push(
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
    }
    if !curboard.is_empty() {
        boards.push(curboard);
    }

    (nums, boards)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<RawBoard>)) -> u32 {
    let numbers = &input.0;
    let mut boards: Vec<Board> = input.1.iter().map(|x| Board::from_raw(x.clone())).collect();
    let mut sol: u32 = 0;
    for n in numbers {
        for b in &mut boards {
            b.find_and_mark(*n);
            if b.is_solved() {
                let size = b.size();
                for x in 0..size {
                    for y in 0..size {
                        if !b.marked.contains(&(x, y)) {
                            sol += b.raw[x][y];
                        }
                    }
                }
                sol *= n;
                break;
            }
        }
        if sol != 0 {
            break;
        }
    }

    sol
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<RawBoard>)) -> u32 {
    let numbers = &input.0;
    let mut boards: Vec<Board> = input.1.iter().map(|x| Board::from_raw(x.clone())).collect();
    let mut sol: u32 = 0;
    let mut solved_count: usize = 0;
    let num_boards = boards.len();
    for n in numbers {
        for b in &mut boards {
            b.find_and_mark(*n);
            if b.is_solved() {
                solved_count += 1;
                if solved_count == num_boards {
                    let size = b.size();
                    for x in 0..size {
                        for y in 0..size {
                            if !b.marked.contains(&(x, y)) {
                                sol += b.raw[x][y];
                            }
                        }
                    }
                    sol *= n;
                    break;
                }
            }
        }
        boards.retain(|b| !b.is_solved());
        if sol != 0 {
            break;
        }
    }

    sol
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 4512);
    }
    #[test]
    fn test_2() {
        let inp = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 1924);
    }
}
