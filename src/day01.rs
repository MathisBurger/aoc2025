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

    for turn in turns {
        match turn {
            ('L', steps) => pos -= steps,
            ('R', steps) => pos += steps,
            _ => {}
        }

        if pos < 0 || pos > 99 {
            pos %= 100;
        }
        if pos < 0 {
            pos = 100 + pos;
        }
        if pos == 0 {
            zero_count += 1;
        }
    }

    println!("Part 1: {}", zero_count);
}
