// https://adventofcode.com/2022/day/3

use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

fn item_priority(item: char) -> usize {
    if item.is_uppercase() {
        item as usize - 'A' as usize + 27
    } else {
        item as usize - 'a' as usize + 1
    }
}

fn solve_part1(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect_vec();
            assert_eq!(chars.len() % 2, 0);

            let (compartment1, compartment2) = chars.split_at(chars.len() / 2);

            // use hashset intersection to find common element
            let in_both: HashSet<char> = &HashSet::from_iter(compartment1.iter().cloned())
                & &HashSet::from_iter(compartment2.iter().cloned());
            let in_both = *in_both.iter().exactly_one().unwrap();

            item_priority(in_both)
        })
        .sum()
}

fn solve_part2(input: &String) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            // Find the intersection of all lines in group
            let badge = *group
                .map(|line| -> HashSet<char> { HashSet::from_iter(line.chars()) })
                .reduce(|s1, s2| &s1 & &s2)
                .unwrap()
                .iter()
                .exactly_one()
                .unwrap();

            item_priority(badge)
        })
        .sum()
}

fn main() {
    let input = read_to_string("input/day3-input.txt").unwrap();

    println!("Part 1 solution = {}", solve_part1(&input));
    println!("Part 2 solution = {}", solve_part2(&input));
}
