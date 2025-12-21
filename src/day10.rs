use std::{collections::HashSet, usize};

#[derive(Debug)]
struct Maschine {
    pub lamps: Vec<bool>,
    pub buttons: Vec<HashSet<usize>>,
    pub joltage_req: Vec<usize>,
}

#[derive(Clone)]
struct ButtonCombination {
    counter: Vec<usize>,
    nb_pressed_buttons: usize,
}

trait Counter {
    fn is_zero(&self) -> bool;
    fn smaller_or_equal(&self, other: &[usize]) -> bool;
    fn equals_modulo2(&self, other: &[usize]) -> bool;
    fn solve2(&self, combinations: &[ButtonCombination]) -> Option<usize>;
}

impl Counter for Vec<usize> {
    fn is_zero(&self) -> bool {
        self.iter().all(|&x| x == 0)
    }

    fn smaller_or_equal(&self, other: &[usize]) -> bool {
        self.iter().zip(other).all(|(&a, &b)| a <= b)
    }

    fn equals_modulo2(&self, other: &[usize]) -> bool {
        self.iter().zip(other).all(|(&a, &b)| a % 2 == b % 2)
    }

    fn solve2(&self, combinations: &[ButtonCombination]) -> Option<usize> {
        if self.is_zero() {
            return Some(0);
        }

        let mut min_cost = None;

        for comb in combinations {
            if !comb.counter.smaller_or_equal(self) {
                continue;
            }
            if !comb.counter.equals_modulo2(self) {
                continue;
            }

            let next_counter: Vec<usize> = self
                .iter()
                .zip(&comb.counter)
                .map(|(&a, &b)| (a - b) / 2)
                .collect();

            if let Some(rec_cost) = next_counter.solve2(combinations) {
                let total_cost: usize = 2 * rec_cost + comb.nb_pressed_buttons;
                min_cost = Some(min_cost.map_or(total_cost, |c: usize| c.min(total_cost)));
            }
        }

        min_cost
    }
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

    pub fn solve_part2(&self) -> Option<usize> {
        let m = self.joltage_req.len();

        // Generate all button combinations
        let combinations = self.all_combinations(m);

        // Solve recursively
        self.joltage_req.clone().solve2(&combinations)
    }

    fn all_combinations(&self, m: usize) -> Vec<ButtonCombination> {
        let n_buttons = self.buttons.len();
        let num_combinations = 1 << n_buttons;
        let mut result = Vec::with_capacity(num_combinations);

        for mask in 0..num_combinations {
            let mut counter = vec![0; m];
            let mut nb_pressed = 0;

            for (j, button) in self.buttons.iter().enumerate() {
                if (mask & (1 << j)) != 0 {
                    nb_pressed += 1;
                    for &idx in button {
                        if idx < m {
                            counter[idx] += 1;
                        }
                    }
                }
            }

            result.push(ButtonCombination {
                counter,
                nb_pressed_buttons: nb_pressed,
            });
        }

        result
    }
}

pub fn solve(input: String) {
    let maschines = parse_input(input);

    let mut sum = 0;
    let mut sum2 = 0;

    for masch in maschines {
        //if let Some(min) = masch.solve() {
        //    sum += min;
        //}
        if let Some(min2) = masch.solve_part2() {
            sum2 += min2;
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
