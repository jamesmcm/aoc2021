use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut lines = input.lines();

    let ie = lines.next().unwrap().chars().map(|x| x == '#').collect();
    lines.next();

    let mut grid = Vec::new();
    for l in lines {
        grid.push(l.chars().map(|x| x == '#').collect());
    }

    (ie, grid)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    let mut base_image = input.1.clone();
    for i in 0..2 {
        let empty = input.0[0] && (i % 2 == 1);
        // Enclose image with 1pixel border
        dbg!(base_image.len(), base_image[0].len());
        let mut new_image = Vec::new();
        new_image.push(vec![empty; base_image[0].len() + 2]);
        for line in base_image.iter() {
            let mut l = Vec::with_capacity(line.len() + 2);
            l.push(empty);
            for x in line {
                l.push(*x);
            }
            l.push(empty);
            new_image.push(l);
        }
        new_image.push(vec![empty; base_image[0].len() + 2]);

        base_image = new_image
            .iter()
            .enumerate()
            .map(|l| {
                l.1.iter()
                    .enumerate()
                    .map(|x| get_val(&new_image, (l.0, x.0), &input.0, empty))
                    .collect()
            })
            .collect();
        // dbg!(&base_image);
    }
    base_image
        .iter()
        .map(|l| l.iter().filter(|x| **x).count())
        .sum()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    let mut base_image = input.1.clone();
    for i in 0..50 {
        let empty = input.0[0] && (i % 2 == 1);
        // Enclose image with 1pixel border
        dbg!(base_image.len(), base_image[0].len());
        let mut new_image = Vec::new();
        new_image.push(vec![empty; base_image[0].len() + 2]);
        for line in base_image.iter() {
            let mut l = Vec::with_capacity(line.len() + 2);
            l.push(empty);
            for x in line {
                l.push(*x);
            }
            l.push(empty);
            new_image.push(l);
        }
        new_image.push(vec![empty; base_image[0].len() + 2]);

        base_image = new_image
            .iter()
            .enumerate()
            .map(|l| {
                l.1.iter()
                    .enumerate()
                    .map(|x| get_val(&new_image, (l.0, x.0), &input.0, empty))
                    .collect()
            })
            .collect();
        // dbg!(&base_image);
    }
    base_image
        .iter()
        .map(|l| l.iter().filter(|x| **x).count())
        .sum()
}

pub fn get_val(grid: &Vec<Vec<bool>>, pos: (usize, usize), ie: &[bool], empty: bool) -> bool {
    ie[get_index(grid, pos, empty)]
}

pub fn get_index(grid: &Vec<Vec<bool>>, pos: (usize, usize), empty: bool) -> usize {
    let (y, x) = pos;
    let mut out: usize = 0;
    // dbg!(&grid);
    let bin = [
        y.checked_sub(1)
            .map(|ym| x.checked_sub(1).map(|xm| grid[ym][xm]).unwrap_or(empty))
            .unwrap_or(empty),
        y.checked_sub(1).map(|ym| grid[ym][x]).unwrap_or(empty),
        y.checked_sub(1)
            .map(|ym| (grid[ym].get(x + 1).map(|z| *z)).unwrap_or(empty))
            .unwrap_or(empty),
        x.checked_sub(1).map(|xm| grid[y][xm]).unwrap_or(empty),
        grid[y][x],
        (grid[y].get(x + 1).map(|z| *z)).unwrap_or(empty),
        grid.get(y + 1)
            .map(|yp| x.checked_sub(1).map(|xm| yp[xm]).unwrap_or(empty))
            .unwrap_or(empty),
        grid.get(y + 1).map(|yp| yp[x]).unwrap_or(empty),
        grid.get(y + 1)
            .map(|yp| yp.get(x + 1).map(|z| *z).unwrap_or(empty))
            .unwrap_or(empty),
    ];
    // dbg!(&bin);
    for x in (0..bin.len()) {
        out = out << 1;
        out += (if bin[x] { 1 } else { 0 });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let inp = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let parsed = input_generator(inp);
        assert_eq!(get_index(&parsed.1, (2, 2), false), 34);
    }
    #[test]
    fn test_part1() {
        let inp = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 35);
    }
    #[test]
    fn test_part2() {
        let inp = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 3351);
    }
}
