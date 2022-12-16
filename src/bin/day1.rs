// https://adventofcode.com/2022/day/1

use std::{fs::read_to_string, cmp::Reverse};

use itertools::Itertools;

fn main() {
    let input = read_to_string("input/day1-input.txt").unwrap();

    // Sum calories per elf, and sort in descending order
    let calories_per_elf = input
        .split("\n\n")
        .map(|elf_input| {
            elf_input
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by_key(|&s| Reverse(s))
        .collect_vec();

    let part1_solution = calories_per_elf[0];
    println!("Part 1 solution = {part1_solution}");

    let part2_solution = calories_per_elf[0..3].iter().sum::<u32>();
    println!("Part 2 solution = {part2_solution}");
}
