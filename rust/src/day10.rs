use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bracket {
    Normal,
    Curly,
    Angle,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sigil {
    pub bracket: Bracket,
    pub state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validation {
    Valid,
    Incomplete,
    Corrupt(Sigil),
}

impl Sigil {
    pub fn from_char(c: char) -> Self {
        match c {
            '(' => Self {
                bracket: Bracket::Normal,
                state: State::Open,
            },
            ')' => Self {
                bracket: Bracket::Normal,
                state: State::Close,
            },
            '{' => Self {
                bracket: Bracket::Curly,
                state: State::Open,
            },
            '}' => Self {
                bracket: Bracket::Curly,
                state: State::Close,
            },
            '<' => Self {
                bracket: Bracket::Angle,
                state: State::Open,
            },
            '>' => Self {
                bracket: Bracket::Angle,
                state: State::Close,
            },
            '[' => Self {
                bracket: Bracket::Square,
                state: State::Open,
            },
            ']' => Self {
                bracket: Bracket::Square,
                state: State::Close,
            },
            _ => {
                panic!("Bad char");
            }
        }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<Sigil>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Sigil::from_char(c))
                .collect::<Vec<Sigil>>()
        })
        .collect()
}

pub fn validate(line: &[Sigil]) -> Validation {
    let mut stack: Vec<Sigil> = Vec::new();
    for c in line {
        if c.state == State::Open {
            stack.push(*c);
        } else {
            let last_element: Option<Sigil> = stack.pop();
            match last_element {
                None => {
                    return Validation::Corrupt(*c);
                }
                Some(le) => {
                    if le.bracket != c.bracket {
                        return Validation::Corrupt(*c);
                    }
                }
            }
        }
    }
    if stack.is_empty() {
        Validation::Valid
    } else {
        Validation::Incomplete
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Vec<Sigil>>) -> usize {
    let mut output = 0;
    for l in input {
        let valid_state = validate(&l);
        match valid_state {
            Validation::Corrupt(Sigil {
                bracket: Bracket::Normal,
                state: State::Close,
            }) => {
                output += 1;
            }
            Validation::Corrupt(Sigil {
                bracket: Bracket::Square,
                state: State::Close,
            }) => {
                output += 2;
            }
            Validation::Corrupt(Sigil {
                bracket: Bracket::Curly,
                state: State::Close,
            }) => {
                output += 3;
            }
            Validation::Corrupt(Sigil {
                bracket: Bracket::Angle,
                state: State::Close,
            }) => {
                output += 4;
            }
            _ => {}
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let inp = "(())";
        let parsed = input_generator(inp);
        assert_eq!(validate(&parsed[0]), Validation::Valid);
    }
    #[test]
    fn test_basic2() {
        let inp = "(()";
        let parsed = input_generator(inp);
        assert_eq!(validate(&parsed[0]), Validation::Incomplete);
    }
    #[test]
    fn test_basic3() {
        let inp = "(()))";
        let parsed = input_generator(inp);
        assert_eq!(
            validate(&parsed[0]),
            Validation::Corrupt(Sigil {
                bracket: Bracket::Normal,
                state: State::Close
            })
        );
    }
    #[test]
    fn test_basic4() {
        let inp = ")";
        let parsed = input_generator(inp);
        assert_eq!(
            validate(&parsed[0]),
            Validation::Corrupt(Sigil {
                bracket: Bracket::Normal,
                state: State::Close
            })
        );
    }
    #[test]
    fn test_basic5() {
        let inp = "(([{(({[[]]}))}]))";
        let parsed = input_generator(inp);
        assert_eq!(validate(&parsed[0]), Validation::Valid);
    }
    #[test]
    fn test_basic6() {
        let inp = "(([{(({[[]}))}]))";
        let parsed = input_generator(inp);
        assert_eq!(
            validate(&parsed[0]),
            Validation::Corrupt(Sigil {
                bracket: Bracket::Curly,
                state: State::Close
            })
        );
    }
    #[test]
    fn test_basic7() {
        let inp = "(([{(({[[]}))}])))";
        let parsed = input_generator(inp);
        assert_eq!(
            validate(&parsed[0]),
            Validation::Corrupt(Sigil {
                bracket: Bracket::Curly,
                state: State::Close
            })
        );
    }
    #[test]
    fn test_basic8() {
        let inp = "(([{(({[[]]}))}]))(";
        let parsed = input_generator(inp);
        assert_eq!(validate(&parsed[0]), Validation::Incomplete);
    }
}
