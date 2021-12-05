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
    let mut mark_numbers = vec![];
    let mut boards: Vec<Vec<Vec<usize>>> = vec![];
    let mut matrix: Vec<Vec<usize>> = vec![];
    for (index, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        if index == 0 {
            for value in line_str.split(",").into_iter() {
                mark_numbers.push(value.parse::<usize>().unwrap());
            }
            continue;
        }

        if index > 1 && line_str != "" {
            let matrix_line = &line_str
                .split(" ")
                .filter(|x| x != &"")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            matrix.push(matrix_line.clone());
            if matrix.len() == matrix_line.len() {
                boards.push(matrix.clone());
                matrix = vec![];
            }
        }
    }

    // 判定用の配列の初期化
    let matrix_num = boards.len();
    let matrix_size = boards[0].len();
    let mut judge_boards: Vec<Vec<Vec<usize>>> = vec![];
    for _ in 0..matrix_num {
        let mut judge_matrix = vec![];
        for _ in 0..matrix_size {
            judge_matrix.push(vec![0usize; matrix_size]);
        }
        judge_boards.push(judge_matrix);
    }

    for number in mark_numbers {
        for h in 0..matrix_num {
            for i in 0..matrix_size {
                for j in 0..matrix_size {
                    if boards[h][i][j] == number {
                        judge_boards[h][i][j] = 1;
                    }
                }
            }
        }

        // ビンゴ判定
        for h in 0..matrix_num {
            let mut row_sum = vec![0usize; matrix_size];
            for (index, row) in judge_boards[h].iter().enumerate() {
                row_sum[index] = row.iter().sum();
            }
            let row_bingo_judge = row_sum.iter().any(|x| x == &matrix_size);

            let mut col_sum = vec![0usize; matrix_size];
            for row in judge_boards[h].iter() {
                for (index, col) in row.iter().enumerate() {
                    col_sum[index] += col;
                }
            }
            let col_bingo_judge = col_sum.iter().any(|x| x == &matrix_size);

            if row_bingo_judge || col_bingo_judge {
                let mut unmarked_num = 0;
                for i in 0..matrix_size {
                    for j in 0..matrix_size {
                        if judge_boards[h][i][j] == 0 {
                            unmarked_num += boards[h][i][j];
                        }
                    }
                }
                println!("part1 ans: {}", number * unmarked_num);
                return;
            }
        }
    }
}

fn part2_solver() {
    let file = File::open("input-part2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut mark_numbers = vec![];
    let mut boards: Vec<Vec<Vec<usize>>> = vec![];
    let mut matrix: Vec<Vec<usize>> = vec![];
    for (index, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        if index == 0 {
            for value in line_str.split(",").into_iter() {
                mark_numbers.push(value.parse::<usize>().unwrap());
            }
            continue;
        }

        if index > 1 && line_str != "" {
            let matrix_line = &line_str
                .split(" ")
                .filter(|x| x != &"")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            matrix.push(matrix_line.clone());
            if matrix.len() == matrix_line.len() {
                boards.push(matrix.clone());
                matrix = vec![];
            }
        }
    }

    // 判定用の配列の初期化
    let init_matrix_num = boards.len();
    let matrix_size = boards[0].len();
    let mut judge_boards: Vec<Vec<Vec<usize>>> = vec![];
    for _ in 0..init_matrix_num {
        let mut judge_matrix = vec![];
        for _ in 0..matrix_size {
            judge_matrix.push(vec![0usize; matrix_size]);
        }
        judge_boards.push(judge_matrix);
    }

    for number in mark_numbers {
        let matrix_num = boards.len();

        for h in 0..matrix_num {
            for i in 0..matrix_size {
                for j in 0..matrix_size {
                    if boards[h][i][j] == number {
                        judge_boards[h][i][j] = 1;
                    }
                }
            }
        }

        // ビンゴ判定
        let mut bingo_matrix_index = vec![];
        for h in 0..matrix_num {
            let mut row_sum = vec![0usize; matrix_size];
            for (index, row) in judge_boards[h].iter().enumerate() {
                row_sum[index] = row.iter().sum();
            }
            let row_bingo_judge = row_sum.iter().any(|x| x == &matrix_size);

            let mut col_sum = vec![0usize; matrix_size];
            for row in judge_boards[h].iter() {
                for (index, col) in row.iter().enumerate() {
                    col_sum[index] += col;
                }
            }
            let col_bingo_judge = col_sum.iter().any(|x| x == &matrix_size);

            if row_bingo_judge || col_bingo_judge {
                if matrix_num > 1 {
                    bingo_matrix_index.push(h);
                } else {
                    let mut unmarked_num = 0;
                    for i in 0..matrix_size {
                        for j in 0..matrix_size {
                            if judge_boards[h][i][j] == 0 {
                                unmarked_num += boards[h][i][j];
                            }
                        }
                    }
                    println!("part2 ans: {}", number * unmarked_num);
                    return;
                }
            }
        }

        // ビンゴマトリクスの削除
        boards = remove_by_ids(boards, &bingo_matrix_index);
        judge_boards = remove_by_ids(judge_boards, &bingo_matrix_index);
    }
}

fn remove_by_ids(arr: Vec<Vec<Vec<usize>>>, ids: &Vec<usize>) -> Vec<Vec<Vec<usize>>> {
    return arr
        .into_iter()
        .enumerate()
        .filter(|(index, _)| !ids.contains(index))
        .map(|(_, val)| val)
        .collect();
}
