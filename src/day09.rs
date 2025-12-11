#[derive(Debug, Clone)]
struct HLine {
    y: i64,
    xmin: i64,
    xmax: i64,
}

#[derive(Debug, Clone)]
struct VLine {
    x: i64,
    ymin: i64,
    ymax: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Candidate {
    y: i64,
    xmin: i64,
    xmax: i64,
    points: Vec<(i64, i64)>,
}

pub fn solve(input: String) {
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let split = l.split_once(",").unwrap();
            (
                split.0.parse::<i64>().unwrap(),
                split.1.parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut largest: i64 = 0;
    for tile in tiles.iter() {
        for tile2 in tiles.iter() {
            let size = ((tile.0 - tile2.0).abs() + 1) * ((tile.1 - tile2.1).abs() + 1);
            if size > largest {
                largest = size;
            }
        }
    }
    println!("Part 1: {}", largest);

    let max_rect = find_max_inscribed_rectangle(&tiles);
    println!("Part 2: {}", max_rect);
}

fn find_max_inscribed_rectangle(polygon: &[(i64, i64)]) -> i64 {
    if polygon.len() < 3 {
        return 0;
    }

    // Create line segments from consecutive points
    let mut lines: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        lines.push((a, b));
    }

    // Separate horizontal and vertical lines
    let mut horizontals: Vec<HLine> = lines
        .iter()
        .filter(|((_, y1), (_, y2))| y1 == y2)
        .map(|((x1, y1), (x2, _))| HLine {
            y: *y1,
            xmin: *x1.min(x2),
            xmax: *x1.max(x2),
        })
        .collect();
    horizontals.sort_by_key(|l| l.y);

    let mut verticals: Vec<VLine> = lines
        .iter()
        .filter(|((x1, _), (x2, _))| x1 == x2)
        .map(|((x1, y1), (_, y2))| VLine {
            x: *x1,
            ymin: *y1.min(y2),
            ymax: *y1.max(y2),
        })
        .collect();
    verticals.sort_by_key(|v| v.x);

    // Sweep from bottom to top
    let mut active_candidates: Vec<Candidate> = Vec::new();
    let mut result = 0_i64;

    while let Some(line) = horizontals.pop() {
        let new = Candidate {
            y: line.y,
            xmin: line.xmin,
            xmax: line.xmax,
            points: vec![(line.xmin, line.y), (line.xmax, line.y)],
        };
        let mut to_add = Vec::new();

        let extended_left: Vec<&Candidate> = active_candidates
            .iter()
            .filter(|cand| cand.xmin == new.xmax)
            .collect();

        for cand in &extended_left {
            if cand.points.contains(&(cand.xmax, cand.y)) {
                let area = calc_area((new.xmax, new.y), (cand.xmax, cand.y));
                result = result.max(area);
            }
        }

        if !extended_left.is_empty() {
            if let Some(block_right) = verticals
                .iter()
                .filter(|vline| vline.x > new.xmax && vline.ymin <= new.y && new.y <= vline.ymax)
                .min_by_key(|vline| vline.x)
            {
                to_add.push(Candidate {
                    y: new.y,
                    xmin: new.xmin,
                    xmax: block_right.x,
                    points: vec![(new.xmin, new.y)],
                });
            }
        }

        let extended_right: Vec<&Candidate> = active_candidates
            .iter()
            .filter(|cand| cand.xmax == new.xmin)
            .collect();

        for cand in &extended_right {
            if cand.points.contains(&(cand.xmin, cand.y)) {
                let area = calc_area((new.xmin, new.y), (cand.xmin, cand.y));
                result = result.max(area);
            }
        }

        if !extended_right.is_empty() {
            if let Some(block_left) = verticals
                .iter()
                .filter(|vline| vline.x < new.xmin && vline.ymin <= new.y && new.y <= vline.ymax)
                .max_by_key(|vline| vline.x)
            {
                to_add.push(Candidate {
                    y: new.y,
                    xmin: block_left.x,
                    xmax: new.xmax,
                    points: vec![(new.xmax, new.y)],
                });
            }
        }

        let mut to_remove: Vec<Candidate> = Vec::new();

        let swallowed: Vec<Candidate> = active_candidates
            .iter()
            .filter(|cand| new.xmin <= cand.xmin && cand.xmax <= new.xmax)
            .cloned()
            .collect();
        to_remove.extend(swallowed);

        let overlapped: Vec<Candidate> = active_candidates
            .iter()
            .filter(|cand| inside_excl(cand, new.xmin) || inside_excl(cand, new.xmax))
            .cloned()
            .collect();

        let no_overlaps = overlapped.is_empty();

        for cand in overlapped {
            for &(x1, y1) in &cand.points {
                if inside_incl(&cand, new.xmin) {
                    let area = calc_area((x1, y1), (new.xmin, new.y));
                    result = result.max(area);
                }
                if inside_incl(&cand, new.xmax) {
                    let area = calc_area((x1, y1), (new.xmax, new.y));
                    result = result.max(area);
                }
            }

            if inside_excl(&cand, new.xmin) && cand.points.contains(&(cand.xmin, cand.y)) {
                to_add.push(Candidate {
                    y: cand.y,
                    xmin: cand.xmin,
                    xmax: new.xmin,
                    points: vec![(cand.xmin, cand.y)],
                });
                to_add.push(Candidate {
                    y: new.y,
                    xmin: cand.xmin,
                    xmax: new.xmin,
                    points: vec![(new.xmin, new.y)],
                });
            }

            if inside_excl(&cand, new.xmax) && cand.points.contains(&(cand.xmax, cand.y)) {
                to_add.push(Candidate {
                    y: cand.y,
                    xmin: new.xmax,
                    xmax: cand.xmax,
                    points: vec![(cand.xmax, cand.y)],
                });
                to_add.push(Candidate {
                    y: new.y,
                    xmin: new.xmax,
                    xmax: cand.xmax,
                    points: vec![(new.xmax, new.y)],
                });
            }

            to_remove.push(cand);
        }

        if no_overlaps {
            to_add.push(new);
        }

        active_candidates.retain(|candidate| !to_remove.contains(candidate));
        active_candidates.extend(to_add);
    }

    result
}

fn calc_area((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
}

fn inside_excl(cand: &Candidate, x: i64) -> bool {
    cand.xmin < x && x < cand.xmax
}

fn inside_incl(cand: &Candidate, x: i64) -> bool {
    cand.xmin <= x && x <= cand.xmax
}
