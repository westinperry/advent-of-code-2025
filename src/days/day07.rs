use std::fs;

pub fn day_7() {
    let file_path = "inputs/day_6_input.txt";
    //let file_path = "inputs/day_6_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");

    let lines: Vec<&str> = context.lines().collect();

    
    
}
