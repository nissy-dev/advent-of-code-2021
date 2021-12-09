use std::collections::HashMap;
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
        let mut left_values = vec![];
        let mut right_values = vec![];
        let mut output_flag = false;
        for value in line_str.split(" ").into_iter() {
            if value == "|" {
                output_flag = true;
                continue;
            }
            if output_flag {
                right_values.push(value.to_string());
            } else {
                left_values.push(value.to_string());
            }
        }

        inputs.push((left_values, right_values));
    }

    let mut cnt = 0;
    for (_, output_arr) in inputs.iter() {
        for output in output_arr.iter() {
            if [2, 3, 4, 7].contains(&output.len()) {
                cnt += 1;
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
        let mut left_values = vec![];
        let mut right_values = vec![];
        let mut output_flag = false;
        for value in line_str.split(" ").into_iter() {
            if value == "|" {
                output_flag = true;
                continue;
            }
            if output_flag {
                right_values.push(value.to_string());
            } else {
                left_values.push(value.to_string());
            }
        }

        inputs.push((left_values, right_values));
    }

    // 0 -> a, b, c, e, f, g (6)
    // 1 -> c, f (2)
    // 2 -> a, c, d, e, g (5)
    // 3 -> a, c, d, f, g (5)
    // 4 -> b, c, d, f (4)
    // 5 -> a, b, d, f, g (5)
    // 6 -> a, b, d, e, f, g (6)
    // 7 -> a, c, f (3)
    // 8 -> a, b, c, d, e, f, g (7)
    // 9 -> a, b, c, d, f, g (6)

    // 桁数で4つの数字 (1, 4, 7, 8) が決まる
    // 2桁で c, f の組が決定
    // c,f を持つ5桁 -> 3
    // c,f を持たない6桁 -> 6
    // e,g を持つ6桁 -> 0
    // e,g を持たない6桁 -> 9
    // e,g を持つ5桁 -> 2
    // e,g を持たない5桁 -> 5
    // 3桁 - 2桁 で a が決まる
    // 7桁 - 4桁 - aに対応するやつで e, g の組が決定
    // e,g が両方あるやつを

    let mut cnt = 0;
    for (mut right, mut left) in inputs.into_iter() {
        let output = left.clone();
        right.append(&mut left);
        let mut map: HashMap<&str, Vec<char>> = HashMap::new();

        for value in right.iter() {
            let mut value_str = value.chars().collect::<Vec<char>>();
            value_str.sort();
            if value.len() == 2 {
                map.insert("1", value_str);
            } else if value.len() == 4 {
                map.insert("4", value_str);
            } else if value.len() == 3 {
                map.insert("7", value_str);
            } else if value.len() == 7 {
                map.insert("8", value_str);
            }
        }

        let cf = map.get("1").unwrap().clone();
        let bcdf = map.get("4").unwrap().clone();
        let acf = map.get("7").unwrap().clone();
        let mut eg = map.get("8").unwrap().clone();
        eg.retain(|x| !bcdf.contains(&x) && !acf.contains(&x));

        for value in right.iter() {
            let mut value_str = value.chars().collect::<Vec<char>>();
            value_str.sort();
            if value.len() == 5 && value_str.contains(&cf[0]) && value_str.contains(&cf[1]) {
                map.insert("3", value_str.clone());
            } else if value.len() == 6
                && (!value_str.contains(&cf[0]) || !value_str.contains(&cf[1]))
            {
                map.insert("6", value_str.clone());
            } else if value.len() == 5
                && (!value_str.contains(&cf[0]) || !value_str.contains(&cf[1]))
                && (value_str.contains(&eg[0]) && value_str.contains(&eg[1]))
            {
                map.insert("2", value_str.clone());
            } else if value.len() == 5
                && (!value_str.contains(&cf[0]) || !value_str.contains(&cf[1]))
                && (!value_str.contains(&eg[0]) || !value_str.contains(&eg[1]))
            {
                map.insert("5", value_str.clone());
            } else if value.len() == 6
                && (value_str.contains(&cf[0]) && value_str.contains(&cf[1]))
                && (value_str.contains(&eg[0]) && value_str.contains(&eg[1]))
            {
                map.insert("0", value_str.clone());
            } else if value.len() == 6
                && (value_str.contains(&cf[0]) && value_str.contains(&cf[1]))
                && (!value_str.contains(&eg[0]) || !value_str.contains(&eg[1]))
            {
                map.insert("9", value_str.clone());
            }
        }

        let mut output_val_str = String::from("");
        for value in output.iter() {
            let mut value_str = value.chars().collect::<Vec<char>>();
            value_str.sort();

            for (key, arr) in map.iter() {
                if &value_str == arr {
                    output_val_str += key;
                }
            }
        }
        cnt += output_val_str.parse::<usize>().unwrap();
    }

    println!("part2 ans: {:?}", cnt);
}
