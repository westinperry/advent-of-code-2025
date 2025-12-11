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

    for _ in 0..=1000 {
        // Find the smallest value in the HashMap
        let min_entry = distances
            .iter()
            .min_by_key(|(_, dist)| *dist)
            .map(|(k, _)| *k)
            .unwrap();
        
        // Remove it and get the keys
        distances.remove(&min_entry);
        let (key1, key2) = min_entry;
        
        // Check if either key exists in any circuit
        let mut found = false;

        for circuit in circuits.iter_mut() {
            if circuit.contains(&key1) {
                if !circuit.contains(&key2) {
                    circuit.push(key2);
                }
                found = true;
                break;
            } else if circuit.contains(&key2) {
                if !circuit.contains(&key1) {
                    circuit.push(key1);
                }
                found = true;
                break;
            }
        }
        
        // If not found in any circuit, create a new one
        if !found {
            circuits.push(vec![key1, key2]);
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