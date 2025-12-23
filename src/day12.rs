type Shape = usize;

#[derive(Debug)]
struct Region {
    pub width: usize,
    pub height: usize,
    pub shape_config: Vec<usize>,
}

pub fn solve(input: String) {
    let (shapes, regions) = parse_input(input);

    let part1_solution = regions
        .iter()
        .filter(|region| {
            let mut needed_space = 0;
            for (idx, shape_req) in region.shape_config.iter().enumerate() {
                let shape_size: usize = shapes[idx];
                needed_space += shape_size * *shape_req;
            }
            needed_space <= region.height * region.width
        })
        .count();

    println!("Part 1: {}", part1_solution);
}

fn parse_input(input: String) -> (Vec<Shape>, Vec<Region>) {
    let mut shapes = vec![];
    let mut regions = vec![];

    let mut shape = 0;

    for line in input.lines() {
        if line.is_empty() {
            shapes.push(shape);
            shape = 0;
            continue;
        }

        if line.len() == 2 {
            continue;
        }

        if line.contains("#") {
            shape += line.chars().filter(|c| *c == '#').count();
        }

        if line.contains("x") {
            let (size, config) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once("x").unwrap();

            regions.push(Region {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                shape_config: config
                    .split(" ")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect(),
            });
        }
    }

    (shapes, regions)
}
