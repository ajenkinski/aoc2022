// https://adventofcode.com/2022/day/12

use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::{algo::k_shortest_path, graphmap::DiGraphMap};

use common::{Coord, Grid};

// node labels are grid (row, col) coordinates
type HeightMap = DiGraphMap<Coord, ()>;

fn cell_height(ch: char) -> usize {
    let ch = match ch {
        'S' => 'a',
        'E' => 'z',
        _ => ch,
    };
    (ch as usize) - ('a' as usize)
}

fn parse_input(input: &String) -> (HeightMap, Coord, Coord) {
    let grid = Grid::new(
        input
            .trim()
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    );

    let mut graph = HeightMap::new();

    let mut start_coord: Coord = (0, 0);
    let mut end_coord: Coord = (0, 0);

    for coord in grid.all_coords() {
        let this_val = grid[coord];

        if this_val == 'S' {
            start_coord = coord;
        } else if this_val == 'E' {
            end_coord = coord;
        }

        for nc in grid.neighbor_coords(coord) {
            if cell_height(this_val) + 1 >= cell_height(grid[nc]) {
                graph.add_edge(coord, nc, ());
            }
        }
    }

    (graph, start_coord, end_coord)
}

fn solve_part1(graph: &HeightMap, start_coord: Coord, end_coord: Coord) -> usize {
    let res = k_shortest_path(graph, start_coord, Some(end_coord), 1, |_| 1);
    res[&end_coord]
}

fn main() {
    let input = read_to_string("input/day12-input.txt").unwrap();
    let (graph, start_coord, end_coord) = parse_input(&input);

    println!(
        "Part 1 solution = {}",
        solve_part1(&graph, start_coord, end_coord)
    );
}
