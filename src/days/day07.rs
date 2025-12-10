use std::{collections::HashMap, fs};

pub fn day_7() {
    let file_path = "inputs/day_7_input.txt";
    let file_path = "inputs/day_7_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");
    let grid: Vec<Vec<char>> = context.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start_col = grid[0].iter().position(|&c| c == 'S').expect("No 'S' found error!");

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    
    
}


