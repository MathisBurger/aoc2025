pub fn solve(input: String) {
    let mut matrix: Vec<Vec<bool>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| x.chars().map(|c| c == '@').collect())
        .collect();

    let (part_1, _) = solve_once(&matrix);
    println!("Part 1: {}", part_1);

    let mut sum = 0;

    loop {
        let (sum_loc, moved) = solve_once(&matrix);
        if sum_loc == 0 {
            break;
        }

        sum += sum_loc;

        for moved_item in moved {
            if let Some(row_vec) = matrix.get_mut(moved_item.0) {
                if let Some(cell) = row_vec.get_mut(moved_item.1) {
                    *cell = false;
                }
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn solve_once(matrix: &Vec<Vec<bool>>) -> (usize, Vec<(usize, usize)>) {
    let max_height = matrix.len();
    let max_width = matrix.get(0).unwrap().len();

    let mut sum = 0;
    let mut moved: Vec<(usize, usize)> = vec![];

    for x in 0..max_height {
        for y in 0..max_width {
            if get_field(&matrix, x, y, 0, 0) && is_moveable(&matrix, x, y) {
                sum += 1;
                moved.push((x, y));
            }
        }
    }

    (sum, moved)
}

fn is_moveable(matrix: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let mut obstructions = 0;

    if get_field(matrix, x, y, -1, -1) {
        obstructions += 1
    }
    if get_field(matrix, x, y, -1, -0) {
        obstructions += 1
    }
    if get_field(matrix, x, y, -1, 1) {
        obstructions += 1
    }
    if get_field(matrix, x, y, 0, -1) {
        obstructions += 1
    }
    if get_field(matrix, x, y, 0, 1) {
        obstructions += 1
    }
    if get_field(matrix, x, y, 1, -1) {
        obstructions += 1
    }
    if get_field(matrix, x, y, 1, 0) {
        obstructions += 1
    }
    if get_field(matrix, x, y, 1, 1) {
        obstructions += 1
    }

    return obstructions < 4;
}

fn get_field(matrix: &Vec<Vec<bool>>, x: usize, y: usize, dir_x: isize, dir_y: isize) -> bool {
    let max_height = matrix.len() as isize;
    let max_width = matrix.get(x).unwrap().len() as isize;

    let new_x: isize = x as isize + dir_x;
    let new_y: isize = y as isize + dir_y;

    // Top or down there are no obfuscations
    if (new_x < 0) || (new_x >= max_height) {
        return false;
    }

    if (new_y < 0) || (new_y >= max_width) {
        return false;
    }

    return *matrix
        .get(new_x as usize)
        .unwrap()
        .get(new_y as usize)
        .unwrap();
}
