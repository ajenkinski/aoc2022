// https://adventofcode.com/2022/day/5

use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Debug)]
struct Step {
    num_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

#[derive(Debug)]
struct Problem {
    stacks: Vec<Vec<char>>,
    steps: Vec<Step>,
}

fn parse_input(input: &String) -> Problem {
    let (stacks_str, steps_str) = input.split("\n\n").collect_tuple().unwrap();

    let stacks_lines = stacks_str.lines().collect_vec();

    // last line of stacks section is the stack numbers
    let num_stacks = stacks_lines[stacks_lines.len() - 1]
        .split_ascii_whitespace()
        .count();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];
    for line in stacks_lines[0..stacks_lines.len() - 1].into_iter() {
        // each line looks like "[T]     [H]     [V] [Q]         [H]"
        for (stack_num, crate_char) in line.chars().skip(1).step_by(4).enumerate() {
            if crate_char != ' ' {
                stacks[stack_num].push(crate_char);
            }
        }
    }

    // we want top of stack at end of vecs, so we can use push and pop
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let steps = steps_str
        .lines()
        .map(|line| {
            // each line looks like "move 1 from 8 to 7"
            let parts = line.split_ascii_whitespace().collect_vec();

            Step {
                num_to_move: parts[1].parse::<usize>().unwrap(),
                from_stack: parts[3].parse::<usize>().unwrap() - 1,
                to_stack: parts[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect_vec();

    Problem { stacks, steps }
}

fn solve_part1(problem: &Problem) -> String {
    let mut stacks = problem.stacks.clone();

    for step in problem.steps.iter() {
        for _ in 0..step.num_to_move {
            let tmp = stacks[step.from_stack].pop().unwrap();
            stacks[step.to_stack].push(tmp);
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect()
}

fn solve_part2(problem: &Problem) -> String {
    let mut stacks = problem.stacks.clone();

    for step in problem.steps.iter() {
        let from_stack = &mut stacks[step.from_stack];
        let tmp = from_stack.split_off(from_stack.len() - step.num_to_move);
        stacks[step.to_stack].extend_from_slice(&tmp);
    }

    stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect()
}

fn main() {
    let input = read_to_string("input/day5-input.txt").unwrap();

    let problem = parse_input(&input);
    println!("Part 1 solution = {}", solve_part1(&problem));
    println!("Part 2 solution = {}", solve_part2(&problem));
}
