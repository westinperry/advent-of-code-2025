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

    // println!("{:?}", grid)
    let mut result_grid = grid.clone();
    let grid_width = grid[0].len();
    let grid_height = grid.len();

    //println!("{}, {}", grid_width, grid_height);
    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];

    for i in 0..grid_width {
        for j in 0..grid_height {
            // If current grid is @
            if grid[i][j] == '@' {
                let mut adjacenet_rolls = 0;

                // Looping through directions to check adjacent cells for rolls of paper
                for (di, dj) in directions {
                    let ni = di + i as isize;
                    let nj = dj + j as isize;

                    if ni >= 0 && ni <= grid_width as isize &&
                       nj >= 0 && nj <= grid_height as isize {

                        if grid[ni as usize][nj as usize] == '@' {
                            adjacenet_rolls += 1;
                        }

                    }
                }

                if adjacenet_rolls < 4 {
                    result_grid[i][j] = 'x';
                } else {
                    result_grid[i][j] = '@';
                }
            }
            
        }
    }
   
}