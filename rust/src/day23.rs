use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    pub fn from_char(c: char) -> Amphipod {
        use Amphipod::*;
        match c {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            _ => panic!("bad char"),
        }
    }
    pub fn move_cost(&self) -> usize {
        use Amphipod::*;
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }
    pub fn index(&self) -> usize {
        use Amphipod::*;
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }
}

type State = ([Vec<Amphipod>; 4], [Option<Amphipod>; 7]);

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> State {
    let re0 = Regex::new("#(\\.|A|B|C|D)(\\.|A|B|C|D)\\.(\\.|A|B|C|D)\\.(\\.|A|B|C|D)\\.(\\.|A|B|C|D)\\.(\\.|A|B|C|D)(\\.|A|B|C|D)#").unwrap();
    let re1 = Regex::new("###(\\.|A|B|C|D)#(\\.|A|B|C|D)#(\\.|A|B|C|D)#(\\.|A|B|C|D)###").unwrap();
    let re2 = Regex::new("  #(\\.|A|B|C|D)#(\\.|A|B|C|D)#(\\.|A|B|C|D)#(\\.|A|B|C|D)#").unwrap();
    let mut it = input.lines();
    it.next();
    let c0 = re0.captures(it.next().unwrap()).unwrap();
    let c1 = re1.captures(it.next().unwrap()).unwrap();
    let c2 = re2.captures(it.next().unwrap()).unwrap();
    let mut rooms: [Vec<Amphipod>; 4] = [vec![], vec![], vec![], vec![]];
    let mut halls: [Option<Amphipod>; 7] = [None; 7];
    (1..=4).for_each(|i| {
        let ch2 = c2.get(i).unwrap().as_str().chars().next().unwrap();
        let ch1 = c1.get(i).unwrap().as_str().chars().next().unwrap();
        let mut out = Vec::new();
        if ch2 != '.' {
            out.push(Amphipod::from_char(ch2))
        };
        if ch1 != '.' {
            out.push(Amphipod::from_char(ch1))
        };
        rooms[i - 1] = out;
    });
    (1..=7).for_each(|i| {
        let c = c0.get(i).unwrap().as_str().chars().next().unwrap();
        halls[i - 1] = if c == '.' {
            None
        } else {
            Some(Amphipod::from_char(c))
        };
    });
    (rooms, halls)
}

pub fn hall_to_x(hall: usize) -> usize {
    match hall {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 5,
        4 => 7,
        5 => 9,
        6 => 10,
        _ => panic!("unexpected hall"),
    }
}
pub fn room_to_x(room: usize) -> usize {
    match room {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => panic!("unexpected room"),
    }
}

pub fn hall_to_room_cost(hall: usize, room: usize) -> usize {
    let hallx = hall_to_x(hall);
    let roomx = room_to_x(room);

    if hallx > roomx {
        hallx - roomx
    } else {
        roomx - hallx
    }
}
pub fn room_to_room_cost(room1: usize, room2: usize) -> usize {
    let room1x = room_to_x(room1);
    let room2x = room_to_x(room2);

    if room1x > room2x {
        room1x - room2x
    } else {
        room2x - room1x
    }
}

