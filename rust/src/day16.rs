use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<bool> {
    input
        .chars()
        .flat_map(|c| to_bits(c.to_digit(16).unwrap() as u8))
        .collect()
}

pub fn to_bits(x: u8) -> Vec<bool> {
    let mut out: Vec<bool> = Vec::with_capacity(4);
    for z in (0..4).rev() {
        out.push(to_bool(((1 << z) & x) >> z));
    }
    out
}

pub fn to_bool(x: u8) -> bool {
    x > 0
}
pub fn to_u8(x: bool) -> u8 {
    if x {
        1
    } else {
        0
    }
}

pub fn from_bits(x: &[bool]) -> u8 {
    let mut out: u8 = 0;
    for z in 0..x.len() {
        out += to_u8(x[z]) * (1 << (x.len() - z - 1));
    }
    out
}

pub fn from_bits_u16(x: &[bool]) -> u16 {
    let mut out: u16 = 0;
    for z in 0..x.len() {
        out += (to_u8(x[z]) as u16) * (1 << (x.len() - z - 1));
    }
    out
}

pub fn from_bin_str(x: &str) -> Vec<bool> {
    x.chars()
        .map(|c| if c == '0' { false } else { true })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    version: u8,
    packet_type: u8,
    value: Option<usize>,
    length_type: Option<bool>,
    length_subpackets: Option<u16>,
    subpackets: Option<Vec<Packet>>,
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[bool]) -> usize {
    let packet = parse_packet(input, true);
    recurse_sum(&packet.0)
}
#[aoc(day16, part2)]
pub fn solve_part2(input: &[bool]) -> usize {
    let packet = parse_packet(input, true);
    compute_val(&packet.0)
}

pub fn recurse_sum(p: &Packet) -> usize {
    let mut local_v = p.version as usize;
    if let Some(v) = p.subpackets.as_ref() {
        for sp in v {
            local_v += recurse_sum(sp);
        }
    }
    local_v
}

pub fn compute_val(p: &Packet) -> usize {
    if let Some(v) = p.value.as_ref() {
        return *v;
    }
    match p.packet_type {
        0 => p
            .subpackets
            .as_ref()
            .map(|z| z.iter().map(|x| compute_val(&x)).sum())
            .unwrap_or(0),
        1 => p
            .subpackets
            .as_ref()
            .map(|z| z.iter().map(|x| compute_val(&x)).fold(1, |acc, v| acc * v))
            .unwrap_or(0),
        2 => p
            .subpackets
            .as_ref()
            .map(|z| {
                z.iter()
                    .map(|x| compute_val(&x))
                    .reduce(|a, b| if a < b { a } else { b })
                    .unwrap_or(0)
            })
            .unwrap_or(0),
        3 => p
            .subpackets
            .as_ref()
            .map(|z| {
                z.iter()
                    .map(|x| compute_val(&x))
                    .reduce(|a, b| if a > b { a } else { b })
                    .unwrap_or(0)
            })
            .unwrap_or(0),
        5 => p
            .subpackets
            .as_ref()
            .map(|z| {
                z.iter()
                    .map(|x| compute_val(&x))
                    .reduce(|a, b| if a > b { 1 } else { 0 })
                    .unwrap_or(0)
            })
            .unwrap_or(0),
        6 => p
            .subpackets
            .as_ref()
            .map(|z| {
                z.iter()
                    .map(|x| compute_val(&x))
                    .reduce(|a, b| if a < b { 1 } else { 0 })
                    .unwrap_or(0)
            })
            .unwrap_or(0),
        7 => p
            .subpackets
            .as_ref()
            .map(|z| {
                z.iter()
                    .map(|x| compute_val(&x))
                    .reduce(|a, b| if a == b { 1 } else { 0 })
                    .unwrap_or(0)
            })
            .unwrap_or(0),
        _ => panic!("Unexpected type"),
    }
}

