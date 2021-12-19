use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Pos = (i32, i32, i32);
// https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum Rotation {
    Nothing,
    X90,
    X180,
    X270,
    Z90,
    Z90Y90,
    Z90Y180,
    Z90Y270,
    Z180,
    Z180X90,
    Z180X180,
    Z180X270,
    Z270,
    Z270Y90,
    Z270Y180,
    Z270Y270,
    Y90,
    Y90Z90,
    Y90Z180,
    Y90Z270,
    Y270,
    Y270Z270,
    Y270Z180,
    Y270Z90,
}

impl Rotation {
    pub fn apply(&self, pos: Pos) -> Pos {
        use Rotation::*;
        match self {
            Nothing => pos,
            X90 => (pos.0, -pos.2, pos.1),
            X180 => (pos.0, -pos.1, -pos.2),
            X270 => (pos.0, pos.2, -pos.1),

            Z90 => (-pos.1, pos.0, pos.2),
            Z90Y90 => (pos.2, pos.0, pos.1),
            Z90Y180 => (pos.1, pos.0, -pos.2),
            Z90Y270 => (-pos.2, pos.0, -pos.1),

            Z180 => (-pos.0, -pos.1, pos.2),
            Z180X90 => (-pos.0, -pos.2, -pos.1),
            Z180X180 => (-pos.0, pos.1, -pos.2),
            Z180X270 => (-pos.0, pos.2, pos.1),

            Z270 => (pos.1, -pos.0, pos.2),
            Z270Y90 => (pos.2, -pos.0, -pos.1),
            Z270Y180 => (-pos.1, -pos.0, -pos.2),
            Z270Y270 => (-pos.2, -pos.0, pos.1),

            Y90 => (-pos.2, pos.1, pos.0),
            Y90Z90 => (pos.1, pos.2, pos.0),
            Y90Z180 => (pos.2, -pos.1, pos.0),
            Y90Z270 => (-pos.1, -pos.2, pos.0),

            Y270 => (-pos.2, -pos.1, -pos.0),
            Y270Z270 => (-pos.1, pos.2, -pos.0),
            Y270Z180 => (pos.2, pos.1, -pos.0),
            Y270Z90 => (pos.1, -pos.2, -pos.0),
        }
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Vec<(i32, i32, i32)>> {
    input
        .split("\n\n")
        .map(|s| {
            let mut it = s.lines();
            let mut v = Vec::new();
            let r = Regex::new("([0-9\\-]+),([0-9\\-]+),([0-9\\-]+)").unwrap();
            it.next();
            for l in it {
                let c = r.captures(l).unwrap();
                v.push((
                    c.get(1).unwrap().as_str().parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    c.get(3).unwrap().as_str().parse().unwrap(),
                ));
            }
            v
        })
        .collect()
}

#[derive(Clone, PartialEq, Debug)]
pub struct SolvedScanner {
    position: Pos,
    rotation: Rotation,
    raw_relative_points: HashSet<Pos>,
    transformed_points_from_origin: HashSet<Pos>,
    label: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnsolvedScanner {
    label: usize,
    raw_relative_points: HashSet<Pos>,
}

pub fn solve_scanners(input: &Vec<Vec<(i32, i32, i32)>>) -> (Vec<SolvedScanner>, HashSet<Pos>) {
    let mut solved_points = HashSet::new();
    let mut solved_scanners = Vec::new();
    let mut unsolved_scanners = Vec::new();
    for p in input[0].iter() {
        solved_points.insert(p.clone());
    }

    solved_scanners.push(SolvedScanner {
        position: (0, 0, 0),
        rotation: Rotation::Nothing,
        raw_relative_points: HashSet::from_iter(input[0].iter().map(|x| *x)),
        transformed_points_from_origin: HashSet::from_iter(input[0].iter().map(|x| *x)),
        label: 0,
    });

    for i in 1..input.len() {
        unsolved_scanners.push(UnsolvedScanner {
            raw_relative_points: HashSet::from_iter(input[i].iter().map(|x| *x)),
            label: i,
        });
    }

    let mut found: Option<SolvedScanner> = None;

    while !unsolved_scanners.is_empty() {
        for unsolved in unsolved_scanners.iter() {
            for solved in solved_scanners.iter() {
                for rot in Rotation::iter() {
                    let transformed_points: HashSet<Pos> = unsolved
                        .raw_relative_points
                        .iter()
                        .map(|x| rot.apply(*x))
                        .collect();

                    // new scanner origin + transformed pos = solved transformed pos from origin
                    // so: new scanner origin = solved transformed pos from origin - transformed pos
                    // Calculate all differences between point sets - all possible origins
                    // Does any origin have 12 or more overlapping points?
                    // If so, origin + rotation is correct

                    let mut possible_origins = Vec::new();
                    for pu in transformed_points.iter() {
                        for ps in solved.transformed_points_from_origin.iter() {
                            possible_origins.push((ps.0 - pu.0, ps.1 - pu.1, ps.2 - pu.2));
                        }
                    }

                    for o in possible_origins {
                        let new_set: HashSet<Pos> = transformed_points
                            .iter()
                            .map(|x| (o.0 + x.0, o.1 + x.1, o.2 + x.2))
                            .collect();

                        let intersect =
                            new_set.intersection(&solved.transformed_points_from_origin);
                        if intersect.count() >= 12 {
                            // println!(
                            //     "Solved: s: {:?} u: {:?} new_origin: {:?}",
                            //     &solved.label, &unsolved.label, &o
                            // );
                            found = Some(SolvedScanner {
                                position: o,
                                raw_relative_points: unsolved.raw_relative_points.clone(),
                                transformed_points_from_origin: new_set.clone(),
                                rotation: rot,
                                label: unsolved.label,
                            });
                            break;
                        }
                    }
                }
            }
            if found.is_some() {
                break;
            }
        }

        if let Some(f) = &found {
            solved_scanners.push(f.clone());
            unsolved_scanners.retain(|x| x.label != f.label);

            // println!("Solved: {:?}", &solved_scanners);

            for p in f.transformed_points_from_origin.iter() {
                solved_points.insert(*p);
            }
        }
        found = None
    }

    (solved_scanners, solved_points)
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Vec<Vec<(i32, i32, i32)>>) -> usize {
    let (_, solved_points) = solve_scanners(input);
    solved_points.len()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Vec<Vec<(i32, i32, i32)>>) -> i32 {
    let (solved_scanners, _) = solve_scanners(input);

    let mut max_dist: i32 = 0;
    for s1 in &solved_scanners {
        for s2 in &solved_scanners {
            let d = manhattan(s1.position, s2.position);
            if d > max_dist {
                max_dist = d;
            }
        }
    }
    max_dist
}

pub fn manhattan(a: Pos, b: Pos) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let inp = "--- scanner 0 ---
404,-588,-901

--- scanner 1 ---
605,423,415";
        let parsed = input_generator(inp);
        assert_eq!(parsed, vec![vec![(404, -588, -901)], vec![(605, 423, 415)]]);
    }
    #[test]
    fn test_part1() {
        let inp = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        let parsed = input_generator(inp);
        assert_eq!(solve_part1(&parsed), 79);
    }
    #[test]
    fn test_part2() {
        let inp = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        let parsed = input_generator(inp);
        assert_eq!(solve_part2(&parsed), 3621);
    }
}
