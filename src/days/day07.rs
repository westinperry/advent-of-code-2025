use std::fs;

pub fn day_7() {
    let file_path = "inputs/day_7_input.txt";
    // let file_path = "inputs/day_7_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");
    let grid: Vec<Vec<char>> = context.lines().map(|line| line.chars().collect()).collect();

    let mut tachyon_beam: Vec<char> = vec!['.'; grid[0].len()];
    let mut split_counter: i32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                tachyon_beam[j] = '|';
            }

            if grid[i][j] == '^' && tachyon_beam[j] == '|' {
                tachyon_beam[j-1] = '|';
                tachyon_beam[j+1] = '|';
                tachyon_beam[j] = '.';

                split_counter += 1;
            }
        }
    }
    println!("{:?}", split_counter);
    
}
