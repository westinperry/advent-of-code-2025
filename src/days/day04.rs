use std::fs;


pub fn day_4() {
    //let file_path = "inputs/day_4_input.txt";
    let file_path = "inputs/day_4_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");

    // Creates a grid that of the input file.
    // (collect chars of each line, second collect - collects all vectors)
    let grid: Vec<Vec<char>> = context
        .lines()
        .map(|line| line.chars().collect())
        .collect();


    println!("{:?}", grid)
}