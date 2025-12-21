use std::collections::{HashMap, HashSet};

pub fn solve(input: String) {
    let nodes: HashMap<String, Vec<String>> = parse_input(input);
    let mut set = HashSet::new();
    set.insert("you".to_string());

    let mut path_count: u32 = 0;

    solve_with_dfs(&nodes, &"you".to_string(), set, &mut path_count);

    println!("Part 1: {}", path_count);
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

    let visitable_nodes: Vec<String> = nodes
        .get(current)
        .unwrap()
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
