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

    let mut cnt = 0;
    let mut prev_value = i32::max_value();
    for (_, line) in reader.lines().enumerate() {
        if let Ok(value) = i32::from_str_radix(&line.unwrap(), 10) {
            if value > prev_value {
                cnt += 1;
            }
            prev_value = value;
        }
    }

    println!("part1 ans: {}", cnt);
}

fn part2_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut inputs = vec![];
    for line in reader.lines() {
        if let Ok(value) = i32::from_str_radix(&line.unwrap(), 10) {
            inputs.push(value);
        }
    }

    let mut cnt = 0;
    let mut index = 0;
    while index < inputs.len() - 3 {
        if inputs[index] < inputs[index + 3] {
            cnt += 1;
        }
        index += 1;
    }
    println!("part2 ans: {}", cnt);
}
