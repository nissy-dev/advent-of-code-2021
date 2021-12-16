use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    part2_solver();
}

// https://github.com/rhysd/misc/blob/master/adventofcode/2021/12_1.rs
fn part1_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut poly = String::new();
    let mut rules = HashMap::new();
    let mut cnt = 0;
    reader.lines().for_each(|l| {
        let l = l.unwrap();

        if cnt == 0 {
            poly = l.to_string();
        } else if l != "" {
            let values = l.split(" -> ").collect::<Vec<&str>>();
            rules.insert(values[0].to_string(), values[1].to_string());
        }

        cnt += 1;
    });

    let cnt = 10;
    for _ in 0..cnt {
        let mut inserts = vec![];
        for i in 0..poly.len() - 1 {
            let key = &poly[i..i + 2];
            if rules.contains_key(key) {
                let value = rules.get(key).unwrap();
                inserts.push((i + 1, value));
            }
        }

        for (i, (pos, value)) in inserts.into_iter().enumerate() {
            poly.insert(pos + i, value.chars().nth(0).unwrap());
        }
    }

    let mut counter = HashMap::new();
    for val in poly.chars() {
        let cnt = counter.entry(val).or_insert(0);
        *cnt += 1;
    }

    let ans = counter.values().max().unwrap() - counter.values().min().unwrap();
    println!("part1 ans: {:?}", ans);
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut poly = String::new();
    let mut rules = HashMap::new();
    // ペアのカウント
    let mut pairs = HashMap::new();
    // 文字のカウント
    let mut chars = HashMap::new();
    let mut cnt = 0;
    reader.lines().for_each(|l| {
        let l = l.unwrap();

        if cnt == 0 {
            poly = l.to_string();
        } else if l != "" {
            let values = l.split(" -> ").collect::<Vec<&str>>();
            rules.insert(values[0].to_string(), values[1].to_string());
            pairs.insert(values[0].to_string(), 0usize);
            chars.entry(values[1].to_string()).or_insert(0usize);
        }

        cnt += 1;
    });

    for i in 0..poly.len() - 1 {
        let key = &poly[i..i + 2];
        if let Some(value) = pairs.get_mut(key) {
            *value += 1;
        }
    }

    for val in poly.chars() {
        let cnt = chars.entry(val.to_string()).or_insert(0);
        *cnt += 1;
    }

    let cnt = 40;
    for _ in 0..cnt {
        for (pair, value) in pairs.clone().into_iter() {
            // 検索してペアのカウントをへらす
            *pairs.get_mut(&pair).unwrap() -= value;
            *pairs
                .get_mut(&(pair.chars().nth(0).unwrap().to_string() + &rules[&pair]))
                .unwrap() += value;
            *pairs
                .get_mut(
                    &(rules[&pair].as_str().to_owned() + &pair.chars().nth(1).unwrap().to_string()),
                )
                .unwrap() += value;
            // 挿入文字を増やす
            *chars.get_mut(&rules[&pair]).unwrap() += value;
        }
    }

    let ans = chars.values().max().unwrap() - chars.values().min().unwrap();
    println!("part2 ans: {:?}", ans);
}
