// https://adventofcode.com/2022/day/7

use std::{
    cell::RefCell,
    collections::HashMap,
    fs::read_to_string,
    rc::{Rc, Weak},
};

use itertools::Itertools;

struct Dir {
    files: Vec<File>,
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    parent: Option<Weak<RefCell<Dir>>>,
}

struct File {
    size: usize,
}

fn parse_input(input: &String) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir {
        files: vec![],
        dirs: HashMap::new(),
        parent: None,
    }));

    let mut cur_dir: Rc<RefCell<Dir>> = root.clone();

    for line in input.lines() {
        let parts = line.split_whitespace().collect_vec();
        if line.starts_with("$ cd") {
            let to_dir = parts[2];
            if to_dir == ".." {
                let tmp = cur_dir
                    .borrow()
                    .parent
                    .as_ref()
                    .expect("Tried to cd above root")
                    .upgrade()
                    .unwrap();
                cur_dir = tmp;
            } else if to_dir == "/" {
                cur_dir = root.clone();
            } else {
                let tmp = cur_dir
                    .borrow()
                    .dirs
                    .get(to_dir)
                    .expect(format!("Couldn't cd to to_dir={}", to_dir).as_str())
                    .clone();
                cur_dir = tmp;
            }
        } else if line.starts_with("dir ") {
            let name = parts[1];
            if !cur_dir.borrow().dirs.contains_key(name) {
                cur_dir.borrow_mut().dirs.insert(
                    name.to_string(),
                    Rc::new(RefCell::new(Dir {
                        files: vec![],
                        dirs: HashMap::new(),
                        parent: Some(Rc::downgrade(&cur_dir)),
                    })),
                );
            }
        } else if let Ok(size) = parts[0].parse::<usize>() {
            cur_dir.borrow_mut().files.push(File { size });
        }
    }

    root
}

fn get_dir_sizes(dir: Rc<RefCell<Dir>>, all_sizes: &mut Vec<usize>) -> usize {
    let own_size = dir.borrow().files.iter().map(|f| f.size).sum::<usize>();
    let children_size = dir
        .borrow()
        .dirs
        .values()
        .map(|d| get_dir_sizes(d.clone(), all_sizes))
        .sum::<usize>();

    let total = own_size + children_size;
    all_sizes.push(total);
    total
}

fn main() {
    let input = read_to_string("input/day7-input.txt").unwrap();
    let root = parse_input(&input);

    let mut dir_sizes = vec![];
    let total_used = get_dir_sizes(root.clone(), &mut dir_sizes);

    let part1_solution = dir_sizes.iter().filter(|&s| *s <= 100000).sum::<usize>();
    println!("Part 1 solution = {}", part1_solution);

    // part 2
    let filesystem_size: usize = 70000000;
    let required_free_space: usize = 30000000;
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
