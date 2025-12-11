use std::{fs};

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
    let result = calc_3d_dist(&points[0], &points[1]);

    println!("Distance: {:?}", result);
}

fn calc_3d_dist(p1: &Point, p2: &Point) -> f32 {

    let dist = ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0) + (p2.z - p1.z).powf(2.0)).sqrt();

    dist

}