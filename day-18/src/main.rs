use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // solve part1
    part1_solver();

    // solve part2
    part2_solver();
}

// see: https://github.com/jeffomatic/adventofcode/blob/main/2021-rust/day18a/src/main.rs

// 値と深さを持ったツリー
#[derive(Debug, Clone)]
struct VecTree {
    vals: Vec<u32>,
    depths: Vec<u32>,
}

impl VecTree {
    fn parse(s: &str) -> VecTree {
        let mut t = VecTree {
            vals: Vec::new(),
            depths: Vec::new(),
        };

        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ',' => (),
                ']' => {
                    depth -= 1;
                }
                d => {
                    t.vals.push(d.to_digit(10).unwrap());
                    t.depths.push(depth - 1);
                }
            }
        }

        t
    }

    // 爆発の操作
    fn explode(&mut self) -> bool {
        for i in 0..self.depths.len() {
            let depth = self.depths[i];
            // 足す前は必ず depth は４より小さいので、この判定で問題ない
            if depth != 4 {
                continue;
            }

            // add left value to left neighbor
            if i != 0 {
                self.vals[i - 1] += self.vals[i];
            }

            // add right value to right neighbor
            if i + 2 < self.vals.len() {
                self.vals[i + 2] += self.vals[i + 1];
            }

            self.vals[i] = 0;
            self.depths[i] = 3;
            self.vals.remove(i + 1);
            self.depths.remove(i + 1);

            return true;
        }

        false
    }

    // 分割の操作
    fn split(&mut self) -> bool {
        for i in 0..self.vals.len() {
            let v = self.vals[i];
            if v < 10 {
                continue;
            }

            let (a, b) = if v % 2 == 0 {
                (v / 2, v / 2)
            } else {
                (v / 2, v / 2 + 1)
            };

            self.vals[i] = a;
            self.depths[i] += 1;
            self.vals.insert(i + 1, b);
            self.depths.insert(i + 1, self.depths[i]);

            return true;
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn add(&mut self, other: &VecTree) {
        self.vals.extend(other.vals.iter());
        self.depths.extend(other.depths.iter());
        for i in 0..self.depths.len() {
            self.depths[i] += 1;
        }
    }

    fn score(&self) -> u32 {
        let mut vals = self.vals.clone();
        let mut depths = self.depths.clone();

        while vals.len() > 1 {
            for i in 0..depths.len() - 1 {
                if depths[i] == depths[i + 1] {
                    vals[i] = 3 * vals[i] + 2 * vals[i + 1];
                    vals.remove(i + 1);
                    depths.remove(i + 1);

                    if depths[i] > 0 {
                        depths[i] -= 1;
                    }

                    break;
                }
            }
        }

        vals[0]
    }
}

fn part1_solver() {
    let file = File::open("input-part1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let mut tree = VecTree::parse(&lines[0]);
    for line in &lines[1..] {
        tree.add(&VecTree::parse(line));
        tree.reduce();
    }

    println!("part 1: ans {}", tree.score());
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let mut scores = vec![];
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let mut tree = VecTree::parse(&lines[i]);
            tree.add(&VecTree::parse(&lines[j]));
            tree.reduce();
            scores.push(tree.score());
        }
    }

    println!("part 2: ans {}", scores.iter().max().unwrap());
}
