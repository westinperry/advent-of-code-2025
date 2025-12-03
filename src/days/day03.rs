use std::fs;

pub fn day_3() {
    let file_path = "inputs/day_3_input.txt";

    let context = fs::read_to_string(file_path).expect("Error reading file");

    let mut context_vec: Vec<String> = Vec::new();

    for line in context.lines() {
        context_vec.push(line.to_string());
    }

    let mut total_joltage = 0;

    for sequence in context_vec {
        let digits: Vec<u32> = sequence
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid char"))
            .collect();

        let numbers_to_pick = 12;
        let mut stack: Vec<u32> = Vec::new();
        let mut numbers_to_remove = digits.len() - numbers_to_pick;

        // Iterating over each digit
        for &d in digits.iter() {
            // While stack is not empty, last element smaller than current digit, and still elements to remove
            while !stack.is_empty() && *stack.last().unwrap() < d && numbers_to_remove > 0 {
                // Remove last stack element to attach larger digit
                stack.pop();
                numbers_to_remove -= 1;
            }

            // Push digit onto stack. (First digit or when needed to make 12 digits long or if
            // current digit is greater than current last stack element)
            stack.push(d);
        }

        let mut final_number: u64 = 0;
        // Iterate over first 12 numbers in stack
        for &digit in stack.iter().take(numbers_to_pick) {
            // Increase final number by factor 10, add current digit (till 12 digits)
            final_number = final_number * 10 + digit as u64;
        }

        // Add to total
        total_joltage += final_number;
    }

    println!("Total joltage for day 3: {}", total_joltage);
}