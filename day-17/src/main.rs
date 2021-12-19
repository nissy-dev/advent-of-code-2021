use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    // part2_solver();
}

fn part1_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut area = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        line.split(" ").for_each(|x| {
            if x.starts_with("x=") {
                let mut val = String::from(x);
                val = val[2..(val.len() - 1) as usize].to_string();
                val.split("..").for_each(|v| {
                    area.push(v.parse::<i32>().unwrap());
                });
            } else if x.starts_with("y=") {
                let mut val = String::from(x);
                val = val[2..].to_string();
                val.split("..").for_each(|v| {
                    area.push(v.parse::<i32>().unwrap());
                });
            }
        });
    }

    // let mut pos = vec![];
    // let mut v = vec![];

    println!("{:?}", area);
}

fn part2_solver() {}
