use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    part2_solver();
}

// これじゃなんかうまく行かない...
// fn bit_to_decimal(bit: &str) -> isize {
//     let mut num = 0isize;
//     let digit = bit.len();
//     for (i, c) in bit.chars().enumerate() {
//         if c == '#' {
//             num += 2_isize.pow((digit - 1 - i) as u32);
//         }
//     }
//     return num;
// }

// fn part1_solver() {
//     let file = File::open("input-part1.txt").unwrap();
//     let reader = BufReader::new(file);

//     let mut algorithm = String::new();
//     let mut input_image = vec![];
//     for (index, line) in reader.lines().enumerate() {
//         let line = line.unwrap();
//         if index == 0 {
//             algorithm.push_str(&line);
//         } else if index > 1 {
//             input_image.push(line.chars().collect::<Vec<char>>());
//         }
//     }

//     let mut loop_cnt = 0;
//     let loop_limit = 50;
//     while loop_cnt < loop_limit {
//         let image_size = input_image.len();
//         let mut tmp_image = vec![vec!['.'; image_size + 4]; image_size + 4];
//         for i in 0..image_size {
//             for j in 0..image_size {
//                 tmp_image[i + 2][j + 2] = input_image[i][j];
//             }
//         }

//         let mut new_image = vec![vec!['.'; image_size + 2]; image_size + 2];
//         for i in 1..(image_size + 3) as i32 {
//             for j in 1..(image_size + 3) as i32 {
//                 let mut pixels = String::new();
//                 for (dx, dy) in vec![
//                     (-1, -1),
//                     (-1, 0),
//                     (-1, 1),
//                     (0, -1),
//                     (0, 0),
//                     (0, 1),
//                     (1, -1),
//                     (1, 0),
//                     (1, 1),
//                 ] {
//                     let tmp_x = (i + dx) as usize;
//                     let tmp_y = (j + dy) as usize;
//                     pixels.push(tmp_image[tmp_x][tmp_y]);
//                 }
//                 let algorithm_index = bit_to_decimal(&pixels) as usize;
//                 let new_x = (i - 1) as usize;
//                 let new_y = (j - 1) as usize;
//                 new_image[new_x][new_y] = algorithm.chars().nth(algorithm_index).unwrap();
//             }
//         }
//         input_image = new_image;
//         loop_cnt += 1;
//     }

//     let mut ans = 0;
//     let image_size = input_image.len();
//     println!("{}", image_size);
//     for i in 0..image_size {
//         for j in 0..image_size {
//             if input_image[i][j] == '#' {
//                 ans += 1;
//             }
//         }
//     }
//     println!("part1 ans : {}", ans);
// }

// see: https://github.com/yosuke-furukawa/advent-of-code-2021/blob/main/day-20/src/main.rs
fn decode_from_binary(bin: &[u8]) -> u128 {
    bin.iter().enumerate().rev().fold(0, |acc, (pos, num)| {
        acc + *num as u128 * 2_u128.pow(bin.len() as u32 - 1 - pos as u32)
    })
}

fn part1_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let algorithm: Vec<u8> = lines[0]
        .clone()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let image: Vec<Vec<u8>> = lines[2..]
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut prev = image;
    let steps = 2;
    for step in 0..steps {
        let mut next = vec![];
        for y in -1..prev.len() as i32 + 1 {
            let mut next_line = vec![];
            for x in -1..prev[0].len() as i32 + 1 {
                let mut binary = vec![];
                for yy in y - 1..=y + 1 {
                    for xx in x - 1..=x + 1 {
                        if yy >= 0 && yy < prev.len() as i32 && xx >= 0 && xx < prev[0].len() as i32
                        {
                            binary.push(prev[yy as usize][xx as usize]);
                        } else {
                            // ここがポイントだったみたい
                            // 外側は、.........(000000000) のところも index = 0 と解釈される
                            // 今回の input は、index = 0 が # だから、0, 1 を交互に出る
                            binary.push(if step % 2 != 0 { 1 } else { 0 });
                        }
                    }
                }
                next_line.push(algorithm[decode_from_binary(&binary) as usize]);
            }
            next.push(next_line);
        }
        prev = next;
    }

    let mut result = 0;
    for p in prev.iter() {
        for n in p.iter() {
            if *n > 0 {
                result += 1;
            }
        }
    }

    println!("part1 ans : {}", result);
}

fn part2_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let algorithm: Vec<u8> = lines[0]
        .clone()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let image: Vec<Vec<u8>> = lines[2..]
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut prev = image;
    let steps = 50;
    for step in 0..steps {
        let mut next = vec![];
        for y in -1..prev.len() as i32 + 1 {
            let mut next_line = vec![];
            for x in -1..prev[0].len() as i32 + 1 {
                let mut binary = vec![];
                for yy in y - 1..=y + 1 {
                    for xx in x - 1..=x + 1 {
                        if yy >= 0 && yy < prev.len() as i32 && xx >= 0 && xx < prev[0].len() as i32
                        {
                            binary.push(prev[yy as usize][xx as usize]);
                        } else {
                            // ここがポイントだったみたい
                            binary.push(if step % 2 != 0 { 1 } else { 0 });
                        }
                    }
                }
                next_line.push(algorithm[decode_from_binary(&binary) as usize]);
            }
            next.push(next_line);
        }
        prev = next;
    }

    let mut result = 0;
    for p in prev.iter() {
        for n in p.iter() {
            if *n > 0 {
                result += 1;
            }
        }
    }

    println!("part1 ans : {}", result);
}
