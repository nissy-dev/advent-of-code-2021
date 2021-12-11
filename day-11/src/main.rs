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

    let mut inputs = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let step = 100;
    let mut flash_cnt = 0;
    let size = inputs.len();
    for _ in 0..step {
        // add 1 to all elements
        for i in 0..size {
            for j in 0..size {
                inputs[i][j] += 1;
            }
        }

        // 隣接する要素が9以上なら1を加える
        let mut stop = false;
        while !stop {
            stop = true;
            // 要素の更新
            let mut new_inputs = inputs.clone();
            for i in 0..size {
                for j in 0..size {
                    for (dx, dy) in vec![
                        (0, 1),
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, -1),
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                    ]
                    .iter()
                    {
                        let x = i as i32 + dx;
                        let y = j as i32 + dy;
                        if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                            if inputs[i][j] != 0 && inputs[x as usize][y as usize] > 9 {
                                new_inputs[i][j] += 1;
                            }
                        }
                    }
                }
            }

            for i in 0..size {
                for j in 0..size {
                    if inputs[i][j] > 9 {
                        new_inputs[i][j] = 0;
                    }
                }
            }

            inputs = new_inputs;

            for i in 0..size {
                for j in 0..size {
                    if inputs[i][j] > 9 {
                        stop = false;
                    }
                }
            }
        }

        for i in 0..size {
            for j in 0..size {
                if inputs[i][j] == 0 {
                    flash_cnt += 1;
                }
            }
        }
    }

    println!("part1 ans: {:?}", flash_cnt);
}

fn part2_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inputs = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let mut step_cnt = 0;
    let mut flag = true;
    let size = inputs.len();
    while flag {
        // add 1 to all elements
        for i in 0..size {
            for j in 0..size {
                inputs[i][j] += 1;
            }
        }

        // 隣接する要素が9以上なら1を加える
        let mut stop = false;
        while !stop {
            stop = true;
            // 要素の更新
            let mut new_inputs = inputs.clone();
            for i in 0..size {
                for j in 0..size {
                    for (dx, dy) in vec![
                        (0, 1),
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, -1),
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                    ]
                    .iter()
                    {
                        let x = i as i32 + dx;
                        let y = j as i32 + dy;
                        if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                            if inputs[i][j] != 0 && inputs[x as usize][y as usize] > 9 {
                                new_inputs[i][j] += 1;
                            }
                        }
                    }
                }
            }

            for i in 0..size {
                for j in 0..size {
                    if inputs[i][j] > 9 {
                        new_inputs[i][j] = 0;
                    }
                }
            }

            inputs = new_inputs;

            for i in 0..size {
                for j in 0..size {
                    if inputs[i][j] > 9 {
                        stop = false;
                    }
                }
            }
        }

        let mut sum = 0;
        for i in 0..size {
            for j in 0..size {
                sum += inputs[i][j];
            }
        }

        step_cnt += 1;

        if sum == 0 {
            flag = false;
        }
    }

    println!("part2 ans: {:?}", step_cnt);
}
