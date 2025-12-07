use std::collections::HashMap;

pub fn solve(input: String) {
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| x.chars().collect())
        .collect();

    let part2_matrix = matrix.clone();

    solve_beams(&mut matrix);
    determine_reachable_paths(part2_matrix);
}

fn solve_beams(matrix: &mut Vec<Vec<char>>) {
    let mut num_splits = 0;

    for line_index in 0..matrix.len() {
        if line_index == matrix.len() - 1 {
            break;
        }

        let line = matrix.get(line_index).unwrap().clone();

        for col_index in 0..line.len() {
            if *line.get(col_index).unwrap() == '|' || *line.get(col_index).unwrap() == 'S' {
                let next_line = matrix.get_mut(line_index + 1).unwrap();
                if *next_line.get(col_index).unwrap() == '^' {
                    num_splits += 1;
                    if col_index > 0 {
                        let left_element = next_line.get_mut(col_index - 1).unwrap();
                        if *left_element != '|' {
                            //num_splits += 1;
                        }
                        *left_element = '|';
                    }
                    if col_index + 1 < next_line.len() {
                        let right_element = next_line.get_mut(col_index + 1).unwrap();
                        if *right_element != '|' {
                            //num_splits += 1;
                        }
                        *right_element = '|';
                    }
                } else {
                    *next_line.get_mut(col_index).unwrap() = '|';
                }
            }
        }
    }

    println!("Part 1: {}", num_splits);
}

fn determine_reachable_paths(matrix: Vec<Vec<char>>) {
    let start_col = matrix
        .get(0)
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, el)| el == &&'S')
        .unwrap()
        .0;
    let total = count_paths_fast(&matrix, start_col);

    println!("Part 2: {}", total);
}

fn count_paths_fast(matrix: &Vec<Vec<char>>, start_col: usize) -> u64 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // dp[row][col] = Anzahl der Pfade, die an (row, col) ankommen
    let mut dp_current: HashMap<usize, u64> = HashMap::new();
    dp_current.insert(start_col, 1);

    for row in 0..rows - 1 {
        let mut dp_next: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in dp_current.iter() {
            let next = matrix[row + 1][col];

            if next == '^' {
                if col > 0 {
                    *dp_next.entry(col - 1).or_insert(0) += count;
                }
                if col + 1 < cols {
                    *dp_next.entry(col + 1).or_insert(0) += count;
                }
            } else {
                *dp_next.entry(col).or_insert(0) += count;
            }
        }

        dp_current = dp_next;
    }
    dp_current.values().sum()
}
