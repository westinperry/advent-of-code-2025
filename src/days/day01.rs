use std::fs;

pub fn day_1() {
    // File expected, new dial directions per line
    let file_path = "inputs/day_1_input.txt";

    let contents = fs::read_to_string(file_path).expect("read file error");

    let context_vec = read_lines(&contents);

    let mut curr_num: i32 = 50;
    let mut count: i32 = 0;

    for line in context_vec {
        // Getting first char from line
        let dir = line.chars().next().unwrap();
        // Getting distance from line (making int)
        let dist: i32 = line[1..].parse().expect("invalid number");

        /*
        Get number of rotations, then for each remainder determine if
        number would go over or be 0. Similar for each direction
        */
        if dir == 'R' {
            count += dist / 100;
            for _ in 0..(dist % 100) {
                curr_num = (curr_num + 1) % 100;
                if curr_num == 0 {
                    count += 1;
                }
            }
        } else {
            count += dist / 100;
            for _ in 0..(dist % 100) {
                curr_num = (curr_num - 1) % 100;
                if curr_num == 0 {
                    count += 1;
                }
            }
        }
    }

    println!("Combination to the safe is: {}", count);
}

// Takes reference to str -> returns vector of all lines
fn read_lines(contents: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}
