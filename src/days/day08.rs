use std::{collections::HashMap, fs};

struct Point {
    x: f32,
    y: f32,
    z: f32
}

pub fn day_8() {
    //let file_path = "inputs/day_8_input.txt";
    let file_path = "inputs/day_8_input_test.txt";
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


}



fn calc_3d_dist(p1: &Point, p2: &Point) -> usize {

    let dist = ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0) + (p2.z - p1.z).powf(2.0)).sqrt();

    dist as usize

}