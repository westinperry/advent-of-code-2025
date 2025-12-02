use std::fs;

pub fn day_2() {
    let file_path = "inputs/day_2_input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed Reading in File!");

    let ids: Vec<&str> = contents.trim().split(',').collect();
    let mut invalid_ids: u64 = 0;

    // For each ID
    for id in ids {
        // Had some trouble here.
        // Split ids
        let start_stop: Vec<u64> = id
            .trim()
            .split('-')
            .filter_map(|s: &str| s.parse().ok())
            .collect();

        // In range of first num to second num
        for number in start_stop[0]..=start_stop[1] {
            let s = number.to_string();
            let num_len = s.len();
            
            // Checking index less than half of string (no point going past half)
            for k in 1..=(num_len / 2) {
                // If pattern can exist for length k
                if num_len % k == 0 {
                    let pattern = &s[0..k];
                    
                    // Check if repeated pattern equals full string
                    // Need break to prevent double counting same string different pattern 3, 33, 333 in string 333333
                    if pattern.repeat(num_len / k) == s {
                        invalid_ids += number;
                        // println!("{}", number);
                        break;
                    }

                }
            }
        }
    }

    println!("Total of invalid IDs for day 2: {}", invalid_ids)
}

// 27329677164 too high
// 27180728081 yes