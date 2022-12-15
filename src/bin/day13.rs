// https://adventofcode.com/2022/day/13

use std::{fs::read_to_string, iter::Peekable};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Elem {
    Num(usize),
    List(Vec<Elem>),
}

type Packet = Vec<Elem>;

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l == r,
            (Self::List(l), Self::List(r)) => l == r,
            (Self::List(l), Self::Num(_)) => l.len() == 1 && l[0] == *other,
            (Self::Num(_), Self::List(r)) => r.len() == 1 && *self == r[0],
        }
    }
}

impl Eq for Elem {}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l.partial_cmp(r),
            (Self::List(l), Self::List(r)) => l.partial_cmp(r),
            (Self::List(l), Self::Num(_)) => l.partial_cmp(&vec![other.clone()]),
            (Self::Num(_), Self::List(r)) => vec![self.clone()].partial_cmp(r),
        }
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_input(input: &String) -> Vec<(Packet, Packet)> {
    input
        .trim()
        .split("\n\n")
        .map(|pair_str| {
            let (str1, str2) = pair_str.split_once("\n").unwrap();
            (parse_packet(str1), parse_packet(str2))
        })
        .collect()
}

fn parse_packet(input: &str) -> Packet {
    let mut chars = input.chars().peekable();
    match parse_list(&mut chars) {
        Elem::List(packet) => packet,
        _ => unreachable!(),
    }
}

fn parse_list<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Elem {
    assert_eq!(chars.next(), Some('['));

    let mut elems = vec![];

    let mut found_close = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            ']' => {
                chars.next();
                found_close = true;
                break;
            }
            ',' => {
                chars.next();
            }
            '[' => {
                elems.push(parse_list(chars));
            }
            '0'..='9' => {
                elems.push(parse_num(chars));
            }
            _ => {
                panic!("Unexpected char: {:?}", ch);
            }
        }
    }

    assert!(found_close, "Unclosed list found");
    Elem::List(elems)
}

fn parse_num<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Elem {
    let digits = chars
        .peeking_take_while(char::is_ascii_digit)
        .collect::<String>();
    Elem::Num(digits.parse().unwrap())
}

fn solve_part1(pairs: &Vec<(Packet, Packet)>) -> usize {
    // sum of one-based indexes of pairs that are correctly ordered
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1 <= p2)
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_part2(pairs: &Vec<(Packet, Packet)>) -> usize {
    let mut packets = pairs
        .iter()
        .flat_map(|(p1, p2)| [p1.clone(), p2.clone()].into_iter())
        .collect_vec();

    let extra1 = vec![Elem::List(vec![Elem::Num(2)])];
    let extra2 = vec![Elem::List(vec![Elem::Num(6)])];

    packets.push(extra1.clone());
    packets.push(extra2.clone());

    packets.sort();

    (packets.iter().position(|e| *e == extra1).unwrap() + 1)
        * (packets.iter().position(|e| *e == extra2).unwrap() + 1)
}

fn main() {
    let input = read_to_string("input/day13-input.txt").unwrap();
    let packet_pairs = parse_input(&input);

    println!("Part 1 solution = {}", solve_part1(&packet_pairs));
    println!("Part 2 solution = {}", solve_part2(&packet_pairs));
}
