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

    let data = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let operation = line.split(" ").next().unwrap();
            let range = line
                .split(" ")
                .last()
                .unwrap()
                .split(",")
                .map(|pos| {
                    let mut start = pos.split("..").next().unwrap()[2..]
                        .parse::<isize>()
                        .unwrap();
                    let mut end = pos.split("..").last().unwrap().parse::<isize>().unwrap();
                    if start < 0 && start < -50 {
                        start = -50;
                    }
                    if end > 0 && end > 50 {
                        end = 50;
                    }
                    (start, end)
                })
                .collect::<Vec<_>>();
            (operation.to_owned(), range.to_owned())
        })
        .collect::<Vec<_>>();

    let cube_size = 101;
    let mut cubes = vec![vec![vec![0; cube_size]; cube_size]; cube_size];
    for (operation, range) in data {
        let (x_min, x_max) = range[0];
        let (y_min, y_max) = range[1];
        let (z_min, z_max) = range[2];
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    if operation == "on" {
                        cubes[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = 1;
                    } else {
                        cubes[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = 0;
                    }
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..cube_size {
        for y in 0..cube_size {
            for z in 0..cube_size {
                if cubes[x][y][z] == 1 {
                    count += 1;
                }
            }
        }
    }

    println!("part1 ans: {}", count);
}

// see: https://github.com/yosuke-furukawa/advent-of-code-2021/blob/main/day-22/src/main.rs
fn area(range: &Vec<(isize, isize)>) -> i64 {
    let (x_min, x_max) = range[0];
    let (y_min, y_max) = range[1];
    let (z_min, z_max) = range[2];
    ((x_max - x_min + 1) * (y_max - y_min + 1) * (z_max - z_min + 1)) as i64
}

fn overlap(range: &Vec<(isize, isize)>, checked_ranges: &[Vec<(isize, isize)>]) -> i64 {
    checked_ranges
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let min_x = range[0].0.max(r[0].0);
            let max_x = range[0].1.min(r[0].1);
            let min_y = range[1].0.max(r[1].0);
            let max_y = range[1].1.min(r[1].1);
            let min_z = range[2].0.max(r[2].0);
            let max_z = range[2].1.min(r[2].1);
            if max_x - min_x >= 0 && max_y - min_y >= 0 && max_z - min_z >= 0 {
                let new_range = vec![(min_x, max_x), (min_y, max_y), (min_z, max_z)];
                // 除きたい範囲と点灯範囲の差分を計算 (1つごとに差分計算する)
                area(&new_range) - overlap(&new_range, &checked_ranges[i + 1..])
            } else {
                0
            }
        })
        .sum()
}

fn part2_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let operation = line.split(" ").next().unwrap();
            let range = line
                .split(" ")
                .last()
                .unwrap()
                .split(",")
                .map(|pos| {
                    let start = pos.split("..").next().unwrap()[2..]
                        .parse::<isize>()
                        .unwrap();
                    let end = pos.split("..").last().unwrap().parse::<isize>().unwrap();
                    (start, end)
                })
                .collect::<Vec<_>>();
            (operation.to_owned(), range.to_owned())
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    // 点灯させたい範囲から除きたい範囲のリスト
    let mut checked_ranges = vec![];
    for (operation, range) in data.into_iter().rev() {
        if operation == "on" {
            // 重複を除いた範囲を計算
            result += area(&range) - overlap(&range, &checked_ranges);
        }
        checked_ranges.push(range);
    }

    println!("part2 ans: {}", result);
}