pub fn parse_packet(input: &[bool], root_packet: bool) -> (Packet, usize, &[bool]) {
    let mut index: usize = 0;

    let version = from_bits(&input[index..index + 3]);
    index += 3;
    let packet_type = from_bits(&input[index..index + 3]);
    index += 3;
    if packet_type == 4 {
        // Literal value
        let mut value: usize = 0;
        loop {
            let bits = &input[index..index + 5];
            index += 5;
            value = value << 4;
            value += from_bits(&bits[1..]) as usize;
            if !bits[0] {
                break;
            }
        }
        // if index % 4 != 0 {
        //     index += (4 - (index % 4));
        // }
        // Consume padding if in outer packet
        if root_packet {
            while index < input.len() {
                if !input[index] {
                    index += 1;
                }
            }
        }
        (
            Packet {
                version,
                packet_type,
                value: Some(value),
                length_type: None,
                length_subpackets: None,
                subpackets: None,
            },
            index,
            &input[index..],
        )
    } else {
        let length_type = input[index];
        index += 1;
        let mut subpackets = Vec::new();
        if !length_type {
            let length_subpackets = from_bits_u16(&input[index..index + 15]);
            index += 15;

            let mut bits_parsed = 0;
            let mut packet_input = &input[index..];
            while bits_parsed < length_subpackets as usize {
                let p = parse_packet(&packet_input, false);
                subpackets.push(p.0);
                bits_parsed += p.1;
                packet_input = p.2;
            }
            index += bits_parsed;
            // Consume padding if in outer packet
            if root_packet {
                while index < input.len() {
                    if !input[index] {
                        index += 1;
                    }
                }
            }
            (
                Packet {
                    version,
                    packet_type,
                    value: None,
                    length_type: Some(length_type),
                    length_subpackets: Some(length_subpackets),
                    subpackets: Some(subpackets),
                },
                index,
                &input[index..],
            )
        } else {
            // num packets
            let length_subpackets = from_bits_u16(&input[index..index + 11]);
            index += 11;
            let mut subpackets = Vec::new();
            let mut bits_parsed = 0;
            let mut packet_input = &input[index..];
            while subpackets.len() < length_subpackets as usize {
                let p = parse_packet(&packet_input, false);
                subpackets.push(p.0);
                bits_parsed += p.1;
                packet_input = p.2;
            }
            index += bits_parsed;
            // Consume padding if in outer packet
            if root_packet {
                while index < input.len() {
                    if !input[index] {
                        index += 1;
                    }
                }
            }
            (
                Packet {
                    version,
                    packet_type,
                    value: None,
                    length_type: Some(length_type),
                    length_subpackets: Some(length_subpackets),
                    subpackets: Some(subpackets),
                },
                index,
                &input[index..],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits() {
        assert_eq!(to_bits(15), vec![true, true, true, true]);
    }
    #[test]
    fn test_from_bits() {
        assert_eq!(from_bits(&[false, false, true, true]), 3);
    }
    #[test]
    fn test_from_bits_u16() {
        let bin_str = "000000000011011";
        assert_eq!(from_bits_u16(&from_bin_str(bin_str)), 27);
    }
    #[test]
    fn test_from_bits2() {
        assert_eq!(from_bits(&[true, false, false]), 4);
    }
    #[test]
    fn test_to_bits2() {
        let hex_str = "D2FE28";
        let bin_str = "110100101111111000101000";
        assert_eq!(input_generator(hex_str), from_bin_str(bin_str));
    }
    #[test]
    fn test_to_bits3() {
        let hex_str = "38006F45291200";
        let bin_str = "00111000000000000110111101000101001010010001001000000000";
        assert_eq!(input_generator(hex_str), from_bin_str(bin_str));
    }
    #[test]
    fn test_one_packet() {
        let hex_str = "D2FE28";
        let empty: &[bool] = &[];
        assert_eq!(
            parse_packet(&input_generator(hex_str), true),
            (
                Packet {
                    version: 6,
                    packet_type: 4,
                    value: Some(2021),
                    length_type: None,
                    length_subpackets: None,
                    subpackets: None
                },
                24,
                empty
            )
        );
    }
    #[test]
    fn test_simple_packets() {
        let hex_str = "38006F45291200";
        let empty: &[bool] = &[];
        assert_eq!(
            parse_packet(&input_generator(hex_str), true),
            (
                Packet {
                    version: 1,
                    packet_type: 6,
                    value: None,
                    length_type: Some(false),
                    length_subpackets: Some(27),
                    subpackets: Some(vec![
                        Packet {
                            version: 6,
                            packet_type: 4,
                            value: Some(10),
                            length_type: None,
                            length_subpackets: None,
                            subpackets: None
                        },
                        Packet {
                            version: 2,
                            packet_type: 4,
                            value: Some(20),
                            length_type: None,
                            length_subpackets: None,
                            subpackets: None
                        },
                    ])
                },
                56,
                empty
            )
        );
    }
    #[test]
    fn test_more_packets() {
        let hex_str = "EE00D40C823060";
        let empty: &[bool] = &[];
        assert_eq!(
            parse_packet(&input_generator(hex_str), true),
            (
                Packet {
                    version: 7,
                    packet_type: 3,
                    value: None,
                    length_type: Some(true),
                    length_subpackets: Some(3),
                    subpackets: Some(vec![
                        Packet {
                            version: 2,
                            packet_type: 4,
                            value: Some(1),
                            length_type: None,
                            length_subpackets: None,
                            subpackets: None
                        },
                        Packet {
                            version: 4,
                            packet_type: 4,
                            value: Some(2),
                            length_type: None,
                            length_subpackets: None,
                            subpackets: None
                        },
                        Packet {
                            version: 1,
                            packet_type: 4,
                            value: Some(3),
                            length_type: None,
                            length_subpackets: None,
                            subpackets: None
                        },
                    ])
                },
                56,
                empty
            )
        );
    }
    #[test]
    fn test_part_1_1() {
        let hex_str = "8A004A801A8002F478";
        assert_eq!(solve_part1(&input_generator(hex_str)), 16);
    }
    #[test]
    fn test_part_2_1() {
        let hex_str = "C200B40A82";
        assert_eq!(solve_part2(&input_generator(hex_str)), 3);
    }
    #[test]
    fn test_part_2_2() {
        let hex_str = "04005AC33890";
        assert_eq!(solve_part2(&input_generator(hex_str)), 54);
    }
}
