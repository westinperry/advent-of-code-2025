use std::{collections::HashMap, fs, hash::Hash};

pub fn day_11() {
    let content = fs::read_to_string("./inputs/day_11_input_test.txt")
        .expect("Error reading file");

    let graph = parse_graph(&content);

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

