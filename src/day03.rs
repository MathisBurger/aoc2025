pub fn solve(input: String) {
    let banks: Vec<Vec<u8>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|bank| {
            bank.chars()
                .map(|num| num.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let mut sum: u64 = 0;
    let mut sum2: u64 = 0;

    for bank in banks {
        sum += find_combined_joltage(&bank, 2);
        sum2 += find_combined_joltage(&bank, 12);
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}

fn find_combined_joltage(bank: &Vec<u8>, max_amount_batteries: usize) -> u64 {
    let mut sum: u64 = 0;
    let mut battery_count = max_amount_batteries;
    let mut current_index = 0;

    while battery_count > 0 {
        let (initial_index, initial) =
            find_largest_joltage(&bank, current_index, bank.len() - battery_count + 1);

        current_index = initial_index + 1;
        sum += initial as u64 * u64::pow(10, battery_count as u32 - 1);
        battery_count -= 1;
    }

    sum
}

fn find_largest_joltage(bank: &Vec<u8>, start_index: usize, last_index: usize) -> (usize, u8) {
    let mut largest: u8 = 0;
    let mut largest_index = 0;

    for index in start_index..last_index {
        if *bank.get(index).unwrap() > largest {
            largest = *bank.get(index).unwrap();
            largest_index = index;
        }
    }

    (largest_index, largest)
}
