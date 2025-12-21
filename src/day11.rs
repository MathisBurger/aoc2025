use std::collections::{HashMap, HashSet};

pub fn solve(input: String) {
    let nodes: HashMap<String, Vec<String>> = parse_input(input);
    let mut set = HashSet::new();
    set.insert("you".to_string());

    let mut path_count: u32 = 0;

    solve_with_dfs(&nodes, &"you".to_string(), set, &mut path_count);

    println!("Part 1: {}", path_count);

    println!("Part 2: {}", solve2(&nodes));
}

fn solve2(nodes: &HashMap<String, Vec<String>>) -> u64 {
    let waypoint1 = "fft";
    let waypoint2 = "dac";

    let mut memo = HashMap::new();
    count_paths_with_waypoints(
        nodes, "svr", "out", waypoint1, waypoint2, false, false, &mut memo,
    )
}

fn count_paths_with_waypoints(
    nodes: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    wp1: &str,
    wp2: &str,
    visited_wp1: bool,
    visited_wp2: bool,
    memo: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    // Check if we've visited both waypoints and reached the end
    if current == end {
        return if visited_wp1 && visited_wp2 { 1 } else { 0 };
    }

    // Check memo
    let key = (current.to_string(), visited_wp1, visited_wp2);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    // Update waypoint status
    let new_wp1 = visited_wp1 || current == wp1;
    let new_wp2 = visited_wp2 || current == wp2;

    let mut total = 0u64;

    if let Some(neighbors) = nodes.get(current) {
        for neighbor in neighbors {
            total +=
                count_paths_with_waypoints(nodes, neighbor, end, wp1, wp2, new_wp1, new_wp2, memo);
        }
    }

    memo.insert(key, total);
    total
}

fn solve_with_dfs(
    nodes: &HashMap<String, Vec<String>>,
    current: &String,
    visited: HashSet<String>,
    path_count: &mut u32,
) {
    if current == "out" {
        *path_count += 1;
        return;
    }

    if let Some(neighbors) = nodes.get(current) {
        let visitable_nodes: Vec<String> = neighbors
            .iter()
            .filter(|node| !visited.contains(*node))
            .cloned()
            .collect();

        for node in visitable_nodes {
            let mut new_visited = visited.clone();
            new_visited.insert(node.clone());
            solve_with_dfs(nodes, &node, new_visited, path_count);
        }
    }
}

fn parse_input(input: String) -> HashMap<String, Vec<String>> {
    let mut node_map = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (name, connections) = line.split_once(": ").unwrap();
        let conn_vec: Vec<String> = connections.split(" ").map(str::to_string).collect();
        node_map.insert(name.to_string(), conn_vec);
    }

    node_map
}
