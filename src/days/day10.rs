use std::fs;

pub fn day_10() {
    let content = fs::read_to_string("./inputs/day_10_input_test.txt").expect("Error reading file");

    let input_rows: Vec<String> = content
        .lines()
        .map(|line| line.to_string())
        .collect();
    
    let row_vectors: Vec<Vec<&str>> = input_rows
        .iter()
        .map(|item| item.split(" ").collect())
        .collect();

    for row in &row_vectors {
        let goal_str = row[0];
        let num_lights = goal_str.len() - 2;

        let goal_mask = goal(goal_str);

        let mut button_masks = Vec::new();
        for i in 1..row.len() {
            let mask = buttons(row[i], num_lights as u32);
            button_masks.push(mask);
        }

        println!("Goal: {:b}", goal_mask);
        println!("Buttons: {:?}", button_masks);
    }

}

fn goal(s: &str) -> u32 {

    let s = &s[1..s.len()-1];

    let mut key = 0;
    let mut count = 0;

    for c in s.chars() {
        if c == '#' {
            key |= 1 << (s.len() - count - 1);
        }
        count += 1;
    }

    println!("{:b}", key);

    key

}

fn buttons(b: &str, num_lights: u32) -> u32 {
    let b = &b[1..b.len()-1];
    let mut mask: u32 = 0;

    for token in b.split(',') {
        let val = token.trim().parse::<u32>().expect("Error parsing button index");
        assert!(val < num_lights, "Button index out of range");
        mask |= 1 << (num_lights - 1 - val);
    }

    mask
}