// Can only ever move to destination or not from any given hall
pub fn new_hall_states(
    state: &State,
    mincost: &mut HashMap<State, usize>,
    best_from: &mut HashMap<State, State>,
    room_size: usize,
) -> Vec<State> {
    let (rooms, halls) = state;
    halls
        .iter()
        .enumerate()
        .map(|(i, h)| {
            if let Some(&x) = h.as_ref() {
                let dest = x.index();
                // Is dest free?
                let dest_free = (rooms[dest].len() == 0)
                    || (rooms[dest].iter().all(|x| x.index() == dest)
                        && rooms[dest].len() < room_size);
                // Are halls to dest free?
                let halls_free = (if i > (dest + 1) {
                    dest + 2..=i
                } else {
                    i + 1..=dest + 1
                })
                .all(|z| if z == i { true } else { halls[z].is_none() });

                if dest_free && halls_free {
                    // Calculate movement cost
                    let new_cost = x.move_cost()
                        * (hall_to_room_cost(i, dest)
                            + (room_size - rooms[dest].len()));
                    let cost = mincost.get(&state).expect("mincost missing") + new_cost;
                    let mut new_halls = halls.clone();
                    let to_move = new_halls[i].take().unwrap(); // x
                    let mut new_rooms = rooms.clone();
                    new_rooms[dest].push(to_move);
                    let new_state = (new_rooms, new_halls);
                    if mincost.get(&new_state).map(|x| *x).unwrap_or(usize::MAX) > cost {
                        // Update minimum cost map
                        // Update min cost from map
                        mincost
                            .entry(new_state.clone())
                            .and_modify(|v| *v = cost)
                            .or_insert(cost);
                        best_from
                            .entry(new_state.clone())
                            .and_modify(|v| *v = state.clone())
                            .or_insert(state.clone());
                        return Some(new_state);
                    }
                }
                None
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .collect()
}

// Can move to destination or to any reachable hall
// If 2 elements in room only outer one can leave
pub fn new_room_states(
    state: &State,
    mincost: &mut HashMap<State, usize>,
    best_from: &mut HashMap<State, State>,
    room_size: usize,
) -> Vec<State> {
    let (rooms, halls) = state;

    let mut room_to_room: Vec<State> = rooms
        .iter()
        .enumerate()
        .map(|(i, r)| {
            if r.len() > 0 {
                let x = r[r.len() - 1];
                let dest = x.index();
                // Is dest free?
                let mut dest_free = (rooms[dest].len() == 0)
                    || (rooms[dest].iter().all(|x| x.index() == dest)
                        && rooms[dest].len() < room_size);
                // Do not try to move to dest if already in dest
                if dest == i {
                    dest_free = false;
                }
                // Are halls to dest free?
                let halls_free = if i > dest {
                    dest + 2..=i + 1
                } else {
                    i + 2..=dest + 1
                }
                .all(|z| halls[z].is_none());

                if dest_free && halls_free {
                    // Calculate movement cost
                    let new_cost = x.move_cost()
                        * (room_to_room_cost(i, dest) 
                            + (room_size - rooms[dest].len())  // Vertical move down
                            + (1 + room_size - r.len())); // Vertical move up
                    let cost = mincost.get(&state).expect("mincost missing") + new_cost;
                    let mut new_halls = halls.clone();
                    let mut new_rooms = rooms.clone();
                    let to_move = new_rooms[i].pop().unwrap();
                    new_rooms[dest].push(to_move);
                    let new_state = (new_rooms, new_halls);
                    if mincost.get(&new_state).map(|x| *x).unwrap_or(usize::MAX) > cost {
                        // Update minimum cost map
                        // Update min cost from map
                        mincost
                            .entry(new_state.clone())
                            .and_modify(|v| *v = cost)
                            .or_insert(cost);
                        best_from
                            .entry(new_state.clone())
                            .and_modify(|v| *v = state.clone())
                            .or_insert(state.clone());
                        return Some(new_state);
                    }
                }
                None
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .collect();

    let mut room_to_halls: Vec<State> = rooms
        .iter()
        .enumerate()
        .map(|(i, r)| {
            // Do not move out of room if room solved
            if r.iter().all(|x| x.index() == i) {
                return vec![];
            }
            if r.len() == 0 {
                return vec![];
            }

            (0..(halls.len()))
                .map(|dh| {
                    let x = r[r.len() - 1];
                    // Is dest free?
                    let dest_free = halls[dh].is_none();
                    // Are halls to dest free?
                    let halls_free = if (i + 1) >= dh {
                        dh..=i + 1
                    } else {
                        i + 2..=dh
                    }
                    .all(|z| halls[z].is_none());

                    if dest_free && halls_free {
                        // Calculate movement cost
                        let new_cost = x.move_cost()
                            * (hall_to_room_cost(dh, i) + (1+ room_size - r.len()) ); // Vertical move up
                        let cost = mincost.get(&state).expect("mincost missing") + new_cost;
                        let mut new_halls = halls.clone();
                        let mut new_rooms = rooms.clone();
                        let to_move = new_rooms[i].pop().unwrap();
                        new_halls[dh] = Some(to_move);

                        let new_state = (new_rooms, new_halls);
                        if mincost.get(&new_state).map(|x| *x).unwrap_or(usize::MAX) > cost {
                            // Update minimum cost map
                            // Update min cost from map
                            mincost
                                .entry(new_state.clone())
                                .and_modify(|v| *v = cost)
                                .or_insert(cost);
                            best_from
                                .entry(new_state.clone())
                                .and_modify(|v| *v = state.clone())
                                .or_insert(state.clone());
                            return Some(new_state);
                        }
                    }
                    None
                })
                .collect()
        })
        .flatten()
        .filter_map(|x| x)
        .collect();

    room_to_room.append(&mut room_to_halls);
    room_to_room
}

pub fn p1_to_p2(s: &State) -> State {
    use Amphipod::*;
    let mut news = s.clone();
    let last_val = news.0[0].pop().unwrap();
    news.0[0].push(D);
    news.0[0].push(D);
    news.0[0].push(last_val);

    let last_val = news.0[1].pop().unwrap();
    news.0[1].push(B);
    news.0[1].push(C);
    news.0[1].push(last_val);
    let last_val = news.0[2].pop().unwrap();
    news.0[2].push(A);
    news.0[2].push(B);
    news.0[2].push(last_val);
    let last_val = news.0[3].pop().unwrap();
    news.0[3].push(C);
    news.0[3].push(A);
    news.0[3].push(last_val);
    news

}

#[aoc(day23, part1)]
pub fn solve_part1(input: &State) -> usize {
    use Amphipod::*;
    let mut state = input.clone();
    let room_size: usize = 2;

    let mut mincost: HashMap<State, usize> = HashMap::new();
    let mut best_from: HashMap<State, State> = HashMap::new();
    let solved_state = (
        [vec![A, A], vec![B, B], vec![C, C], vec![D, D]],
        [None, None, None, None, None, None, None],
    );

    mincost.insert(state.clone(), 0);

    let mut pos_states: Vec<State> = vec![state.clone()];

    while !pos_states.is_empty() {
        // For possible moves, first check any in halls - can they move to destination (they can
        // never move to hall from hall)
        let s = pos_states.pop().unwrap();
        let mut hall_states: Vec<State> = new_hall_states(&s, &mut mincost, &mut best_from, room_size);
        // Then any in rooms - can they leave and enter hall or destination
        let mut room_states: Vec<State> = new_room_states(&s, &mut mincost, &mut best_from, room_size);
        pos_states.append(&mut room_states);
        pos_states.append(&mut hall_states);
    }
    *mincost.get(&solved_state).expect("No solution")
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &State) -> usize {
    use Amphipod::*;
    let mut state = p1_to_p2(input);
    let room_size: usize = 4;

    let mut mincost: HashMap<State, usize> = HashMap::new();
    let mut best_from: HashMap<State, State> = HashMap::new();
    let solved_state = (
        [vec![A, A,A,A], vec![B, B,B,B], vec![C, C,C,C], vec![D, D,D,D]],
        [None, None, None, None, None, None, None],
    );

    mincost.insert(state.clone(), 0);

    let mut pos_states: Vec<State> = vec![state.clone()];

    while !pos_states.is_empty() {
        // For possible moves, first check any in halls - can they move to destination (they can
        // never move to hall from hall)
        let s = pos_states.pop().unwrap();
        let mut hall_states: Vec<State> = new_hall_states(&s, &mut mincost, &mut best_from, room_size);
        // Then any in rooms - can they leave and enter hall or destination
        let mut room_states: Vec<State> = new_room_states(&s, &mut mincost, &mut best_from, room_size);
        pos_states.append(&mut room_states);
        pos_states.append(&mut hall_states);
    }
    *mincost.get(&solved_state).expect("No solution")
}

#[cfg(test)]
mod tests {
    use super::*;
    use Amphipod::*;

    #[test]
    fn test_parse() {
        let inp = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

        assert_eq!(
            input_generator(&inp),
            ([vec![A, B], vec![D, C], vec![C, B], vec![A, D]], [None; 7])
        );
    }
    #[test]
    fn test_hallcost1() {
        assert_eq!(hall_to_room_cost(1, 0), 1);
    }
    #[test]
    fn test_hallcost2() {
        assert_eq!(hall_to_room_cost(0, 0), 2);
    }
    #[test]
    fn test_hallcost3() {
        assert_eq!(hall_to_room_cost(0, 1), 4);
    }
    #[test]
    fn test_hallcost4() {
        assert_eq!(hall_to_room_cost(0, 2), 6);
    }
    #[test]
    fn test_hallcost5() {
        assert_eq!(hall_to_room_cost(2, 2), 3);
    }
    #[test]
    fn test_hallcost6() {
        assert_eq!(hall_to_room_cost(4, 2), 1);
    }
    #[test]
    fn test_hallcost7() {
        assert_eq!(hall_to_room_cost(1, 2), 5);
    }
    #[test]
    fn test_hallcost8() {
        assert_eq!(hall_to_room_cost(2, 1), 1);
    }
    #[test]
    fn test_hallcost9() {
        assert_eq!(hall_to_room_cost(3, 1), 1);
    }
    #[test]
    fn test_hallcost10() {
        assert_eq!(hall_to_room_cost(4, 1), 3);
    }
    #[test]
    fn test_hallmove1() {
        let inp = "#############
#.........A.#
###.#C#B#B###
  #A#D#C#D#
  #########";
        let state = input_generator(&inp);
        let mut mincost: HashMap<State, usize> = HashMap::new();
        let mut best_from: HashMap<State, State> = HashMap::new();
        mincost.insert(state.clone(), 0);
        let hall_states: Vec<State> = new_hall_states(&state, &mut mincost, &mut best_from, 2);
        let new_state = ([vec![A, A], vec![D, C], vec![C, B], vec![D, B]], [None; 7]);
        assert_eq!(hall_states, vec![new_state.clone()]);
        assert_eq!(*mincost.get(&new_state).unwrap(), 8);
    }
    #[test]
    fn test_hallmove2() {
        let inp = "#############
#.....A.C...#
###.#D#.#B###
  #A#B#C#D#
  #########";
        let state = input_generator(&inp);
        let mut mincost: HashMap<State, usize> = HashMap::new();
        let mut best_from: HashMap<State, State> = HashMap::new();
        mincost.insert(state.clone(), 0);
        let hall_states: Vec<State> = new_hall_states(&state, &mut mincost, &mut best_from, 2);
        let new_state1 = (
            [vec![A, A], vec![B, D], vec![C], vec![D, B]],
            [None, None, None, None, Some(C), None, None],
        );
        let new_state2 = (
            [vec![A], vec![B, D], vec![C, C], vec![D, B]],
            [None, None, None, Some(A), None, None, None],
        );
        assert_eq!(hall_states, vec![new_state1.clone(), new_state2.clone()]);
        assert_eq!(*mincost.get(&new_state1).unwrap(), 4);
        assert_eq!(*mincost.get(&new_state2).unwrap(), 200);
    }
    #[test]
    fn test_hallmove3() {
        let inp = "#############
#...C.A.....#
###.#D#.#B###
  #A#B#C#D#
  #########";
        let state = input_generator(&inp);
        let mut mincost: HashMap<State, usize> = HashMap::new();
        let mut best_from: HashMap<State, State> = HashMap::new();
        mincost.insert(state.clone(), 0);
        let hall_states: Vec<State> = new_hall_states(&state, &mut mincost, &mut best_from, 2);
        assert_eq!(hall_states, vec![]);
    }
    #[test]
    fn test_roommove1() {
        let inp = "#############
#A..........#
###D#B#C#.###
  #A#B#C#D#
  #########";
        let state = input_generator(&inp);
        let mut mincost: HashMap<State, usize> = HashMap::new();
        let mut best_from: HashMap<State, State> = HashMap::new();
        mincost.insert(state.clone(), 0);
        let hall_states: Vec<State> = new_hall_states(&state, &mut mincost, &mut best_from, 2);
        assert_eq!(hall_states, vec![]);

        let new_state1 = (
            [vec![A], vec![B, B], vec![C, C], vec![D, D]],
            [Some(A), None, None, None, None, None, None],
        );

        let room_states: Vec<State> = new_room_states(&state, &mut mincost, &mut best_from, 2);
        assert!(room_states.contains(&new_state1));
        assert_eq!(*mincost.get(&new_state1).unwrap(), 8000);
    }
    #[test]
    fn test_roomhallmove1() {
        let inp = "#############
#A..C.A.B.C.#
###.#.#.#.###
  #D#B#.#D#
  #########";
        let state = input_generator(&inp);
        let mut mincost: HashMap<State, usize> = HashMap::new();
        let mut best_from: HashMap<State, State> = HashMap::new();
        mincost.insert(state.clone(), 0);
        let hall_states: Vec<State> = new_hall_states(&state, &mut mincost, &mut best_from, 2);
        assert_eq!(hall_states, vec![]);

        let new_state1 = (
            [vec![], vec![B], vec![], vec![D]],
            [Some(A), Some(D), Some(C), Some(A), Some(B), Some(C), None],
        );

        let room_states: Vec<State> = new_room_states(&state, &mut mincost, &mut best_from, 2);
        assert_eq!(room_states, vec![new_state1.clone()]);
        assert_eq!(*mincost.get(&new_state1).unwrap(), 3000);
    }
    #[test]
    fn test_roommove2() {
        let inp = "#############
#DA.........#
###D#B#C#.###
  #A#B#C#.#
  #########";
        let state = input_generator(&inp);
        let mut mincost: HashMap<State, usize> = HashMap::new();
        let mut best_from: HashMap<State, State> = HashMap::new();
        mincost.insert(state.clone(), 0);
        let hall_states: Vec<State> = new_hall_states(&state, &mut mincost, &mut best_from, 2);
        assert_eq!(hall_states, vec![]);

        let new_state1 = (
            [vec![A], vec![B, B], vec![C, C], vec![D]],
            [Some(D), Some(A), None, None, None, None, None],
        );

        let room_states: Vec<State> = new_room_states(&state, &mut mincost, &mut best_from, 2);
        assert!(room_states.contains(&new_state1));
        assert_eq!(*mincost.get(&new_state1).unwrap(), 9000);
    }
    fn test_roomhallmove2() {
        let inp = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        let out = "#############
#...B.......#
###B#C#.#D###
  #A#D#C#A#
  #########";
        let state = input_generator(&inp);
        let state2 = input_generator(&out);
        let mut mincost: HashMap<State, usize> = HashMap::new();
        let mut best_from: HashMap<State, State> = HashMap::new();
        mincost.insert(state.clone(), 0);
        let hall_states: Vec<State> = new_hall_states(&state, &mut mincost, &mut best_from, 2);
        assert_eq!(hall_states, vec![]);

        let room_states: Vec<State> = new_room_states(&state, &mut mincost, &mut best_from, 2);
        assert!(room_states.contains(&state2));
        assert_eq!(*mincost.get(&state2).unwrap(), 40);
    }
    #[test]
    fn test_part1() {
        let inp = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        let state = input_generator(&inp);
        assert_eq!(solve_part1(&state), 12521);
    }
    #[test]
    fn test_part2() {
        let inp = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        let state = input_generator(&inp);
        assert_eq!(solve_part2(&state), 44169);
    }
}
