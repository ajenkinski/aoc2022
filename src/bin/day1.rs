// https://adventofcode.com/2022/day/1

use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let input = read_to_string("input/day1-input.txt").unwrap();

    let items_per_elf: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|elf_input| {
            elf_input
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    // Sum calories per elf, and sort in descending order
    let calories_per_elf: Vec<u32> = items_per_elf
        .iter()
        .map(|items| items.iter().sum::<u32>())
        .sorted_by(|a, b| Ord::cmp(b, a))
        .collect_vec();

    let part1_solution = calories_per_elf[0];
    println!("Part 1 solution = {}", part1_solution);

    let part2_solution = calories_per_elf[0..3].iter().sum::<u32>();
    println!("Part 2 solution = {}", part2_solution);
}
