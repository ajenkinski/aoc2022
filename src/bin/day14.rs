// https://adventofcode.com/2022/day/14

use std::{fs::read_to_string, collections::HashSet};

use itertools::Itertools;

type Coord = (isize, isize);
type Path = Vec<Coord>;

#[derive(Default)]
struct Scan {
    grid: HashSet<Coord>,
    min_row: isize,
    min_col: isize,
    max_row: isize,
    max_col: isize,

    // If set, behaves as if there is an infinite horizontal line at this row
    floor: Option<isize>,
}

impl Scan {
    fn new() -> Scan {
        Scan::default()
    }

    fn is_set(&self, coord: Coord) -> bool {
        self.grid.contains(&coord) || self.floor.map_or(false, |f| coord.0 == f)
    }

    fn set(&mut self, coord: Coord) {
        let (row, col) = coord;
        if self.grid.insert(coord) {
            self.min_row = row.min(self.min_row);
            self.min_col = col.min(self.min_col);
            self.max_row = row.max(self.max_row);
            self.max_col = col.max(self.max_col);
        }
    }

    fn set_floor(&mut self, below_max: isize) {
        self.max_row += below_max;
        self.floor = Some(self.max_row);
    }
}

fn parse_input(input: &String) -> Vec<Path> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point_str| {
                    let (x, y) = point_str.split_once(",").unwrap();
                    (y.parse().unwrap(), x.parse().unwrap())
                })
                .collect()
        })
        .collect()
}

fn create_scan(rock_paths: &Vec<Path>, with_floor: bool) -> Scan {
    let mut scan = Scan::new();

    for path in rock_paths.iter() {
        for (&(row0, col0), &(row1, col1)) in path.iter().tuple_windows() {
            let (from_row, to_row) = (row0.min(row1), row0.max(row1));
            let (from_col, to_col) = (col0.min(col1), col0.max(col1));
            for coord in (from_row..=to_row).cartesian_product(from_col..=to_col) {
                scan.set(coord);
            }
        }
    }

    if with_floor {
        scan.set_floor(2);
    }

    scan
}

/// Simulate dropping a unit of sand.  Returns Some(Some(coord)) if unit came to rest, Some(None) if unit never hit an obstacle, or
/// None if starting location is full
fn simulate_drop(scan: &Scan) -> Option<Option<Coord>> {
    let mut cur_row: isize = 0;
    let mut cur_col: isize = 500;

    if scan.is_set((cur_row, cur_col)) {
        return None
    }

    let row_bounds = 0..=scan.max_row;
    let col_bounds = 0..=scan.max_col;
    let in_bounds = |row: isize, col: isize| {
         let col_in_bounds = scan.floor.is_some() || col_bounds.contains(&col);
         col_in_bounds && row_bounds.contains(&row)
    };

    loop {
        // check down
        let mut moved = false;
        for (next_row, next_col) in [
            (cur_row + 1, cur_col),
            (cur_row + 1, cur_col - 1),
            (cur_row + 1, cur_col + 1),
        ] {
            if !in_bounds(next_row, next_col) {
                return Some(None);
            } else if !scan.is_set((next_row, next_col)) {
                cur_row = next_row;
                cur_col = next_col;
                moved = true;
                break;
            }
        }

        if !moved {
            return Some(Some((cur_row, cur_col)));
        }
    }
}

fn solve_part1(paths: &Vec<Path>) -> usize {
    let mut scan = create_scan(paths, false);

    let mut count: usize = 0;

    while let Some(Some(coord)) = simulate_drop(&scan) {
        count += 1;
        scan.set(coord);
    }

    count
}

fn solve_part2(paths: &Vec<Path>) -> usize {
    let mut scan = create_scan(paths, true);

    let mut count: usize = 0;

    while let Some(loc) = simulate_drop(&scan) {
        let coord = loc.expect(format!("Ran into scan edge at count {}", count).as_str());
        count += 1;
        scan.set(coord);
    }

    count
}

fn main() {
    let input = read_to_string("input/day14-input.txt").unwrap();

    let rock_paths = parse_input(&input);

    println!("Part 1 solution = {}", solve_part1(&rock_paths));
    println!("Part 2 solution = {}", solve_part2(&rock_paths));
}
