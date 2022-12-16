// https://adventofcode.com/2022/day/14

use std::fs::read_to_string;

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

fn parse_input(input: &String) -> Vec<Sensor> {
    let sensor_rx = Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();

    input
        .trim()
        .lines()
        .map(|line| {
            let caps = sensor_rx
                .captures(line)
                .expect(format!("Match failed for line {line:?}").as_str());

            let location = Point {
                x: caps["sx"].parse().unwrap(),
                y: caps["sy"].parse().unwrap(),
            };

            let nearest_beacon = Point {
                x: caps["bx"].parse().unwrap(),
                y: caps["by"].parse().unwrap(),
            };

            Sensor {
                location,
                nearest_beacon,
            }
        })
        .collect()
}

fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

fn main() {
    let input = read_to_string("input/day15-input.txt").unwrap();
    let sensors = parse_input(&input);

    for s in sensors.iter() {
        println!("{s:#?}");
    }

    let sensor = &sensors[6];
    println!(
        "Distance for {:#?} is {}",
        sensor,
        manhattan_distance(&sensor.location, &sensor.nearest_beacon)
    );
}
