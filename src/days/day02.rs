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

            // Only even numbers matter
            if num_len % 2 == 0 {
                let half = num_len / 2;
                // If first half equals second half invalid ID found
                if &s[0..half] == &s[half..] {
                    //println!("matched: {}", number);
                    invalid_ids += number;
                }
            }
        }
    }

    println!("{}", invalid_ids)
}
