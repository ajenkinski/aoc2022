// https://adventofcode.com/2022/day/8

use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;
use take_until::TakeUntilExt;

type Grid = common::Grid<char>;

fn parse_input(input: &String) -> Grid {
    Grid::new(input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn solve_part1(grid: &Grid) -> usize {
    let grid_width = grid.num_cols();
    let grid_height = grid.num_rows();

    let mut visible_coords = HashSet::new();

    // coords visible from top or bottom
    for col in 1..(grid_width - 1) {
        // from top
        let mut max = grid[0][col];
        for row in 1..(grid_height - 1) {
            if grid[row][col] > max {
                visible_coords.insert((row, col));
            }
            max = max.max(grid[row][col]);
        }

        // from bottom
        max = grid[grid_height - 1][col];
        for row in (1..(grid_height - 1)).rev() {
            if grid[row][col] > max {
                visible_coords.insert((row, col));
            }
            max = max.max(grid[row][col]);
        }
    }

    // coords visible from left or right
    for row in 1..(grid_height - 1) {
        // from left
        let mut max = grid[row][0];
        for col in 1..(grid_width - 1) {
            if grid[row][col] > max {
                visible_coords.insert((row, col));
            }
            max = max.max(grid[row][col]);
        }

        // from right
        max = grid[row][grid_width - 1];
        for col in (1..(grid_width - 1)).rev() {
            if grid[row][col] > max {
                visible_coords.insert((row, col));
            }
            max = max.max(grid[row][col]);
        }
    }

    visible_coords.len() + grid_width * 2 + (grid_height - 2) * 2
}

/// Return all paths radiating out from (row, col) to the edges
fn get_paths_from(grid: &Grid, row: usize, col: usize) -> [Vec<(usize, usize)>; 4] {
    let width = grid.num_cols();
    let height = grid.num_rows();

    [
        ((col + 1)..width).map(|c| (row, c)).collect(), // to right
        (0..col).rev().map(|c| (row, c)).collect(),     // to left
        (0..row).rev().map(|r| (r, col)).collect(),     // up
        ((row + 1)..height).map(|r| (r, col)).collect(), // down
    ]
}

fn find_scenic_score(grid: &Grid, row: usize, col: usize) -> usize {
    let my_height = grid[row][col];
    get_paths_from(grid, row, col)
        .into_iter()
        .map(|coords| {
            coords
                .into_iter()
                .take_until(|&(r, c)| grid[r][c] >= my_height)
                .count()
        })
        .product()
}

fn solve_part2(grid: &Grid) -> usize {
    (1..grid.num_rows() - 1)
        .cartesian_product(1..grid.num_cols() - 1)
        .map(|(row, col)| find_scenic_score(grid, row, col))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_scenic_score() {
        let input = "
30373
25512
65332
33549
35390"
            .trim()
            .to_string();

        let grid = parse_input(&input);

        assert_eq!(find_scenic_score(&grid, 3, 2), 8);
    }

    #[test]
    fn test_part2() {
        let input = "
30373
25512
65332
33549
35390"
            .trim()
            .to_string();

        let grid = parse_input(&input);

        assert_eq!(solve_part2(&grid), 8);
    }
}

fn main() {
    let input = read_to_string("input/day8-input.txt").unwrap();
    let grid = parse_input(&input);

    println!("Part 1 solution = {}", solve_part1(&grid));
    println!("Part 1 solution = {}", solve_part2(&grid));
}
