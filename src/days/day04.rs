use std::{fs, ops::Index};


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

    // println!("{:?}", grid)
    let mut result_grid = grid.clone();
    let mut current_location = vec![0,0];
    let grid_width = grid[0].len();
    let grid_height = grid.len();

    //println!("{}, {}", grid_width, grid_height);

    for i in 0..grid_width {
        for j in 0..grid_height {
            // If current grid is @
            if grid[i][j] == '@' {
                let mut adjacenet_rolls = 0;


                if adjacenet_rolls < 4 {
                    result_grid[i][j] = 'x';
                } else {
                    result_grid[i][j] = '@';
                }
            }
            
        }
    }
   
}