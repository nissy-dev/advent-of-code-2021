use std::cmp;
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
    let mut inputs: Vec<(Vec<usize>, Vec<usize>)> = vec![];

    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        let line_split: Vec<&str> = line_str.split("->").collect();
        let x = &line_split[0][0..line_split[0].len() - 1]
            .split(",")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let y = &line_split[1][1..]
            .split(",")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        inputs.push((x.clone(), y.clone()));

        if x[0] > x_max || y[0] > x_max {
            x_max = cmp::max(x[0], y[0]);
        }

        if x[1] > y_max || y[1] > y_max {
            y_max = cmp::max(x[1], y[1]);
        }
    }

    let mut points = vec![vec![0usize; x_max + 1]; y_max + 1];
    for (start, end) in inputs {
        if start[0] == end[0] {
            let x = start[0];
            let min = cmp::min(start[1], end[1]);
            let max = cmp::max(start[1], end[1]);
            for i in min..max + 1 {
                points[i][x] += 1;
            }
        }

        if start[1] == end[1] {
            let y = start[1];
            let min = cmp::min(start[0], end[0]);
            let max = cmp::max(start[0], end[0]);
            for i in min..max + 1 {
                points[y][i] += 1;
            }
        }
    }

    let mut count = 0;
    for i in 0..y_max + 1 {
        for j in 0..x_max + 1 {
            if points[i][j] >= 2 {
                count += 1
            }
        }
    }

    println!("part1 ans: {}", count);
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut inputs: Vec<(Vec<usize>, Vec<usize>)> = vec![];

    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        let line_split: Vec<&str> = line_str.split("->").collect();
        let x = &line_split[0][0..line_split[0].len() - 1]
            .split(",")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let y = &line_split[1][1..]
            .split(",")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        inputs.push((x.clone(), y.clone()));

        if x[0] > x_max || y[0] > x_max {
            x_max = cmp::max(x[0], y[0]);
        }

        if x[1] > y_max || y[1] > y_max {
            y_max = cmp::max(x[1], y[1]);
        }
    }

    let mut points = vec![vec![0usize; x_max + 1]; y_max + 1];
    for (start, end) in inputs {
        if start[0] == end[0] {
            let x = start[0];
            let min = cmp::min(start[1], end[1]);
            let max = cmp::max(start[1], end[1]);
            for i in min..max + 1 {
                points[i][x] += 1;
            }
        }

        if start[1] == end[1] {
            let y = start[1];
            let min = cmp::min(start[0], end[0]);
            let max = cmp::max(start[0], end[0]);
            for i in min..max + 1 {
                points[y][i] += 1;
            }
        }

        let diff_x = (start[0] as isize - end[0] as isize).abs();
        let diff_y = (start[1] as isize - end[1] as isize).abs();
        if diff_x == diff_y {
            let mut tmp_x = start[0] as isize;
            let mut tmp_y = start[1] as isize;
            let dx = if start[0] < end[0] { 1 } else { -1 };
            let dy = if start[1] < end[1] { 1 } else { -1 };
            for _ in 0..diff_x + 1 {
                points[tmp_y as usize][tmp_x as usize] += 1;
                tmp_x += dx;
                tmp_y += dy;
            }
        }
    }

    let mut count = 0;
    for i in 0..y_max + 1 {
        for j in 0..x_max + 1 {
            if points[i][j] >= 2 {
                count += 1
            }
        }
    }

    println!("part2 ans: {}", count);
}
