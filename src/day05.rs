pub fn solve(input: String) {
    let (mut ranges, ids) = determine_ranges_and_ids(input);

    let mut fresh = 0;

    for id in ids {
        for range in ranges.clone() {
            if range.0 <= id && id <= range.1 {
                fresh += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", fresh);

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(usize, usize)> = Vec::new();

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let total: usize = merged.iter().map(|(s, e)| e - s + 1).sum();
    println!("Part 2: {}", total);
}

fn determine_ranges_and_ids(input: String) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges_done = false;

    let mut ranges = vec![];
    let mut ids = vec![];

    for line in input.lines() {
        if line.is_empty() {
            ranges_done = true;
            continue;
        }

        if ranges_done {
            ids.push(line.parse::<usize>().unwrap());
        } else {
            let (lft, rht) = line.split_once("-").unwrap();
            ranges.push((lft.parse::<usize>().unwrap(), rht.parse::<usize>().unwrap()));
        }
    }

    return (ranges, ids);
}
