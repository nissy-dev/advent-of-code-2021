use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    reader.lines().for_each(|l| {
        let l = l.unwrap();
        let values = l.split('-').collect::<Vec<_>>();
        adj_list
            .entry(values[0].to_string())
            .or_insert(vec![])
            .push(values[1].to_string());
        adj_list
            .entry(values[1].to_string())
            .or_insert(vec![])
            .push(values[0].to_string());
    });

    let mut path_num = 0;
    let mut q = VecDeque::new();
    q.push_back((String::from("start"), HashSet::new()));
    while !q.is_empty() {
        let (n, v) = q.pop_front().unwrap();
        for n in &adj_list[&n] {
            match n {
                n if n == "end" => path_num += 1,
                n if n == "start" => { /* do nothing */ }
                // 小文字のノードの場合
                n if n.chars().all(char::is_lowercase) => {
                    if !v.contains(&n) {
                        // 自分を追加したhashsetを作成 (経路保存みたいな感じか)
                        let mut v = v.clone();
                        v.insert(n);
                        q.push_back((n.to_string(), v));
                    }
                }
                n => q.push_back((n.to_string(), v.clone())),
            }
        }
    }

    println!("part1 ans: {:?}", path_num);
}

// https://github.com/rhysd/misc/blob/master/adventofcode/2021/12_2.rs
fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    reader.lines().for_each(|l| {
        let l = l.unwrap();
        let values = l.split('-').collect::<Vec<_>>();
        adj_list
            .entry(values[0].to_string())
            .or_insert(vec![])
            .push(values[1].to_string());
        adj_list
            .entry(values[1].to_string())
            .or_insert(vec![])
            .push(values[0].to_string());
    });

    let mut path_num = 0;
    let mut q = VecDeque::new();
    q.push_back((String::from("start"), HashMap::new()));
    while !q.is_empty() {
        let (n, v) = q.pop_front().unwrap();
        for n in &adj_list[&n] {
            match n {
                n if n == "end" => path_num += 1,
                n if n == "start" => { /* do nothing */ }
                // 小文字のノードの場合
                n if n.chars().all(char::is_lowercase) => {
                    let mut new_v = v.clone();
                    let flag = new_v.values().all(|&v| v <= 1);
                    let map_value = new_v.entry(n).or_insert(0);
                    if map_value == &0 {
                        *map_value += 1;
                        q.push_back((n.to_string(), new_v));
                    } else if map_value == &1 && flag {
                        // do nothing
                        *map_value += 1;
                        q.push_back((n.to_string(), new_v));
                    }
                }
                n => q.push_back((n.to_string(), v.clone())),
            }
        }
    }

    println!("part2 ans: {:?}", path_num);
}
