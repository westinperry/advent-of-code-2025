use std::{collections::HashMap, fs};

pub fn day_8() {
    let file_path = "inputs/day_7_input.txt";
    //let file_path = "inputs/day_7_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");
    let grid: Vec<Vec<char>> = context.lines().map(|line| line.chars().collect()).collect();


}
