// https://adventofcode.com/2022/day/6

use std::fs::read_to_string;

use itertools::Itertools;

/// Returns the index after the first sequence of marker_len unique characters
fn solve(input: &[char], marker_len: usize) -> usize {
    input
        .windows(marker_len)
        .enumerate()
        .find_map(|(start, win)| {
            if win.iter().unique().count() == marker_len {
                Some(start + marker_len)
            } else {
                None
            }
        })
        .expect("Failed to find marker")
}

fn main() {
    let input = read_to_string("input/day6-input.txt")
        .unwrap()
        .chars()
        .collect_vec();

    println!("Part 1 solution = {}", solve(&input, 4));
    println!("Part 2 solution = {}", solve(&input, 14));
}
