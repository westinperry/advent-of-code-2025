use std::{fs};

pub fn day_9() {
    let file_path = "inputs/day_9_input.txt";

    let input = fs::read_to_string(file_path).expect("Error reading file");
    
    let parsed_cord_vec: Vec<Point> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').expect("Invalid format");
            Point {
                x: x_str.trim().parse().expect("Error X"),
                y: y_str.trim().parse().expect("Error Y"),
            }
        })
        .collect();
    
    let mut max:i64 = 0;

    for i in 0..parsed_cord_vec.len() {
        for j in i+1..parsed_cord_vec.len() {
            if is_valid_rect(&parsed_cord_vec[i], &parsed_cord_vec[j], &parsed_cord_vec) {
                let result = calc_2d_dist(&parsed_cord_vec[i], &parsed_cord_vec[j]);
                if result > max {
                    max = result;
                }
            }
        }
    }

    println!("Max area: {max}");

}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}

fn calc_2d_dist(p1: &Point, p2: &Point) -> i64 {
    let x = (p1.x - p2.x).abs() + 1;
    let y = (p1.y - p2.y).abs() + 1;

    let area = x * y;

    area

}

fn is_valid_rect(p1: &Point, p2: &Point, poly: &Vec<Point>) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    let n = poly.len();

    // Check if any "Wall" cuts strictly through the rectangle
    for i in 0..n {
        let u = &poly[i];
        let v = &poly[(i + 1) % n];

        if u.x == v.x { // Vertical Wall
            let wall_x = u.x;
            let wall_min_y = u.y.min(v.y);
            let wall_max_y = u.y.max(v.y);
            
            // If wall is strictly between X bounds and overlaps Y bounds
            if wall_x > min_x && wall_x < max_x {
                if wall_max_y > min_y && wall_min_y < max_y { return false; }
            }
        } else { // Horizontal Wall
            let wall_y = u.y;
            let wall_min_x = u.x.min(v.x);
            let wall_max_x = u.x.max(v.x);

            // If wall is strictly between Y bounds and overlaps X bounds
            if wall_y > min_y && wall_y < max_y {
                if wall_max_x > min_x && wall_min_x < max_x { return false; }
            }
        }
    }

    // 2. Ray Casting: Check if the center is inside
    let center_x = (min_x + max_x) as f64 / 2.0;
    let center_y = (min_y + max_y) as f64 / 2.0 + 0.01; 
    let mut intersections = 0;

    for i in 0..n {
        let u = &poly[i];
        let v = &poly[(i + 1) % n];

        // Ray cast to the right. Only Vertical edges can block a horizontal ray.
        if u.x == v.x {
            let vy_min = u.y.min(v.y) as f64;
            let vy_max = u.y.max(v.y) as f64;

            // Does the ray pass through this edge's Y range?
            if center_y > vy_min && center_y < vy_max {
                // Is the edge to the right of our center?
                if (u.x as f64) > center_x {
                    intersections += 1;
                }
            }
        }
    }

    // Odd number of intersections means we are Inside
    intersections % 2 != 0
}