// https://adventofcode.com/2022/day/9

use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Instruction {
    count: usize, // number of times to repeat
    dr: isize,    // row delta
    dc: isize,    // column delta
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

fn parse_input(input: &String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_once(" ").unwrap();
            let count = n.parse::<usize>().unwrap();
            let (dr, dc) = match dir {
                "U" => (-1, 0),
                "D" => (1, 0),
                "L" => (0, -1),
                "R" => (0, 1),
                _ => panic!("Unrecognized direction: {:?}", dir),
            };
            Instruction { count, dr, dc }
        })
        .collect()
}

fn solve(instructions: &Vec<Instruction>, num_knots: usize) -> usize {
    assert!(num_knots >= 2);
    let mut rope = vec![Coord { row: 0, col: 0 }; num_knots];
    let mut visited: HashSet<Coord> = HashSet::new();

    for instruction in instructions.iter() {
        for _ in 0..instruction.count {
            rope[0].row += instruction.dr;
            rope[0].col += instruction.dc;

            for knot in 1..rope.len() {
                if rope[knot - 1].row.abs_diff(rope[knot].row) > 1
                    || rope[knot - 1].col.abs_diff(rope[knot].col) > 1
                {
                    rope[knot].row += (rope[knot - 1].row - rope[knot].row).signum();
                    rope[knot].col += (rope[knot - 1].col - rope[knot].col).signum();
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .trim()
        .to_string();

        let instructions = parse_input(&input);
        assert_eq!(solve(&instructions, 2), 13);
    }

    #[test]
    fn test_part2() {
        let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .trim()
            .to_string();

        let instructions = parse_input(&input);
        assert_eq!(solve(&instructions, 10), 36);
    }
}

fn main() {
    let input = read_to_string("input/day9-input.txt").unwrap();

    let instructions = parse_input(&input);

    println!("Part 1 solution = {}", solve(&instructions, 2));
    println!("Part 2 solution = {}", solve(&instructions, 10));
}
