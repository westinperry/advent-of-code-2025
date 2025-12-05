use std::fs;

pub fn day_5() {
    //let file_path = "inputs/day_5_input.txt";
    let file_path = "inputs/day_5_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");

    let mut context_vec: Vec<String> = Vec::new();

    for line in context.lines() {
        context_vec.push(line.to_string());
    }

    // Seperate into vectors
    let space_idx = context_vec.iter().position(|x| x.is_empty());
    let range_vec = &context_vec[..space_idx.unwrap()];
    let id_vec = &context_vec[space_idx.unwrap()+1..];



    println!("{:?}, {:?}", range_vec, id_vec);
}