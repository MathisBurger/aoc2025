pub fn solve(input: String) {
    let turns: Vec<(char, i32)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut chars = l.chars();
            let first = chars.next().unwrap();
            let rest: String = chars.collect();
            (first, rest.parse::<i32>().unwrap())
        })
        .collect();

    let mut pos = 50;
    let mut zero_count = 0;
    let mut any_zero_count = 0;

    for turn in turns {
        let steps = match turn {
            ('L', steps) => -steps,
            ('R', steps) => steps,
            _ => 0,
        };
        let cleared_steps = steps % 100;
        any_zero_count += steps.abs() / 100;
        let pos_bef = pos;
        pos += cleared_steps;

        if pos < 0 {
            pos += 100;
        }
        if pos > 99 {
            pos %= 100;
        }
        if pos == 0 {
            zero_count += 1;
        }

        if pos - cleared_steps != pos_bef && pos_bef + cleared_steps != 100 && pos_bef != 0 {
            any_zero_count += 1;
        }
    }

    println!("Part 1: {}", zero_count);
    println!("Part 2: {}", zero_count + any_zero_count);
}
