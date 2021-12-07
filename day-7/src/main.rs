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

    let mut inputs = vec![];
    for (_, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        for value in line_str.split(",").into_iter() {
            inputs.push(value.parse::<usize>().unwrap());
        }
    }

    let max = inputs.iter().max().unwrap().clone();
    let min = inputs.iter().min().unwrap().clone();

    let mut cost = vec![];
    for pos in min..max + 1 {
        let mut tmp_cost = 0;
        for value in inputs.clone().into_iter() {
            tmp_cost += (value as isize - pos as isize).abs();
        }
        cost.push(tmp_cost);
    }

    println!("part1 ans: {:?}", cost.iter().min().unwrap());
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inputs = vec![];
    for (_, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        for value in line_str.split(",").into_iter() {
            inputs.push(value.parse::<usize>().unwrap());
        }
    }

    let max = inputs.iter().max().unwrap().clone();
    let min = inputs.iter().min().unwrap().clone();

    let mut cost = vec![];
    for pos in min..max + 1 {
        let mut tmp_cost = 0;
        for value in inputs.clone().into_iter() {
            let diff = (value as isize - pos as isize).abs();
            tmp_cost += diff * (diff + 1) / 2;
        }
        cost.push(tmp_cost);
    }

    println!("part2 ans: {:?}", cost.iter().min().unwrap());

    // println!("part2 ans: {:?}", cnt);
}
