use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    part2_solver();
}

fn bit_to_decimal(bit: &str) -> isize {
    let mut num = 0isize;
    let digit = bit.len();
    for (i, c) in bit.chars().enumerate() {
        if c == '1' {
            num += 2_isize.pow((digit - 1 - i) as u32);
        }
    }
    return num;
}

fn decode_for_part1(packet: &str) -> (isize, usize) {
    let mut packet_ver_sum = 0;
    let packet_ver = bit_to_decimal(&packet[0..3]);
    let type_id = bit_to_decimal(&packet[3..6]);
    packet_ver_sum += packet_ver;

    if type_id == 4 {
        let mut value = String::new();
        let mut scan_continue = true;
        let mut scan_index = 6;
        while scan_continue {
            if &packet[scan_index..scan_index + 1] == "0" {
                scan_continue = false;
            }
            value.push_str(&packet[scan_index + 1..scan_index + 5]);
            scan_index += 5;
        }
        return (packet_ver_sum, scan_index);
    } else {
        let mut returned_index = 0;
        let length_type_id = bit_to_decimal(&packet[6..7]);
        if length_type_id == 0 {
            let total_length = bit_to_decimal(&packet[7..22]) as usize;
            let start_index = 22;
            let mut length = 0;
            while length != total_length {
                let (packet_ver, scan_length) = decode_for_part1(&packet[start_index + length..]);
                packet_ver_sum += packet_ver;
                length += scan_length;
            }
            returned_index = start_index + length;
        } else {
            let total_num = bit_to_decimal(&packet[7..18]);
            let mut index = 18;
            let mut cnt = 0;
            while cnt < total_num {
                let (packet_ver, scan_length) = decode_for_part1(&packet[index..]);
                packet_ver_sum += packet_ver;
                index += scan_length;
                cnt += 1;
            }
            returned_index = index;
        }

        return (packet_ver_sum, returned_index);
    }
}

fn part1_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();
    map.insert('0', "0000");
    map.insert('1', "0001");
    map.insert('2', "0010");
    map.insert('3', "0011");
    map.insert('4', "0100");
    map.insert('5', "0101");
    map.insert('6', "0110");
    map.insert('7', "0111");
    map.insert('8', "1000");
    map.insert('9', "1001");
    map.insert('A', "1010");
    map.insert('B', "1011");
    map.insert('C', "1100");
    map.insert('D', "1101");
    map.insert('E', "1110");
    map.insert('F', "1111");

    let mut bits = String::from("");
    for line in reader.lines() {
        let line = line.unwrap();
        for c in line.chars() {
            bits.push_str(map.get(&c).unwrap());
        }
    }

    let (packet_ver_sum, _) = decode_for_part1(&bits);
    println!("part1 ans: {:?}", packet_ver_sum);
}

fn decode_for_part2(packet: &str) -> (isize, usize) {
    let type_id = bit_to_decimal(&packet[3..6]);
    if type_id == 4 {
        let mut value = String::new();
        let mut scan_continue = true;
        let mut scan_index = 6;
        while scan_continue {
            if &packet[scan_index..scan_index + 1] == "0" {
                scan_continue = false;
            }
            value.push_str(&packet[scan_index + 1..scan_index + 5]);
            scan_index += 5;
        }
        return (bit_to_decimal(&value), scan_index);
    } else {
        let mut returned_index = 0;
        let mut values = vec![];
        let length_type_id = bit_to_decimal(&packet[6..7]);
        if length_type_id == 0 {
            let total_length = bit_to_decimal(&packet[7..22]) as usize;
            let start_index = 22;
            let mut length = 0;
            while length != total_length {
                let (value, scan_length) = decode_for_part2(&packet[start_index + length..]);

                values.push(value);
                length += scan_length;
            }
            returned_index = start_index + length;
        } else {
            let total_num = bit_to_decimal(&packet[7..18]);
            let mut index = 18;
            let mut cnt = 0;
            while cnt < total_num {
                let (value, scan_length) = decode_for_part2(&packet[index..]);
                values.push(value);
                index += scan_length;
                cnt += 1;
            }
            returned_index = index;
        }

        let mut value = 0;
        if type_id == 0 {
            value = values.iter().sum();
        } else if type_id == 1 {
            value = values.iter().product();
        } else if type_id == 2 {
            value = *values.iter().min().unwrap();
        } else if type_id == 3 {
            value = *values.iter().max().unwrap();
        } else if type_id == 5 {
            if values[0] > values[1] {
                value = 1;
            }
        } else if type_id == 6 {
            if values[0] < values[1] {
                value = 1;
            }
        } else if type_id == 7 {
            if values[0] == values[1] {
                value = 1;
            }
        }

        return (value, returned_index);
    }
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();
    map.insert('0', "0000");
    map.insert('1', "0001");
    map.insert('2', "0010");
    map.insert('3', "0011");
    map.insert('4', "0100");
    map.insert('5', "0101");
    map.insert('6', "0110");
    map.insert('7', "0111");
    map.insert('8', "1000");
    map.insert('9', "1001");
    map.insert('A', "1010");
    map.insert('B', "1011");
    map.insert('C', "1100");
    map.insert('D', "1101");
    map.insert('E', "1110");
    map.insert('F', "1111");

    let mut bits = String::from("");
    for line in reader.lines() {
        let line = line.unwrap();
        for c in line.chars() {
            bits.push_str(map.get(&c).unwrap());
        }
    }

    let (value, _) = decode_for_part2(&bits);
    println!("part2 ans: {:?}", value);
}
