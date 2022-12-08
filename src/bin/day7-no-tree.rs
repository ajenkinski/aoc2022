/*
https://adventofcode.com/2022/day/7

My first solution to day 7 used a tree, because I wanted to try implementing a tree in Rust.  It occurred
to me though that I could also solve it by building up the directory sizes as I parse the file, without
ever constructing an explicit tree.  The only thing the problem ends up needing is the list of sizes, 
and total used space.
 */

use std::{collections::HashMap, fs::read_to_string};


fn parse_input(input: &String) -> (usize, Vec<usize>) {
    let mut dir_sizes: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut cur_dir: Vec<&str> = vec![];

    // A section is a command, and its output if any
    for section in input.split("$ ").skip(1) {
        let mut lines = section.trim().lines();
        let cmd = lines.next().unwrap().trim();
        if cmd.starts_with("cd ") {
            match &cmd[3..] {
                "/" => {
                    cur_dir = vec!["/"];
                }
                ".." => {
                    cur_dir.pop();
                }
                to_dir => {
                    cur_dir.push(to_dir);
                }
            }
        } else if cmd == "ls" {
            let own_size = lines
                .filter_map(|line| {
                    // only care about file lines
                    let (maybe_size, _) = line.split_once(" ").expect("Expected two parts");
                    maybe_size.parse::<usize>().ok()
                })
                .sum::<usize>();

            for i in 1..=cur_dir.len() {
                dir_sizes
                    .entry(cur_dir[0..i].to_vec())
                    .and_modify(|size| *size += own_size)
                    .or_insert(own_size);
            }
        } else {
            panic!("Unknown command: '{}'", cmd);
        }
    }

    let root_size = *dir_sizes.get(&vec!["/"]).unwrap();
    (root_size, dir_sizes.into_values().collect())
}

fn main() {
    let input = read_to_string("input/day7-input.txt").unwrap();
    let (total_used, dir_sizes) = parse_input(&input);

    let part1_solution = dir_sizes.iter().filter(|&s| *s <= 100_000).sum::<usize>();
    println!("Part 1 solution = {}", part1_solution);

    // part 2
    let filesystem_size: usize = 70_000_000;
    let required_free_space: usize = 30_000_000;
    let current_free = filesystem_size - total_used;
    assert!(current_free < required_free_space);
    let need_to_free = required_free_space - current_free;

    // find smallest size at least need_to_free
    let part2_solution = *dir_sizes
        .iter()
        .filter(|&s| *s >= need_to_free)
        .min()
        .expect("Didn't find any directories to delete");

    println!("Part 2 solution = {}", part2_solution);
}
