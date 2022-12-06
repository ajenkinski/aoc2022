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

// alternate implementation using a bitfield to keep track of unique characters.  I found this technique in the
// reddit solution thread and liked it. Here's the source: https://github.com/mkeeter/advent-of-code/blob/master/2022/06/src/main.rs
fn solve_masks(input: &[char], marker_len: usize) -> usize {
    // mask will contain a 1 bit for every unique character in the last marker_len characters
    let mut mask: usize = 0;
    for i in 0..marker_len {
        mask ^= 1 << input[i] as usize - 'a' as usize;
    }

    for i in marker_len..input.len() {
        if mask.count_ones() as usize == marker_len {
            return i;
        }
        mask ^= 1 << input[i] as usize - 'a' as usize;
        mask ^= 1 << input[i - marker_len] as usize - 'a' as usize;
    }

    panic!("Failed to find marker");
}

fn main() {
    let input = read_to_string("input/day6-input.txt")
        .unwrap()
        .trim()
        .chars()
        .collect_vec();

    println!("Part 1 solution = {}", solve(&input, 4));
    println!("Part 2 solution = {}", solve(&input, 14));

    assert_eq!(solve(&input, 4), solve_masks(&input, 4));
    assert_eq!(solve(&input, 14), solve_masks(&input, 14));
}
