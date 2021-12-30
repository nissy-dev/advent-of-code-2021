use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    part2_solver();
}

// https://github.com/rhysd/misc/blob/master/adventofcode/2021/12_1.rs
fn part1_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut paper_x: usize = 0;
    let mut paper_y: usize = 0;
    let mut values = vec![];
    let mut folds = vec![];
    reader.lines().for_each(|l| {
        let l = l.unwrap();
        if l.starts_with("fold along") {
            let value = l.split(" ").last().unwrap();
            let axis = value.chars().next().unwrap().to_string();
            let pos = value[2..].parse::<usize>().unwrap();
            folds.push((axis, pos));
        } else if l != "" {
            let value = l
                .split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if paper_x < value[0] {
                paper_x = value[0];
            }
            if paper_y < value[1] {
                paper_y = value[1];
            }
            values.push((value[0], value[1]));
        }
    });

    let mut paper = vec![vec![0; paper_x + 1]; paper_y + 1];
    for (x, y) in values {
        paper[y][x] = 1;
    }

    for (axis, pos) in folds {
        paper_y = paper.len() - 1;
        paper_x = paper[0].len() - 1;

        if axis == "x" {
            for y in 0..paper_y + 1 {
                for x in pos + 1..paper_x + 1 {
                    let x_fold = pos * 2 - x;
                    if x_fold < paper_x + 1 && paper[y][x_fold] == 1 {
                        paper[y][x] = 1;
                    }
                }
            }
            paper = paper.iter().map(|e| e[pos + 1..].to_vec()).collect();
        } else {
            for y in 0..pos + 1 {
                for x in 0..paper_x + 1 {
                    let y_fold = pos * 2 - y;
                    if y_fold < paper_y + 1 && paper[y_fold][x] == 1 {
                        paper[y][x] = 1;
                    }
                }
            }
            paper = paper[0..pos + 1].to_vec();
        }
        break;
    }

    println!(
        "part1 ans: {:?}",
        paper.iter().map(|e| e.iter().sum::<usize>()).sum::<usize>()
    );
}

fn part2_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut paper_x: usize = 0;
    let mut paper_y: usize = 0;
    let mut values = vec![];
    let mut folds = vec![];
    reader.lines().for_each(|l| {
        let l = l.unwrap();
        if l.starts_with("fold along") {
            let value = l.split(" ").last().unwrap();
            let axis = value.chars().next().unwrap().to_string();
            let pos = value[2..].parse::<usize>().unwrap();
            folds.push((axis, pos));
        } else if l != "" {
            let value = l
                .split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if paper_x < value[0] {
                paper_x = value[0];
            }
            if paper_y < value[1] {
                paper_y = value[1];
            }
            values.push((value[0], value[1]));
        }
    });

    let mut paper = vec![vec![0; paper_x + 1]; paper_y + 1];
    for (x, y) in values {
        paper[y][x] = 1;
    }

    // TODO 共通化できそう...
    for (axis, pos) in folds {
        paper_y = paper.len() - 1;
        paper_x = paper[0].len() - 1;

        if axis == "x" {
            for y in 0..paper_y + 1 {
                for x in pos + 1..paper_x + 1 {
                    let x_fold = pos * 2 - x;
                    if x_fold < paper_x + 1 && paper[y][x_fold] == 1 {
                        paper[y][x] = 1;
                    }
                }
            }
            paper = paper.iter().map(|e| e[pos + 1..].to_vec()).collect();
        } else {
            for y in 0..pos + 1 {
                for x in 0..paper_x + 1 {
                    let y_fold = pos * 2 - y;
                    if y_fold < paper_y + 1 && paper[y_fold][x] == 1 {
                        paper[y][x] = 1;
                    }
                }
            }
            paper = paper[0..pos + 1].to_vec();
        }
    }

    for row in paper {
        println!(
            "{:?}",
            row.into_iter()
                .rev()
                .map(|e| if e == 1 { '#' } else { '.' })
                .collect::<String>()
        );
    }
}
