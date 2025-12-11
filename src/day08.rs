use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type DistanceMap = Vec<(f64, JunctionBox, JunctionBox)>;

fn precalculate_distances(set: &HashSet<JunctionBox>) -> DistanceMap {
    let mut distances = vec![];
    let nodes: Vec<_> = set.iter().cloned().collect();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if i == j {
                continue;
            }
            let a = nodes[i];
            let b = nodes[j];
            let d = a.distance_to(&b);
            distances.push((d, a, b));
        }
    }
    distances.sort_by(|a, b| a.0.total_cmp(&b.0));
    distances
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct JunctionBox {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f64 {
        calculate_distance(self, other)
    }
}

pub fn solve(input: String) {
    let junction_boxes: HashSet<JunctionBox> = parse_input(&input);
    let mut mapping: HashMap<JunctionBox, usize> = HashMap::new();
    for (i, v) in junction_boxes.clone().iter().enumerate() {
        mapping.insert(v.clone(), i);
    }

    let distances: Vec<(f64, JunctionBox, JunctionBox)> = precalculate_distances(&junction_boxes);
    let mut connections_made = 0;
    for (_, a, b) in distances {
        //perform_combination(&mut mapping, &distances);
        let k1 = mapping.get(&a).unwrap().clone();
        let k2 = mapping.get(&b).unwrap().clone();

        if k1 != k2 {
            merge_circuits(&mut mapping, k1, k2);
        }
        connections_made += 1;

        if connections_made == 1000 {
            println!("Part 1: {}", get_largest_circuits_product(&mapping));
        }
        if connections_made > 1000 && get_largest_circuits_product(&mapping) == 1000 {
            println!("Part 2: {}", a.x * b.x);
            break;
        }
    }
}

fn get_largest_circuits_product(mapping: &HashMap<JunctionBox, usize>) -> u32 {
    let mut reverse_count_map: HashMap<usize, u32> = HashMap::new();

    for v in mapping.values() {
        if reverse_count_map.contains_key(v) {
            *reverse_count_map.get_mut(v).unwrap() += 1;
        } else {
            reverse_count_map.insert(v.clone(), 1);
        }
    }

    let mut values: Vec<u32> = reverse_count_map.values().cloned().collect();
    values.sort_by(|a, b| b.cmp(a));

    values.iter().take(3).product()
}
/*
fn perform_combination(mapping: &mut HashMap<JunctionBox, u32>, distances: &DistanceMap) {
    let result = shortest_in_set(mapping, distances);

    if let Some((_, el1, el2)) = result {
        let k1 = mapping.get(&el1).unwrap().clone();
        let k2 = mapping.get(&el2).unwrap().clone();

        merge_circuits(mapping, k1, k2);
    }
}
*/
fn merge_circuits(mapping: &mut HashMap<JunctionBox, usize>, k1: usize, k2: usize) {
    let target = k1.min(k2);
    let source = k1.max(k2);
    for value in mapping.values_mut() {
        if *value == source {
            *value = target;
        }
    }
}
/*
fn shortest_in_set(
    mapping: &HashMap<JunctionBox, u32>,
    distances: &DistanceMap,
) -> Option<(f64, JunctionBox, JunctionBox)> {
    for (f, a, b) in distances {
        if mapping.get(&a) == mapping.get(&b) {
            continue;
        } else {
            return Some((f.clone(), a.clone(), b.clone()));
        }
    }

    return None;
}
*/

fn calculate_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn parse_input(input: &String) -> HashSet<JunctionBox> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let spl: Vec<i32> = l.split(",").map(|el| el.parse::<i32>().unwrap()).collect();
            JunctionBox {
                x: spl[0],
                y: spl[1],
                z: spl[2],
            }
        })
        .collect()
}
