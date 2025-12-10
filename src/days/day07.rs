use std::{collections::HashMap, fs};

pub fn day_7() {
    let file_path = "inputs/day_7_input.txt";
    let file_path = "inputs/day_7_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");
    let grid: Vec<Vec<char>> = context.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start_col = grid[0].iter().position(|&c| c == 'S').expect("No 'S' found error!");

    let mut timelines: HashMap<(usize, usize), usize> = HashMap::new();
    timelines.insert((start_col, 0), 1);

    for y in 0..rows {
        let mut new_timeline: HashMap<(usize, usize), usize> = HashMap::new();

        for(&(x, ty), &count) in timelines.iter() {
            if y != ty {
                continue;
            }

            if grid[y][x] == '^' {
                // Left
                if x > 0 {
                    *new_timeline.entry((x - 1, y + 1)).or_insert(0) += count;
                }
                // Right
                if x + 1 < cols {
                    *new_timeline.entry((x + 1, y + 1)).or_insert(0) += count;
                }
            } else {
                // Straight down
                if y + 1 < rows {
                    *new_timeline.entry((x, y + 1)).or_insert(0) += count;
                }
            }
        }

        // Merge timelines
        for (k, v) in new_timeline {
            *timelines.entry(k).or_insert(0) += v;
        }
    }

    let total_timelines: usize = timelines
        .iter()
        .filter(|&((_, y), _)| *y == rows - 1)
        .map(|(_, &count)| count)
        .sum();
    
    println!("{}", total_timelines)
}


