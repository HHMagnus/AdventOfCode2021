use std::fs;

fn main() {
    let input: Vec<char> = fs::read_to_string("input/day16.txt")
        .expect("Could not read file")
        .chars()
        .collect();
    
    println!("{:?}", input);
}