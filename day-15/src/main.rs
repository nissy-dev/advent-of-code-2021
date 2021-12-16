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

    let maze = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|e| e.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut costs = maze
        .clone()
        .iter()
        .map(|e| e.iter().map(|_| i32::MAX).collect())
        .collect::<Vec<Vec<_>>>();
    let mut queue = VecDeque::new();
    queue.push_back((0i32, 0i32));
    costs[0][0] = maze[0][0] as i32;
    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        for (dy, dx) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (ny, nx) = (y + dy, x + dx);
            if nx < 0 || ny < 0 || nx >= maze[0].len() as i32 || ny >= maze.len() as i32 {
                continue;
            }

            if costs[ny as usize][nx as usize]
                > costs[y as usize][x as usize] + maze[ny as usize][nx as usize] as i32
            {
                costs[ny as usize][nx as usize] =
                    costs[y as usize][x as usize] + maze[ny as usize][nx as usize] as i32;
                queue.push_back((ny, nx));
            }
        }
    }

    println!(
        "part1 ans: {:?}",
        costs[maze.len() - 1][maze[0].len() - 1] - maze[0][0] as i32
    );
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);

    let init_maze = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|e| e.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let init_maze_x_len = init_maze[0].len();
    let init_maze_y_len = init_maze.len();
    let maze_x_len = init_maze_x_len * 5;
    let maze_y_len = init_maze_y_len * 5;
    let mut maze = vec![vec![0; maze_x_len]; maze_y_len];
    for y in 0..maze_y_len {
        for x in 0..maze_x_len {
            let (yq, yr) = (y / init_maze_y_len, y % init_maze_y_len);
            let (xq, xr) = (x / init_maze_x_len, x % init_maze_x_len);
            let mut value = init_maze[yr][xr] + xq as u32 + yq as u32;
            if value > 9 {
                value -= 9
            }
            maze[y][x] = value;
        }
    }

    let mut costs = maze
        .clone()
        .iter()
        .map(|e| e.iter().map(|_| i32::MAX).collect())
        .collect::<Vec<Vec<_>>>();
    let mut queue = VecDeque::new();
    queue.push_back((0i32, 0i32));
    costs[0][0] = maze[0][0] as i32;
    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        for (dy, dx) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (ny, nx) = (y + dy, x + dx);
            if nx < 0 || ny < 0 || nx >= maze[0].len() as i32 || ny >= maze.len() as i32 {
                continue;
            }

            if costs[ny as usize][nx as usize]
                > costs[y as usize][x as usize] + maze[ny as usize][nx as usize] as i32
            {
                costs[ny as usize][nx as usize] =
                    costs[y as usize][x as usize] + maze[ny as usize][nx as usize] as i32;
                queue.push_back((ny, nx));
            }
        }
    }

    println!(
        "part2 ans: {:?}",
        costs[maze.len() - 1][maze[0].len() - 1] - maze[0][0] as i32
    );
}
