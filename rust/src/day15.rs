use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;

type Index = (usize, usize);

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn manhattan(start: Index, end: Index) -> usize {
    (end.0 - start.0) + (end.1 - start.1)
}

pub fn get_neighbours(tiles: &Vec<Vec<usize>>, pos: Index) -> Vec<Index> {
    let mut out = Vec::new();
    let down = (pos.0 + 1, pos.1);
    let right = (pos.0, pos.1 + 1);
    let left = (pos.0, pos.1.checked_sub(1));
    let up = (pos.0.checked_sub(1), pos.1);

    if tiles.get(down.0).is_some() {
        out.push(down);
    }
    if tiles[right.0].get(right.1).is_some() {
        out.push(right);
    }
    if left.1.is_some() {
        out.push((left.0, left.1.unwrap()));
    }
    if up.0.is_some() {
        out.push((up.0.unwrap(), up.1));
    }
    out
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    let start = (0, 0);
    let end = (input.len() - 1, input[0].len() - 1);

    let mut openset = HashSet::new();
    openset.insert(start.clone());

    let mut gscore = HashMap::new();
    gscore.insert(start, 0);

    let mut fscore = HashMap::new();
    fscore.insert(start, manhattan(start, end));
    let mut came_from = HashMap::new();

    while openset.len() > 0 {
        // dbg!(&openset);
        let curpos: Index = *openset
            .iter()
            .map(|x| (x, fscore.get(x).unwrap()))
            .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
            .unwrap()
            .0;
        if curpos == end {
            return *gscore.get(&curpos).unwrap();
        }

        openset.remove(&curpos);
        for n in get_neighbours(input, curpos) {
            let tentative_gscore: usize = gscore.get(&curpos).unwrap() + input[n.0][n.1];
            if tentative_gscore < *gscore.get(&n).unwrap_or(&99999999999999) {
                came_from
                    .entry(n)
                    .and_modify(|v| *v = curpos)
                    .or_insert(curpos);
                gscore
                    .entry(n)
                    .and_modify(|v| *v = tentative_gscore)
                    .or_insert(tentative_gscore);
                fscore
                    .entry(n)
                    .and_modify(|v| *v = tentative_gscore + manhattan(n, end))
                    .or_insert(tentative_gscore + manhattan(n, end));
                if !openset.contains(&n) {
                    openset.insert(n);
                }
            }
        }
    }

    return 99999999;
}

pub fn generate_large(tiles: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut mult: Vec<Vec<usize>> = Vec::new();
    for y in 0..=4 {
        let mut cur_row = Vec::new();
        for x in 0..=4 {
            cur_row.push(y + x);
        }
        mult.push(cur_row);
    }

    let split: Vec<Vec<Vec<Vec<usize>>>> = mult
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|x| {
                    tiles
                        .iter()
                        .map(|z| {
                            z.iter()
                                .map(|v| {
                                    let mut newv = v + x;
                                    if newv > 9 {
                                        newv -= 9;
                                    }
                                    newv
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    split
        .into_iter()
        .map(|r| {
            r.into_iter()
                .reduce(|x, y| merge_horizontal(&x, &y))
                .unwrap()
        })
        .reduce(|x, y| merge_vertical(&x, &y))
        .unwrap()
}

pub fn merge_horizontal(left: &Vec<Vec<usize>>, right: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut out = left.clone();
    for row in 0..right.len() {
        for x in right[row].iter() {
            out[row].push(*x);
        }
    }
    out
}
pub fn merge_vertical(up: &Vec<Vec<usize>>, down: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut out = up.clone();
    for row in down {
        out.push(row.clone());
    }

    out
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    solve_part1(&generate_large(input))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 40);
    }
    #[test]
    fn test_gen() {
        let inp = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        let large = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";
        let parsed = input_generator(inp);
        let parsed_large = input_generator(large);

        assert_eq!(generate_large(&parsed), parsed_large);
    }
    #[test]
    fn test_2() {
        let inp = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&generate_large(&parsed)), 315);
    }
}
