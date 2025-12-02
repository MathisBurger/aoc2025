pub fn solve(input: String) {
    let ranges: Vec<(u64, u64)> = input
        .replace("\n", "")
        .split(",")
        .map(|s| {
            let mut parts = s.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let mut sum = 0;
    let mut sum2 = 0;

    for (first, last) in &ranges {
        let mut num = *first;
        while num <= *last {
            let str_num = num.to_string();
            if is_invalid(str_num.clone(), true) {
                sum += num;
            }
            if is_invalid(str_num, false) {
                sum2 += num;
            }
            num += 1;
        }
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}

fn is_invalid(num_str: String, fixed: bool) -> bool {
    let mut size = num_str.len() / 2;

    while size > 0 {
        if fixed && (size != num_str.len() / 2 || num_str.len() % 2 != 0) {
            return false;
        }
        if num_str.len() % size != 0 {
            size -= 1;
            continue;
        }
        let slice_count = num_str.len() / size;
        let initial_slice = &num_str[0..size];
        let mut equal = true;
        for slice_index in 1..slice_count {
            if initial_slice != &num_str[slice_index * size..(slice_index + 1) * size] {
                equal = false;
                break;
            }
        }
        if equal {
            return true;
        }
        size -= 1;
    }
    return false;
}
