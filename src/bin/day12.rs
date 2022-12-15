// https://adventofcode.com/2022/day/12

use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use petgraph::{
    algo::dijkstra::dijkstra,
    graph::DiGraph,
    visit::{GraphBase, Reversed},
};

use common::Grid;

type HeightMap = DiGraph<usize, ()>;
type NodeId = <HeightMap as GraphBase>::NodeId;

fn cell_height(ch: char) -> usize {
    let ch = match ch {
        'S' => 'a',
        'E' => 'z',
        _ => ch,
    };
    (ch as usize) - ('a' as usize)
}

fn parse_input(input: &String) -> (HeightMap, NodeId, NodeId) {
    let grid = Grid::new(
        input
            .trim()
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    );

    let mut graph = HeightMap::new();

    let mut start_node = NodeId::default();
    let mut end_node = NodeId::default();

    let mut coord_node_map = HashMap::new();

    // need to add nodes before we can add edges
    for coord in grid.all_coords() {
        let node_id = graph.add_node(cell_height(grid[coord]));

        coord_node_map.insert(coord, node_id);

        if grid[coord] == 'S' {
            start_node = node_id;
        } else if grid[coord] == 'E' {
            end_node = node_id;
        }
    }

    // add edges
    for from_coord in grid.all_coords() {
        let from_val = grid[from_coord];
        let from_node = coord_node_map[&from_coord];

        for nc in grid.neighbor_coords(from_coord) {
            if cell_height(from_val) + 1 >= cell_height(grid[nc]) {
                graph.add_edge(from_node, coord_node_map[&nc], ());
            }
        }
    }

    (graph, start_node, end_node)
}

fn solve_part1(graph: &HeightMap, start_node: NodeId, end_node: NodeId) -> usize {
    let res = dijkstra(graph, start_node, Some(end_node), |_| 1);
    res[&end_node]
}

fn solve_part2(graph: &HeightMap, end_node: NodeId) -> usize {
    // find lengths of shortest paths from all nodes to end_node.  I do this by reversing the edges in the graph, and then asking
    // for the shortest paths from the end_node to all other nodes.
    let all_lengths = dijkstra(Reversed(graph), end_node, None, |_| 1usize);

    // find the min length after keeping only lengths starting from nodes with height 0
    all_lengths
        .into_iter()
        .filter_map(|(node_id, len)| if graph[node_id] == 0 { Some(len) } else { None })
        .min()
        .unwrap()
}

fn main() {
    let input = read_to_string("input/day12-input.txt").unwrap();
    let (graph, start_node, end_node) = parse_input(&input);

    println!(
        "Part 1 solution = {}",
        solve_part1(&graph, start_node, end_node)
    );

    println!("Part 2 solution = {}", solve_part2(&graph, end_node));
}
