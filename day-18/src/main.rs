use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    part2_solver();
}

fn part1_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);
}

fn part2_solver() {}
