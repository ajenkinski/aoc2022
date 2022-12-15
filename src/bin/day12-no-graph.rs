// https://adventofcode.com/2022/day/12

// Alternate solution that doesn't use a graph library

use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

use common::{Coord, Grid};

type HeightMap = Grid<usize>;

fn cell_height(ch: char) -> usize {
    let ch = match ch {
        'S' => 'a',
        'E' => 'z',
        _ => ch,
    };
    (ch as usize) - ('a' as usize)
}

fn parse_input(input: &String) -> (HeightMap, Coord, Coord) {
    let mut start_coord: Coord = (0, 0);
    let mut end_coord: Coord = (0, 0);

    let map = Grid::new(
        input
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, ch)| {
                        if ch == 'S' {
                            start_coord = (row, col);
                        } else if ch == 'E' {
                            end_coord = (row, col);
                        }
                        cell_height(ch)
                    })
                    .collect()
            })
            .collect(),
    );

    (map, start_coord, end_coord)
}

/// Finds shortest paths from a start coordinate to all other coordinates in map, 
/// using a breadth-first-traversal.
/// If goal is not None, then stops when it finds a path to the goal coordinate.
/// The get_neighbors function determines where the traversal can go from a given coordinate.
/// Returns a map of destination coord to shortest path to that coord from start.
fn find_shortest_paths<F>(
    map: &HeightMap,
    start: Coord,
    goal: Option<Coord>,
    mut get_neighbors: F,
) -> HashMap<Coord, usize>
where
    F: FnMut(&HeightMap, Coord) -> Vec<Coord>,
{
    // (path_length, coord) pairs
    let mut to_visit: VecDeque<(usize, Coord)> = VecDeque::new();
    to_visit.push_back((0, start));

    let mut path_lens: HashMap<Coord, usize> = HashMap::new();

    while let Some((len, coord)) = to_visit.pop_front() {
        // avoid loops
        if path_lens.contains_key(&coord) {
            continue;
        }

        if len > 0 {
            path_lens.insert(coord, len);
        }

        if goal.map_or(false, |c| c == coord) {
            break;
        }

        for neighbor in get_neighbors(map, coord) {
            to_visit.push_back((len + 1, neighbor));
        }
    }

    path_lens
}

fn solve_part1(map: &HeightMap, start: Coord, end: Coord) -> usize {
    // traverse from start until end
    let lens = find_shortest_paths(map, start, Some(end), |map, coord| {
        // neighbors are adjacent cells that are at most 1 higher than this cell
        let my_height = map[coord];
        map.neighbor_coords(coord)
            .filter(|nc| my_height + 1 >= map[*nc])
            .collect()
    });

    lens[&end]
}

fn solve_part2(map: &HeightMap, end: Coord) -> usize {
    // traverse backward from end to all cells
    let all_lengths = find_shortest_paths(map, end, None, |map, coord| {
        // neighbors are adjacent cells that are at most 1 lower than this cell
        let my_height = map[coord];
        map.neighbor_coords(coord)
            .filter(|nc| map[*nc] + 1 >= my_height)
            .collect()
    });

    // find the min length after keeping only lengths starting from nodes with height 0
    all_lengths
        .into_iter()
        .filter_map(|(coord, len)| if map[coord] == 0 { Some(len) } else { None })
        .min()
        .unwrap()
}

fn main() {
    let input = read_to_string("input/day12-input.txt").unwrap();
    let (map, start_node, end_node) = parse_input(&input);

    println!(
        "Part 1 solution = {}",
        solve_part1(&map, start_node, end_node)
    );

    println!("Part 2 solution = {}", solve_part2(&map, end_node));
}
