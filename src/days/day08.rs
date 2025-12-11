use std::{collections::HashMap, fs, cmp::Reverse};

struct Point {
    x: f32,
    y: f32,
    z: f32
}

pub fn day_8() {
    let file_path = "inputs/day_8_input.txt";
    //let file_path = "inputs/day_8_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");
    let points: Vec<Point> = context
        .lines()
        .map(|line| {
            let coords: Vec<f32> = line
                .split(',')
                .map(|s| s.trim().parse().expect("Failed to parse number"))
                .collect();
            Point {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    
    // Adding all values to hashmap
    for p in 0..points.len() {
        for j in 0..points.len() {
            if p != j {

                let distance = calc_3d_dist(&points[p], &points[j]);

                // Add to Hashmap
                if p < j {
                    distances.insert((p, j), distance);
                } else {
                    distances.insert((j, p), distance);
                }

            }
        }
    }


    // for (key, value) in &distances {
    //     println!("{:?}: {:?}", key, value);
    // }


    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for _ in 0..1000 {
        // Find the smallest value in the HashMap
        let min_entry = distances
            .iter()
            .min_by_key(|(_, dist)| *dist)
            .map(|(k, _)| *k)
            .unwrap();
        
        // Remove it and get the keys
        distances.remove(&min_entry);
        let (key1, key2) = min_entry;
        
        // Needed help with merge logic
        // Check if either key exists in any circuit
        let mut circuit1_idx: Option<usize> = None;
        let mut circuit2_idx: Option<usize> = None;
        
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&key1) {
                circuit1_idx = Some(i);
            }
            if circuit.contains(&key2) {
                circuit2_idx = Some(i);
            }
        }

        match (circuit1_idx, circuit2_idx) {
            // Both in same circuit
            (Some(i), Some(j)) if i == j => {},
            // Both in different circuits
            (Some(i), Some(j)) => {
                let circuit2 = circuits.remove(j.max(i));
                let circuit1 = circuits.remove(j.min(i));
                let mut merged = circuit1;
                merged.extend(circuit2);
                circuits.push(merged);
            },
            // Only key1 is in a circuit - add key2 to it
            (Some(i), None) => {
                circuits[i].push(key2);
            },
            // Only key2 is in a circuit - add key1 to it
            (None, Some(j)) => {
                circuits[j].push(key1);
            },
            // Neither is in a circuit - create new one
            (None, None) => {
                circuits.push(vec![key1, key2]);
            }
        }

    }

    // Sort vector descending order
    circuits.sort_by_key(|circuit| Reverse(circuit.len()));

    // Get top three
    let top_three: Vec<&Vec<usize>> = circuits.iter().take(3).collect();
    
    // Get answer
    let mut result = 1;
    for i in 0..top_three.len() {
        result *= top_three[i].len();
    }


    println!("Max Len Circuits: {:?}", result);





}



fn calc_3d_dist(p1: &Point, p2: &Point) -> usize {

    let dist = ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0) + (p2.z - p1.z).powf(2.0)).sqrt();

    dist as usize

}

// 2535 too small