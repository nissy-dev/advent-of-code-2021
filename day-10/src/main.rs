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

    let inputs = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut cnt = 0;
    let mut map = HashMap::new();
    map.insert('(', (')', 3));
    map.insert('[', (']', 57));
    map.insert('{', ('}', 1197));
    map.insert('<', ('>', 25137));
    for i in 0..inputs.len() {
        let mut stack = vec![];
        for char in inputs[i].iter() {
            if map.contains_key(&char) {
                stack.push(char);
            } else {
                let key = stack.pop().unwrap();
                if char != &map[key].0 {
                    for (val, num) in map.values() {
                        if char == val {
                            cnt += num;
                        }
                    }
                    break;
                }
            }
        }
    }

    println!("part1 ans: {:?}", cnt);
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);

    let inputs = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut scores = vec![];
    let mut map = HashMap::new();
    map.insert('(', (')', 1));
    map.insert('[', (']', 2));
    map.insert('{', ('}', 3));
    map.insert('<', ('>', 4));
    for i in 0..inputs.len() {
        let mut stack = vec![];
        let mut flag = true;
        for char in inputs[i].iter() {
            if map.contains_key(&char) {
                stack.push(char);
            } else {
                let key = stack.pop().unwrap();
                if char != &map[key].0 {
                    flag = false;
                    break;
                }
            }
        }

        if flag {
            let mut score: usize = 0;
            stack.reverse();
            for val in stack {
                score = score * 5 + map[val].1;
            }
            scores.push(score);
        }
    }

    scores.sort();
    println!("part2 ans: {:?}", scores[scores.len() / 2]);
}
