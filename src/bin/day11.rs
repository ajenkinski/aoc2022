// https://adventofcode.com/2022/day/11

use std::fs::read_to_string;

use itertools::Itertools;
use regex::Regex;

struct Monkey {
    items: Vec<usize>,
    operator: char,
    operand: Option<usize>,
    test_divisor: usize,
    if_true: usize,
    if_false: usize,
}

fn parse_input(input: &String) -> Vec<Monkey> {
    /* Each input looks like
    Monkey 0:
        Starting items: 99, 63, 76, 93, 54, 73
        Operation: new = old * 11
        Test: divisible by 2
            If true: throw to monkey 7
            If false: throw to monkey 1
     */

    let monkey_rx = Regex::new(
        r"
Monkey \d+:
\s*Starting items: (?P<items>\d+(?:, \d+)*)
\s*Operation: new = old (?P<operator>[*+]) (?P<operand>\d+|old)
\s*Test: divisible by (?P<divisor>\d+)
\s*If true: throw to monkey (?P<if_true>\d+)
\s*If false: throw to monkey (?P<if_false>\d+)"
            .trim(),
    )
    .unwrap();

    input
        .trim()
        .split("\n\n")
        .map(|monkey_str| {
            let caps = monkey_rx.captures(monkey_str).unwrap();

            let items = caps["items"]
                .split(", ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();

            let operand = caps["operand"].parse::<usize>().ok();
            let operator = caps["operator"].chars().next().unwrap();
            let test_divisor = caps["divisor"].parse().unwrap();
            let if_true = caps["if_true"].parse().unwrap();
            let if_false = caps["if_false"].parse().unwrap();

            Monkey {
                items,
                operator,
                operand,
                test_divisor,
                if_true,
                if_false,
            }
        })
        .collect()
}

fn solve(monkeys: &Vec<Monkey>, num_rounds: usize, relief_factor: usize) -> usize {
    // I can mod the levels by the product of the test divisors to keep them from growing too large, since I only 
    // care about the remainders to implement the algorithm
    let mod_factor: usize = monkeys.iter().map(|m| m.test_divisor).product();

    let mut all_items = monkeys.iter().map(|m| m.items.clone()).collect_vec();
    let mut counts = vec![0usize; monkeys.len()];

    for _ in 0..num_rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            let items = std::mem::take(&mut all_items[i]);

            counts[i] += items.len();

            for worry_level in items {
                let operand = monkey.operand.unwrap_or(worry_level);
                let new_level = match monkey.operator {
                    '+' => worry_level + operand,
                    '*' => worry_level * operand,
                    _ => unreachable!(),
                } / relief_factor;

                let new_level = new_level % mod_factor;

                if new_level % monkey.test_divisor == 0 {
                    all_items[monkey.if_true].push(new_level);
                } else {
                    all_items[monkey.if_false].push(new_level);
                }
            }
        }
    }

    counts.sort();
    counts[counts.len() - 2..counts.len()].iter().product()
}

fn main() {
    let input = read_to_string("input/day11-input.txt").unwrap();
    let monkeys = parse_input(&input);

    println!("Part 1 solution = {}", solve(&monkeys, 20, 3));
    println!("Part 2 solution = {}", solve(&monkeys, 10000, 1));
}
