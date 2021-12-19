use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    // part2_solver();
}

fn bitToDecimal(bit: &str) -> i32 {
    let mut num = 0;
    let digit = bit.len();
    for (i, c) in bit.chars().enumerate() {
        if c == '1' {
            num += 2_i32.pow((digit - 1 - i) as u32);
        }
    }
    return num;
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

    let mut ver_sum = 0;
    let mut packets = vec![bits];
    while packets.len() > 0 {
        let packet = packets.pop().unwrap();
        println!("packet: {}", packet);
        let ver = bitToDecimal(&packet[0..3]);
        println!("ver: {}", ver);
        ver_sum += ver;
        let type_id = bitToDecimal(&packet[3..6]);
        if type_id == 4 {
            // TODO: Literal Value
        } else {
            let length_type_id = bitToDecimal(&packet[6..7]);
            if length_type_id == 0 {
                let total_bit_num = bitToDecimal(&packet[7..22]);
                let mut start = 22;
                let mut end = 22;
                let type_id = bitToDecimal(&packet[start + 3..start + 6]);
                while start < total_bit_num as usize + 22 {
                    if type_id == 4 {
                        let mut cnt = 0;
                        while &packet[start + 6 + cnt * 5..start + 7 + cnt * 5] != "0" {
                            cnt += 1;
                        }
                        end = start + 6 + (cnt + 1) * 5;
                        packets.push(packet[start..end].to_string());
                        start = end;
                    } else {
                        packets.push(packet[start..].to_string());
                        break;
                    }
                }
            } else {
                let total_packet_num = bitToDecimal(&packet[7..18]);
                let mut start = 18;
                let mut end = 18;
                for _ in 0..total_packet_num {
                    let type_id = bitToDecimal(&packet[start + 3..start + 6]);
                    if type_id == 4 {
                        let mut cnt = 0;
                        while &packet[start + 6 + cnt * 5..start + 7 + cnt * 5] != "0" {
                            cnt += 1;
                        }
                        end = start + 6 + (cnt + 1) * 5;
                        packets.push(packet[start..end].to_string());
                        start = end;
                    } else {
                        packets.push(packet[start..].to_string());
                        break;
                    }
                }
            }
        }
        packets.reverse();
    }
    println!("{}", ver_sum);
}

fn part2_solver() {}
