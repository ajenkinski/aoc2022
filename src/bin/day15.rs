// https://adventofcode.com/2022/day/14

use std::fs::read_to_string;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    nearest_beacon: Point,
}

fn parse_input(input: &String) -> Result<Vec<Sensor>> {
    let sensor_rx = Regex::new(
        r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)",
    )?;

    input
        .trim()
        .lines()
        .map(|line| -> Result<Sensor> {
            let caps = sensor_rx
                .captures(line)
                .ok_or(anyhow!("Match failed for line {line:?}"))?;

            let location = Point {
                x: caps["sx"].parse()?,
                y: caps["sy"].parse()?,
            };

            let nearest_beacon = Point {
                x: caps["bx"].parse()?,
                y: caps["by"].parse()?,
            };

            Ok(Sensor {
                location,
                nearest_beacon,
            })
        })
        .collect()
}

fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

fn solve_part1(sensors: &Vec<Sensor>, target_row: isize) -> usize {
    // ranges that overlap target_row
    let mut x_ranges: Vec<(isize, isize)> = vec![];

    for sensor in sensors.iter() {
        let dist = manhattan_distance(&sensor.location, &sensor.nearest_beacon) as isize;

        if !((sensor.location.y - dist)..=(sensor.location.y + dist)).contains(&target_row)
            || (sensor.nearest_beacon.y.abs_diff(target_row) as isize == dist)
        {
            continue;
        }

        // calculate the range of row target_row in sensor's area that can't contain a beacon
        let horizontal_dist = dist - (sensor.location.y.abs_diff(target_row) as isize);

        let mut left = sensor.location.x - horizontal_dist;
        let mut right = sensor.location.x + horizontal_dist;

        // account of the case where the beacon is on target row
        if sensor.nearest_beacon.x == left {
            left += 1;
        } else if sensor.nearest_beacon.x == right {
            right -= 1;
        }

        x_ranges.push((left, right));
    }

    // now calculate how many unique elements of target_row are covered by x_ranges

    // first merge overlapping ranges
    let merged_ranges = x_ranges.into_iter().sorted_by_key(|r| r.0).fold(
        vec![],
        |mut merged, range @ (left, right)| {
            if let Some((_, prev_right)) = merged.last_mut() {
                if *prev_right >= left {
                    *prev_right = right.max(*prev_right);
                } else {
                    merged.push(range);
                }
            } else {
                merged.push(range);
            }
            merged
        },
    );

    merged_ranges
        .into_iter()
        .map(|(left, right)| (right - left + 1) as usize)
        .sum()
}

fn main() {
    let input = read_to_string("input/day15-input.txt").unwrap();
    let sensors = parse_input(&input).unwrap();

    println!("Part 1 solution = {}", solve_part1(&sensors, 2000000));
}
