// https://adventofcode.com/2022/day/2

use std::fs::read_to_string;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Move {
    fn parse(ch: char) -> Option<Move> {
        Move::try_from_i32((ch as i32) - ('A' as i32))
    }

    fn try_from_i32(i: i32) -> Option<Move> {
        match i {
            0 => Some(Move::Rock),
            1 => Some(Move::Paper),
            2 => Some(Move::Scissor),
            _ => None,
        }
    }

    /// Return the move that would beat self
    fn winning_move(&self) -> Move {
        Move::try_from_i32((*self as i32 + 1) % 3).unwrap()
    }

    /// Return the move that would lose to self
    fn losing_move(&self) -> Move {
        Move::try_from_i32((*self as i32 - 1).rem_euclid(3)).unwrap()
    }
}

fn get_round_outcome(opponent_move: Move, my_move: Move) -> Outcome {
    if my_move == opponent_move {
        Outcome::Draw
    } else if my_move == opponent_move.winning_move() {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

fn get_round_score(opponent_move: Move, my_move: Move) -> usize {
    let outcome = get_round_outcome(opponent_move, my_move);

    (my_move as usize) + 1 + (outcome as usize) * 3
}

fn solve_part1(rounds: &[(char, char)]) -> usize {
    let scores = rounds.iter().map(|&(opponent_char, my_char)| {
        let opponent_move = Move::parse(opponent_char).unwrap();

        // for part 1, assume X,Y,Z map to A,B,C
        let my_char = char::from_u32((my_char as u32) - ('X' as u32) + ('A' as u32)).unwrap();
        let my_move = Move::parse(my_char).unwrap();

        get_round_score(opponent_move, my_move)
    });

    scores.sum()
}

fn solve_part2(rounds: &[(char, char)]) -> usize {
    let scores = rounds.iter().map(|&(opponent_char, my_char)| {
        let opponent_move = Move::parse(opponent_char).unwrap();

        // for part 2, assume X means we should lose, Y means we should draw, and Z means we should win
        let my_move = match my_char {
            'X' => opponent_move.losing_move(),
            'Y' => opponent_move,
            'Z' => opponent_move.winning_move(),
            _ => panic!("Unexpected move char '{}'", my_char),
        };

        get_round_score(opponent_move, my_move)
    });

    scores.sum()
}

fn main() {
    let input = read_to_string("input/day2-input.txt").unwrap();
    let rounds: Vec<(char, char)> = input
        .lines()
        .map(|line| {
            let chars = line.chars().collect_vec();
            assert_eq!(chars.len(), 3);
            (chars[0], chars[2])
        })
        .collect_vec();

    println!("Part 1 solution = {}", solve_part1(&rounds));
    println!("Part 2 solution = {}", solve_part2(&rounds));
}
