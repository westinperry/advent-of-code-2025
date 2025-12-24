use std::fs;

pub fn day_10() {
    let content = fs::read_to_string("./inputs/day_10_input.txt")
        .expect("Error reading file");

    let input_rows: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    let row_vectors: Vec<Vec<&str>> = input_rows
        .iter()
        .map(|item| item.split_whitespace().collect())
        .collect();

    let mut total_presses: u64 = 0;

    for row in &row_vectors {
        let voltage = row[row.len() - 1];
        let volt_vec = extract_voltage(voltage);

        let mut button_vecs: Vec<Vec<usize>> = Vec::new();
        for token in row.iter().skip(1) {
            if token.starts_with('(') {
                button_vecs.push(button_indices(token));
            }
        }

        if let Some(min_p) = min_presses_part2(&volt_vec, &button_vecs) {
            total_presses += min_p as u64;
        } else {
            panic!("No solution found for row: {:?}", row);
        }
    }

    println!("Total minimum presses: {}", total_presses);
}

fn extract_voltage(voltage_str: &str) -> Vec<u32> {
    let voltage_str = &voltage_str[1..voltage_str.len()-1];
    let voltage = voltage_str.split(",").map(|s| s.parse::<u32>().expect("Error parsing")).collect();

    voltage
}

fn goal(s: &str) -> u32 {
    let s = &s[1..s.len() - 1]; // strip [ ]
    let mut key = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '#' {
            // bit position: from left to right; MSB is index 0
            key |= 1 << (s.len() - i - 1);
        }
    }

    key
}

fn buttons(b: &str, num_lights: u32) -> u32 {
    let b = &b[1..b.len() - 1]; // strip ( )
    let mut mask: u32 = 0;

    if b.trim().is_empty() {
        return 0;
    }

    for token in b.split(',') {
        let val = token.trim().parse::<u32>().expect("Error parsing button index");
        assert!(val < num_lights, "Button index out of range");
        mask |= 1 << (num_lights - 1 - val);
    }

    mask
}

fn button_indices(b: &str) -> Vec<usize> {
    let b = &b[1..b.len() - 1]; // strip ( )
    
    if b.trim().is_empty() {
        return vec![];
    }

    b.split(',')
        .map(|token| token.trim().parse().expect("Error parsing button index"))
        .collect()
}

fn min_presses(goal_mask: u32, buttons: &[u32]) -> Option<u32> {
    let n = buttons.len();
    if n == 0 {
        return if goal_mask == 0 { Some(0) } else { None };
    }

    let mut best: Option<u32> = None;
    let total_states = 1u64 << n;

    for subset in 0..total_states {
        let mut state: u32 = 0;
        let mut presses = 0;

        for i in 0..n {
            if (subset >> i) & 1 == 1 {
                state ^= buttons[i];
                presses += 1;
                if let Some(b) = best {
                    if presses >= b {
                        break;
                    }
                }
            }
        }

        if state == goal_mask {
            match best {
                None => best = Some(presses),
                Some(b) if presses < b => best = Some(presses),
                _ => {}
            }
        }
    }

    best
}

fn min_presses_part2(targets: &[u32], buttons: &[Vec<usize>]) -> Option<u32> {
    let n = buttons.len();
    let num_counters = targets.len();
    
    if n == 0 {
        return if targets.iter().all(|&t| t == 0) { Some(0) } else { None };
    }

    let max_presses = *targets.iter().max().unwrap_or(&0);
    
    fn solve(
        button_idx: usize,
        counters: &mut Vec<u32>,
        buttons: &[Vec<usize>],
        targets: &[u32],
        current_presses: u32,
        best: &mut Option<u32>,
        max_presses: u32,
    ) {
        if let Some(b) = *best {
            if current_presses >= b {
                return;
            }
        }

        if button_idx == buttons.len() {
            if counters.iter().zip(targets).all(|(c, t)| c == t) {
                *best = Some(current_presses);
            }
            return;
        }

        for presses in 0..=max_presses {
            for &idx in &buttons[button_idx] {
                counters[idx] += presses;
            }

            let valid = counters.iter().zip(targets).all(|(c, t)| c <= t);
            
            if valid {
                solve(button_idx + 1, counters, buttons, targets, current_presses + presses, best, max_presses);
            }

            for &idx in &buttons[button_idx] {
                counters[idx] -= presses;
            }
        }
    }

    let mut counters = vec![0u32; num_counters];
    let mut best = None;
    solve(0, &mut counters, buttons, targets, 0, &mut best, max_presses);
    
    best
}
