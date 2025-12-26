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
            let result = calc_2d_dist(&parsed_cord_vec[i], &parsed_cord_vec[j]);
            if result > max {
                max = result;
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

// 2535 too small