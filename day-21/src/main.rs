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

    let mut pos = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            line.split(" ").last().unwrap().parse::<u32>().unwrap()
        })
        .collect::<Vec<u32>>();

    let mut cnt = 0;
    let dice = (1..101).collect::<Vec<u32>>();
    let mut score = vec![0u32, 0u32];
    while score.iter().max().unwrap() < &1000 {
        // update player 1
        let player_index = cnt % 2;
        let mut sum = pos[player_index];
        for i in cnt * 3..cnt * 3 + 3 {
            sum += dice[i % 100] % 100;
        }

        if sum % 10 == 0 {
            score[player_index] += 10;
            pos[player_index] = 10;
        } else {
            score[player_index] += sum % 10;
            pos[player_index] = sum % 10;
        }

        cnt += 1;
    }

    println!(
        "part1 ans: {:?}",
        score.iter().min().unwrap() * cnt as u32 * 3
    );
}

// see: https://github.com/jeffomatic/adventofcode/blob/main/2021-rust/day21b/src/main.rs
// 1, 2, 3 のいずれかが出る３面サイコロを3個ふった時に、それらの合計の取りうる値と頻度
// 1 + 1 + 1 = 3
// 1 + 2 + 1 = 4
// 1 + 3 + 1 = 5
// 1 + 2 + 2 = 5
// 1 + 2 + 3 = 6
// 1 + 3 + 2 = 6
// 1 + 3 + 3 = 7
// ....
const HISTOGRAM: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    p1_pos: u8,
    p1_score: u8,
    p2_pos: u8,
    p2_score: u8,
    p1_turn: bool,
}

fn count_wins(state: State, cache: &mut HashMap<State, (i64, i64)>) -> (i64, i64) {
    if state.p1_score >= 21 {
        return (1, 0);
    }

    if state.p2_score >= 21 {
        return (0, 1);
    }

    let mut total = (0, 0);
    for &(steps, freq) in HISTOGRAM.iter() {
        let mut next_state = state;
        if state.p1_turn {
            next_state.p1_pos = (state.p1_pos + steps - 1) % 10 + 1;
            next_state.p1_score += next_state.p1_pos;
            next_state.p1_turn = false;
        } else {
            next_state.p2_pos = (state.p2_pos + steps - 1) % 10 + 1;
            next_state.p2_score += next_state.p2_pos;
            next_state.p1_turn = true;
        }

        let (p1_wins, p2_wins) = match cache.get(&next_state) {
            Some(&res) => res,
            None => {
                // 勝負がつくまで繰り返す
                let res = count_wins(next_state, cache);
                cache.insert(next_state, res);
                res
            }
        };

        total.0 += p1_wins * freq as i64;
        total.1 += p2_wins * freq as i64;
    }

    total
}

// 問題の意図が理解できなかった...
fn part2_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);

    let pos = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            line.split(" ").last().unwrap().parse::<u32>().unwrap()
        })
        .collect::<Vec<u32>>();

    let (p1_wins, p2_wins) = count_wins(
        State {
            p1_pos: pos[0] as u8,
            p1_score: 0,
            p2_pos: pos[1] as u8,
            p2_score: 0,
            p1_turn: true,
        },
        &mut HashMap::new(),
    );
    println!("part 2 ans : {}", p1_wins.max(p2_wins));
}
