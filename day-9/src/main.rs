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

        inputs.push(
            line_str
                .chars()
                .map(|v| v.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut cnt = 0;
    let x_len = inputs[0].len() as i32;
    let y_len = inputs.len() as i32;
    for i in 0..y_len {
        for j in 0..x_len {
            let value = inputs[i as usize][j as usize];
            let mut surrunding_num = 0;
            let mut surrunding_large_num = 0;
            for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let x = j as i32 + dx;
                let y = i as i32 + dy;
                if x < 0 || x >= x_len as i32 || y < 0 || y >= y_len as i32 {
                    continue;
                }
                surrunding_num += 1;
                if value < inputs[y as usize][x as usize] {
                    surrunding_large_num += 1;
                }
            }

            if surrunding_large_num == surrunding_num {
                cnt += value + 1;
            }
        }
    }

    println!("part1 ans: {:?}", cnt);
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inputs = vec![];
    for (_, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();

        inputs.push(
            line_str
                .chars()
                .map(|v| v.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let x_len = inputs[0].len() as i32;
    let y_len = inputs.len() as i32;
    let mut spot = vec![];
    for i in 0..y_len {
        for j in 0..x_len {
            let value = inputs[i as usize][j as usize];
            let mut surrunding_num = 0;
            let mut surrunding_large_num = 0;
            for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let x = j as i32 + dx;
                let y = i as i32 + dy;
                if x < 0 || x >= x_len as i32 || y < 0 || y >= y_len as i32 {
                    continue;
                }
                surrunding_num += 1;
                if value < inputs[y as usize][x as usize] {
                    surrunding_large_num += 1;
                }
            }

            if surrunding_large_num == surrunding_num {
                spot.push((j as i32, i as i32));
            }
        }
    }

    let mut basin_size_vec = vec![];
    for (x, y) in spot {
        let mut pos_stack = vec![(x, y)];
        let mut pos_stock = vec![(x, y)];
        while let Some((tmp_x, tmp_y)) = pos_stack.pop() {
            for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let x = tmp_x + dx;
                let y = tmp_y + dy;
                if x < 0 || x >= x_len as i32 || y < 0 || y >= y_len as i32 {
                    continue;
                }

                if inputs[y as usize][x as usize] == 9 {
                    continue;
                }

                if !pos_stock.contains(&(x, y)) {
                    pos_stock.push((x, y));
                    pos_stack.push((x, y));
                }
            }
        }
        basin_size_vec.push(pos_stock.len());
    }

    basin_size_vec.sort_by(|a, b| b.cmp(&a));
    println!(
        "part2 ans: {:?}",
        basin_size_vec[0] * basin_size_vec[1] * basin_size_vec[2]
    );
}
