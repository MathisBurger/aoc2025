use std::{collections::HashSet, usize};

#[derive(Debug)]
struct Maschine {
    pub lamps: Vec<bool>,
    pub buttons: Vec<HashSet<usize>>,
    pub joltage_req: Vec<usize>,
}

impl Maschine {
    pub fn solve(&self) -> Option<usize> {
        let n_lamps = self.lamps.len();
        let n_buttons = self.buttons.len();

        // Build matrix with gaussian
        let mut aug = vec![vec![0u8; n_buttons + 1]; n_lamps];

        for (button_idx, lamp_set) in self.buttons.iter().enumerate() {
            for &lamp_idx in lamp_set {
                if lamp_idx < n_lamps {
                    aug[lamp_idx][button_idx] = 1;
                }
            }
        }

        // Fill the target from the lamps
        for (i, &val) in self.lamps.iter().enumerate() {
            aug[i][n_buttons] = if val { 1 } else { 0 };
        }

        // Forward eliminate using gaussian
        let mut pivot_row = 0;
        let mut pivot_cols = Vec::new();

        for col in 0..n_buttons {
            let mut found_pivot = false;
            for row in pivot_row..n_lamps {
                if aug[row][col] == 1 {
                    aug.swap(pivot_row, row);
                    found_pivot = true;
                    break;
                }
            }

            if !found_pivot {
                continue;
            }

            pivot_cols.push(col);

            for row in 0..n_lamps {
                if row != pivot_row && aug[row][col] == 1 {
                    for c in 0..=n_buttons {
                        aug[row][c] ^= aug[pivot_row][c];
                    }
                }
            }

            pivot_row += 1;
        }

        // Check for inconsistencies just to verify the functionality of my code.
        for row in pivot_row..n_lamps {
            if aug[row][n_buttons] == 1 {
                return None;
            }
        }

        let mut solution = vec![0u8; n_buttons];

        for &col in pivot_cols.iter().rev() {
            let row = pivot_cols.iter().position(|&c| c == col).unwrap();
            solution[col] = aug[row][n_buttons];

            for j in (col + 1)..n_buttons {
                solution[col] ^= aug[row][j] & solution[j];
            }
        }

        let mut free_vars = Vec::new();
        for col in 0..n_buttons {
            if !pivot_cols.contains(&col) {
                free_vars.push(col);
            }
        }

        let mut min_weight: usize = solution.iter().map(|&x| x as usize).sum();
        let mut best_solution = solution.clone();

        let num_combinations = 1 << free_vars.len();
        for mask in 0..num_combinations {
            let mut test_solution = solution.clone();

            for (i, &var) in free_vars.iter().enumerate() {
                if (mask & (1 << i)) != 0 {
                    test_solution[var] = 1;

                    for (row_idx, &col_idx) in pivot_cols.iter().enumerate() {
                        if aug[row_idx][var] == 1 {
                            test_solution[col_idx] ^= 1;
                        }
                    }
                }
            }

            let weight = test_solution.iter().map(|&x| x as usize).sum();
            if weight < min_weight {
                min_weight = weight;
                best_solution = test_solution;
            }
        }

        // Extract pressed buttons

        Some(Maschine::find_pressed_buttons(&best_solution).len())
    }

    fn find_pressed_buttons(solution: &Vec<u8>) -> Vec<usize> {
        solution
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| if val == 1 { Some(i) } else { None })
            .collect()
    }
}

pub fn solve(input: String) {
    let maschines = parse_input(input);

    let mut sum = 0;
    let mut sum2 = 0;

    for masch in maschines {
        if let Some(min) = masch.solve() {
            sum += min;
        }
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}

fn parse_input(input: String) -> Vec<Maschine> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();

            Maschine {
                lamps: parts
                    .iter()
                    .find(|p| p.starts_with("["))
                    .unwrap()
                    .chars()
                    .filter(|c| *c != '[' && *c != ']')
                    .map(|c| c == '#')
                    .collect(),
                buttons: parts
                    .iter()
                    .filter(|p| p.starts_with("("))
                    .map(|p| {
                        p.replace("(", "")
                            .replace(")", "")
                            .split(",")
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect()
                    })
                    .collect(),
                joltage_req: parts
                    .iter()
                    .find(|p| p.starts_with("{"))
                    .unwrap()
                    .replace("{", "")
                    .replace("}", "")
                    .split(",")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect()
}
