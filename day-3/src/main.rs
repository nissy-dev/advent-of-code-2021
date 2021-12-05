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
    let mut inputs: Vec<String> = vec![];
    for line in reader.lines() {
        inputs.push(String::from(line.unwrap()));
    }

    let input_length = inputs.len();
    let bit_length = inputs[0].len();

    let mut bit_sum = vec![0usize; bit_length];
    for input in inputs {
        for i in 0..bit_length {
            let value = &input[i..i + 1].parse::<usize>().unwrap();
            bit_sum[i] += value;
        }
    }

    let mut gamma_rate_str = String::from("");
    for i in 0..bit_length {
        let bit = (bit_sum[i] as f32 > (input_length as f32 / 2.0)) as usize;
        gamma_rate_str.push_str(&bit.to_string());
    }

    let gamma_rate = usize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = 2usize.pow(bit_length as u32) - 1 - gamma_rate;

    println!("part1 ans: {}", gamma_rate * epsilon_rate);
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut inputs: Vec<String> = vec![];
    for line in reader.lines() {
        inputs.push(String::from(line.unwrap()));
    }

    let bit_length = inputs[0].len();
    let oxygen_generator_rate = search(bit_length, &inputs, "most");
    let co2_generator_rate = search(bit_length, &inputs, "least");
    println!("part2 ans: {}", oxygen_generator_rate * co2_generator_rate);
}

fn search(bit_length: usize, inputs: &Vec<String>, search_type: &str) -> usize {
    let mut tmp_inputs = inputs.clone();
    for i in 0..bit_length {
        let mut one_inputs = vec![];
        let mut zero_inputs = vec![];

        let input_length = tmp_inputs.len();
        if input_length == 1 {
            break;
        }

        let mut cnt = 0;
        for input in &tmp_inputs {
            let value = &input[i..i + 1].parse::<usize>().unwrap();
            cnt += value;
            if value == &1 {
                one_inputs.push(input.clone());
            } else {
                zero_inputs.push(input.clone());
            }
        }

        let flag = cnt as f32 >= (input_length as f32 / 2.0);
        if (flag && search_type == "most") || (!flag && search_type == "least") {
            tmp_inputs = one_inputs;
        } else {
            tmp_inputs = zero_inputs;
        }
    }

    return usize::from_str_radix(&tmp_inputs[0], 2).unwrap();
}
