use std::{collections::HashMap, collections::HashSet, fs};

pub fn day_11() {
    let content = fs::read_to_string("./inputs/day_11_input.txt")
        .expect("Error reading file");

    let graph = parse_graph(&content);

    let mut visited = HashSet::new();
    let mut memo = HashMap::new();

    let result = count_paths(
        "svr",
        &graph,
        false,
        false,
        &mut visited,
        &mut memo,
    );

    println!("{}", result);
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(":");
        let node = parts.next().unwrap().trim().to_string();

        let outputs = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        graph.insert(node, outputs);
    }

    graph
}

fn count_paths(
    current: &str,
    graph: &HashMap<String, Vec<String>>,
    seen_dac: bool,
    seen_fft: bool,
    visited: &mut HashSet<String>,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    // cycle guard (path-local)
    if visited.contains(current) {
        return 0;
    }

    let seen_dac = seen_dac || current == "dac";
    let seen_fft = seen_fft || current == "fft";

    let key = (current.to_string(), seen_dac, seen_fft);

    // memo hit
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    visited.insert(current.to_string());

    let result = if current == "out" {
        if seen_dac && seen_fft { 1 } else { 0 }
    } else {
        graph
            .get(current)
            .into_iter()
            .flat_map(|v| v.iter())
            .map(|next| {
                count_paths(next, graph, seen_dac, seen_fft, visited, memo)
            })
            .sum()
    };

    visited.remove(current);

    memo.insert(key, result);
    result
}

