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

    for _ in 0..80 {
        let mut zero_cnt = 0;
        for value in inputs.iter_mut() {
            if value != &0 {
                *value = *value - 1;
            } else {
                *value = 6;
                zero_cnt += 1;
            }
        }

        for _ in 0..zero_cnt {
            inputs.push(8);
        }
    }

    println!("part1 ans: {:?}", inputs.len());
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

    // 0, 1, 1, 2, 1, 0, 0
    // 0, 1, 1, 3, 2, 2, 1 (2ずらして足している..?)
    // = (0, 1, 1, 2, 1, 0, 0) + (0, 0, 0, 1, 1, 2, 1, 0, 0)
    // 0, 1  1, 4, 3, 5, 3
    // = (0, 1, 1, 3, 2, 2, 1) + (0, 0, 0, 1, 1, 3, 2, 2, 1)
    // 2, 2, 1, 5, 4. 9. 6
    // = (0, 1  1, 4, 3, 5, 3) + (2, 1, 0, 1  1, 4, 3, 5, 3)

    let mut add_num = vec![0usize; 7];
    for day in 0..7 {
        for value in inputs.iter_mut() {
            if value != &0 {
                *value = *value - 1;
            } else {
                *value = 6;
                add_num[day] += 1;
            }
        }
    }

    add_num.insert(0, 0);
    add_num.insert(0, 0);

    let mut cnt: usize = inputs.len() + add_num.clone().into_iter().sum::<usize>();
    let days = 256;
    let loop_num = ((days - 7) as f32 / 7.0).ceil() as usize;
    let stop_day = days % 7;
    for loop_index in 0..loop_num {
        let mut add_values = vec![];
        for day in 0..7 {
            if loop_index == loop_num - 1 && day >= stop_day {
                println!("part2 ans: {:?}", cnt);
                return;
            }
            let add_value = add_num[day] + add_num[day + 2];
            cnt += add_value;
            add_values.push(add_value);
        }

        for add_value in add_values {
            add_num.remove(0);
            add_num.push(add_value)
        }
    }

    println!("part2 ans: {:?}", cnt);
}
