use std::collections::HashSet;
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

    let x_max = area[1];
    let y_abs_max = -area[2];
    let mut candidates = HashSet::new();
    for x in 1..x_max + 1 {
        for y in 1..y_abs_max + 1 {
            let mut x_dist = x * (x + 1) / 2;
            for t in 1..2 * y_abs_max + 1 {
                if t <= x {
                    x_dist = t * (2 * x - (t - 1)) / 2;
                }
                let y_dist = t * (2 * y - (t - 1)) / 2;
                if x_dist >= area[0] && x_dist <= area[1] && y_dist >= area[2] && y_dist <= area[3]
                {
                    candidates.insert((x, y));
                }
            }
        }
    }

    let mut y_max = 0;
    for (_, y) in &candidates {
        let max = y * (y + 1) / 2;
        if y_max < max {
            y_max = max;
        }
    }

    println!("part1 ans: {:?}", y_max);
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
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

    let x_max = area[1];
    let y_abs_max = -area[2];
    let mut candidates = HashSet::new();
    for x in 1..x_max + 1 {
        for y in -y_abs_max..y_abs_max {
            let mut x_dist = x * (x + 1) / 2;
            for t in 1..2 * y_abs_max + 1 {
                if t <= x {
                    x_dist = t * (2 * x - (t - 1)) / 2;
                }
                let y_dist = t * (2 * y - (t - 1)) / 2;
                if x_dist >= area[0] && x_dist <= area[1] && y_dist >= area[2] && y_dist <= area[3]
                {
                    candidates.insert((x, y));
                }
            }
        }
    }

    println!("part2 ans: {:?}", candidates.len());
}
