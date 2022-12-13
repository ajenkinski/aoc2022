// https://adventofcode.com/2022/day/10

use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Add(isize),
    Noop,
}

fn parse_input(input: &String) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect_vec();
            if parts[0] == "addx" {
                Instruction::Add(parts[1].parse().unwrap())
            } else {
                Instruction::Noop
            }
        })
        .collect_vec()
}

/// Returns a log of the X register values during each clock cycle
fn exec_instructions<'a>(instructions: &'a Vec<Instruction>) -> impl Iterator<Item = isize> + 'a {
    instructions
        .iter()
        .scan(1, |x_value, instruction| {
            let x = *x_value;
            match instruction {
                Instruction::Add(n) => {
                    *x_value += *n;
                    Some(vec![x, x].into_iter())
                }
                Instruction::Noop => Some(vec![x].into_iter()),
            }
        })
        .flatten()
}

fn solve_part1(instructions: &Vec<Instruction>) -> isize {
    exec_instructions(instructions)
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(i, x)| ((i as isize) + 1) * x)
        .sum()
}

fn solve_part2(instructions: &Vec<Instruction>) -> String {
    let width: usize = 40;
    let height: usize = 6;

    let mut pixels = vec![' '; width * height];

    for (cycle, x) in exec_instructions(instructions).enumerate() {
        let draw_col = cycle % width;
        if x.abs_diff(draw_col as isize) <= 1 {
            pixels[cycle] = '#';
        } else {
            pixels[cycle] = '.';
        }
    }

    pixels
        .chunks(width)
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

fn main() {
    let input = read_to_string("input/day10-input.txt").unwrap();
    let instructions = parse_input(&input);

    println!("Part 1 solution = {}", solve_part1(&instructions));
    println!("Part 2 solution = \n{}", solve_part2(&instructions));
}
