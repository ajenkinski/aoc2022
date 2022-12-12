// https://adventofcode.com/2022/day/9

use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Instruction {
    count: usize, // number of times to repeat
    dr: isize,    // row delta
    dc: isize,    // column delta
}

fn parse_input(input: &String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_once(" ").unwrap();
            let count = n.parse::<usize>().unwrap();
            match dir {
                "U" => Instruction {
                    count,
                    dr: -1,
                    dc: 0,
                },
                "D" => Instruction {
                    count,
                    dr: 1,
                    dc: 0,
                },
                "L" => Instruction {
                    count,
                    dr: 0,
                    dc: -1,
                },
                "R" => Instruction {
                    count,
                    dr: 0,
                    dc: 1,
                },
                _ => panic!("Unrecognized direction: {:?}", dir),
            }
        })
        .collect()
}

fn solve_part1(instructions: &Vec<Instruction>) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((0, 0));

    let mut head_row: isize = 0;
    let mut head_col: isize = 0;
    let mut tail_row: isize = 0;
    let mut tail_col: isize = 0;

    for instruction in instructions.iter() {
        for _ in 0..instruction.count {
            head_row += instruction.dr;
            head_col += instruction.dc;
            if head_row.abs_diff(tail_row) > 1 || head_col.abs_diff(tail_col) > 1 {
                tail_row += (head_row - tail_row).signum();
                tail_col += (head_col - tail_col).signum();
            }
            visited.insert((tail_row, tail_col));
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
R 2
    ".trim().to_string();

    let instructions = parse_input(&input);
    assert_eq!(solve_part1(&instructions), 13);    
    }
}

fn main() {
    let input = read_to_string("input/day9-input.txt").unwrap();

    let instructions = parse_input(&input);

    println!("Part 1 solution = {}", solve_part1(&instructions));
}
