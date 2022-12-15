// https://adventofcode.com/2022/day/14

use std::fs::read_to_string;

use common::{Coord, Grid};
use itertools::Itertools;

struct Point {
    x: usize,
    y: usize,
}

type Path = Vec<Point>;

type Scan = Grid<bool>;

fn parse_input(input: &String) -> Vec<Path> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point_str| {
                    let (x, y) = point_str.split_once(",").unwrap();
                    Point {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect()
}

fn create_scan(rock_paths: &Vec<Path>) -> Scan {
    let (x_max, y_max) = rock_paths
        .iter()
        .flatten()
        .fold((0, 0), |(mx, my), point| (mx.max(point.x), my.max(point.y)));

    let mut scan = Scan::new(vec![vec![false; x_max + 1]; y_max + 1]);

    for path in rock_paths.iter() {
        for line in path.windows(2) {
            let (from_row, to_row) = (line[0].y.min(line[1].y), line[0].y.max(line[1].y));
            let (from_col, to_col) = (line[0].x.min(line[1].x), line[0].x.max(line[1].x));
            for coord in (from_row..=to_row).cartesian_product(from_col..=to_col) {
                scan[coord] = true;
            }
        }
    }

    scan
}

/// Simulate dropping a unit of sand.  Returns the final resting point of the unit, or None if the sand fell into the abyss.
/// Panics if there's no room for more sand
fn simulate_drop(scan: &Scan) -> Option<Coord> {
    let mut cur_row: isize = 0;
    let mut cur_col: isize = 500;

    assert!(!scan[(cur_row as usize, cur_col as usize)]);

    let row_bounds = 0..(scan.num_rows() as isize);
    let col_bounds = 0..(scan.num_cols() as isize);
    let in_bounds = |row: isize, col: isize| row_bounds.contains(&row) && col_bounds.contains(&col);

    loop {
        // check down
        let mut moved = false;
        for (next_row, next_col) in [
            (cur_row + 1, cur_col),
            (cur_row + 1, cur_col - 1),
            (cur_row + 1, cur_col + 1),
        ] {
            if !in_bounds(next_row, next_col) {
                return None;
            } else if !scan[(next_row as usize, next_col as usize)] {
                cur_row = next_row;
                cur_col = next_col;
                moved = true;
                break;
            }
        }

        if !moved {
            return Some((cur_row as usize, cur_col as usize));
        }
    }
}

fn solve_part1(scan: &Scan) -> usize {
    let mut scan = scan.clone();

    let mut count: usize = 0;

    while let Some(coord) = simulate_drop(&scan) {
        count += 1;
        scan[coord] = true;
    }

    count
}

fn main() {
    let input = read_to_string("input/day14-input.txt").unwrap();
    let test_input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
        .to_string();

    let rock_paths = parse_input(&input);
    let scan = create_scan(&rock_paths);

    println!("Part 1 solution = {}", solve_part1(&scan));
}
