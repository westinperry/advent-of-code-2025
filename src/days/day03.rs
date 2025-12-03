use std::fs;


pub fn day_3() {
    let file_path = "inputs/day_3_input.txt";

    let context = fs::read_to_string(file_path)
        .expect("Error reading file");

    let mut context_vec: Vec<String> = Vec::new();

    for line in context.lines() {
        context_vec.push(line.to_string());
    }

    let mut total_joltage = 0;

    for sequence in context_vec {
        let chars: Vec<char> = sequence.chars().collect();
        
        let mut best_for_line = 0;

        for i in 0..chars.len() {
            for j in i + 1..chars.len() {
                let first = chars[i].to_digit(10).expect("invalid chars");
                let second = chars[j].to_digit(10).expect("invalid chars");

                let value = 10 * first + second;

                if value > best_for_line {
                    best_for_line = value;
                }
            }

            
        }
        //println!("Best line: {}", best_for_line);
        total_joltage += best_for_line;
    }

    println!("Voltage for day_3: {}", total_joltage)

}


// 17622 Too high
// 17408