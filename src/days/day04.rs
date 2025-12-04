use std::fs;


pub fn day_4() {
    let file_path = "inputs/day_4_input.txt";
    //let file_path = "inputs/day_4_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");

    // Creates a grid that of the input file.
    // (collect chars of each line, second collect - collects all vectors)
    let mut grid: Vec<Vec<char>> = context
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total_accessible_rolls: i32 = 0;

    loop {

        let (new_rolls, new_grid) = find_accessible_rolls(grid);
        grid = new_grid;

        total_accessible_rolls += new_rolls;


        if new_rolls == 0 {
            break
        }
    }
    

    println!("Total Rolls Day 4: {}", total_accessible_rolls)
   
}


fn find_accessible_rolls(grid: Vec<Vec<char>>) -> (i32, Vec<Vec<char>>) {
    // println!("{:?}", grid)
    let mut accessable_rolls = 0;
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let mut adjacent_rolls;
    let mut new_grid = grid.clone();

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
                adjacent_rolls = 0;

                // Looping through directions to check adjacent cells for rolls of paper
                for (di, dj) in directions {
                    let ni = di + i as isize;
                    let nj = dj + j as isize;
                    
                    
                    if ni >= 0 && ni < grid_width as isize &&
                       nj >= 0 && nj < grid_height as isize {
                        if grid[ni as usize][nj as usize] == '@' {
                            adjacent_rolls += 1;

                            //println!("{} {} {}", ni, nj, adjacent_rolls);
                        }

                        

                    }

                    
                }
                if adjacent_rolls < 4 {
                    accessable_rolls += 1;
                    new_grid[i][j] = '.';
                }
            }
            
        }
    }

    println!("{}", accessable_rolls);

    (accessable_rolls, new_grid)
}

// 1569 - Correct