use std::{collections::HashMap, fs};

pub fn day_11() {
    let content = fs::read_to_string("./inputs/day_11_input_test.txt")
        .expect("Error reading file");

    let graph = parse_graph(&content);

    let result = count_paths("you", &graph);

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

fn count_paths( current: &str, graph: &HashMap<String, Vec<String>>) -> usize {
    if current == "out" {
        return 1;
    }

    let mut total = 0;

    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            total += count_paths(next, graph);
        }
    }

    total
}
