use std::fs;

pub fn day_5() {
    let file_path = "inputs/day_5_input.txt";
    //let file_path = "inputs/day_5_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");

    let mut context_vec: Vec<&str> = Vec::new();

    for line in context.lines() {
        context_vec.push(line);
    }

    // Seperate into vectors
    let space_idx = context_vec.iter().position(|x| x.is_empty()).unwrap();
    let range_vec: &[&str] = &context_vec[..space_idx];
    let id_vec = &context_vec[space_idx+1..];

    let ranges = merge_ranges(range_vec.to_vec());

    let mut unique_fresh_ids = 0;

    for range in ranges {
        
        unique_fresh_ids += range.1 - range.0 + 1;

    }

    println!("Day 5 Num of Fresh Food: {}", unique_fresh_ids);
}

// 352681648086052 too low
// 352681648086146

fn merge_ranges(ranges: Vec<&str>) -> Vec<(u64, u64)> {
    // Function for merging the given ranges to make smaller list of ranges

    // Extract string ranges into tuples
    let mut intervals: Vec<(u64, u64)> = Vec::new();
    for id in ranges {
        let parts: Vec<&str> = id
            .trim()
            .split("-")
            .collect();

        let start = parts[0].parse::<u64>();
        let end = parts[1].parse::<u64>();

        if start.is_ok() && end.is_ok() {
            intervals.push((start.unwrap(), end.unwrap()));
        }
    }

    // Sort tuple vector
    intervals.sort_by_key(|x| x.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    // Compares latest entry stop with current start values. If start is within end of previous merge. else..
    for (start, stop) in intervals {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(stop);
            } else {
                merged.push((start, stop));
            }
        } else {
            merged.push((start, stop));
        }
    }

    merged
    
}