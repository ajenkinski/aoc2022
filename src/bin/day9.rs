// https://adventofcode.com/2022/day/9

use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Instruction {
    count: usize, // number of times to repeat
    dr: isize,    // row delta
    dc: isize,    // column delta
}

#[derive(Clone, Copy)]
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
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((0, 0));

    for instruction in instructions.iter() {
        for _ in 0..instruction.count {
            rope[0].row += instruction.dr;
            rope[0].col += instruction.dc;

            for knot in 1..rope.len() {
                // This is needed to convince the compiler to allow two refs into the rope vec
                let (heads, tails) = rope.split_at_mut(knot);
                let prev_knot = &heads[heads.len() - 1];
                let cur_knot = &mut tails[0];

                if prev_knot.row.abs_diff(cur_knot.row) > 1
                    || prev_knot.col.abs_diff(cur_knot.col) > 1
                {
                    cur_knot.row += (prev_knot.row - cur_knot.row).signum();
                    cur_knot.col += (prev_knot.col - cur_knot.col).signum();
                }
            }

            visited.insert((rope[num_knots - 1].row, rope[num_knots - 1].col));
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
