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
    let mut inputs: Vec<(String, i32)> = vec![];
    for line in reader.lines() {
        let line_str = line.unwrap();
        let commands = &line_str.split(" ").collect::<Vec<_>>();
        let command = String::from(commands[0]);
        let value = commands[1].parse::<i32>().unwrap();
        inputs.push((command, value));
    }

    let mut h_pos = 0;
    let mut depth = 0;
    for (command, value) in inputs {
        if command == "forward" {
            h_pos += value;
        } else if command == "up" {
            depth -= value;
        } else if command == "down" {
            depth += value;
        }
    }

    println!("part1 ans: {}", h_pos * depth);
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut inputs: Vec<(String, i32)> = vec![];
    for line in reader.lines() {
        let line_str = line.unwrap();
        let commands = &line_str.split(" ").collect::<Vec<_>>();
        let command = String::from(commands[0]);
        let value = commands[1].parse::<i32>().unwrap();
        inputs.push((command, value));
    }

    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (command, value) in inputs {
        if command == "forward" {
            h_pos += value;
            depth += aim * value
        } else if command == "up" {
            aim -= value;
        } else if command == "down" {
            aim += value;
        }
    }

    println!("part1 ans: {}", h_pos * depth);
}
