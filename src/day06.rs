pub fn solve(input: String) {
    let ops = get_operations(input.clone());

    let mut sum: u64 = 0;

    for op in ops {
        if op.1 == '+' {
            sum += op.0.into_iter().sum::<u64>();
        } else if op.1 == '*' {
            sum += op.0.into_iter().product::<u64>();
        }
    }

    println!("Part 1: {}", sum);

    let ops2 = get_operations2(input);

    let mut sum2: u64 = 0;

    for op in ops2 {
        if op.1 == '+' {
            sum2 += op.0.into_iter().sum::<u64>();
        } else if op.1 == '*' {
            sum2 += op.0.into_iter().product::<u64>();
        }
    }

    println!("Part 2: {}", sum2);
}

fn get_operations(input: String) -> Vec<(Vec<u64>, char)> {
    let lines: Vec<&str> = input.lines().collect();

    let op_line = lines.last().unwrap();
    let ops: Vec<char> = op_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let mut columns: Vec<Vec<u64>> = vec![vec![]; ops.len()];

    for line in &lines[..lines.len() - 1] {
        for (col, num_str) in line.split_whitespace().enumerate() {
            columns[col].push(num_str.parse::<u64>().unwrap());
        }
    }
    ops.into_iter()
        .zip(columns.into_iter())
        .map(|(op, nums)| (nums, op))
        .collect()
}

fn get_operations2(input: String) -> Vec<(Vec<u64>, char)> {
    let mut ops = vec![];

    let lines: Vec<&str> = input.lines().collect();

    let mut index = lines.iter().map(|n| n.len() - 1).max().unwrap_or(0);

    loop {
        let mut operator: char = ' ';
        let mut nums = vec![];

        loop {
            let mut line = get_line(&lines, index);

            if line.trim().is_empty() {
                break;
            }

            if line.ends_with("*") {
                operator = '*';
                line.pop();
            } else if line.ends_with("+") {
                operator = '+';
                line.pop();
            }

            nums.push(line.trim().parse::<u64>().unwrap());

            if index == 0 {
                break;
            }

            index -= 1;
        }

        ops.push((nums, operator));

        if index == 0 {
            break;
        }

        index -= 1;
    }

    ops
}

fn get_line(lines: &Vec<&str>, idx: usize) -> String {
    let mut string = String::new();

    for line in lines.into_iter() {
        string.push(line.chars().nth(idx).unwrap_or(' '));
    }
    string
}
