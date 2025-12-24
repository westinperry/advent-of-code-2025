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
        let goal_str = row[0];
        let num_lights = goal_str.len() - 2; // strip [ ]
        let voltage = row[row.len() - 1];
        
        let goal_mask = goal(goal_str);

        let mut button_masks = Vec::new();
        for token in row.iter().skip(1) {
            if token.starts_with('(') {
                let mask = buttons(token, num_lights as u32);
                button_masks.push(mask);
            }
        }

        if let Some(min_p) = min_presses(goal_mask, &button_masks) {
            total_presses += min_p as u64;
        } else {
            panic!("No solution found for row: {:?}", row);
        }
    }

    println!("Total minimum presses: {}", total_presses);
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
