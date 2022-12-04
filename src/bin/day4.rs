// https://adventofcode.com/2022/day/4

use std::fs::read_to_string;

use itertools::Itertools;

fn parse_input(input: &String) -> Vec<((usize, usize), (usize, usize))> {
    // each line looks like "2-4,6-8"
    input
        .lines()
        .map(|line| {
            let nums = line
                .split(&['-', ','])
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect_vec();
            assert_eq!(nums.len(), 4);
            ((nums[0], nums[1]), (nums[2], nums[3]))
        })
        .collect_vec()
}

fn main() {
    let input = read_to_string("input/day4-input.txt").unwrap();
    let range_pairs = parse_input(&input);

    // Count how many ranges are completely contained in their companion range
    let part1_result = range_pairs
        .iter()
        .filter(|((start1, end1), (start2, end2))| {
            (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1)
        })
        .count();

    println!("Part 1 solution = {}", part1_result);

    // Count how many ranges overlap their companion range
    let part2_result = range_pairs
        .iter()
        .filter(|((start1, end1), (start2, end2))| {
            (start1..=end1).contains(&start2) || (start2..=end2).contains(&start1)
        })
        .count();

    println!("Part 2 solution = {}", part2_result);
}
