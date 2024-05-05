use std::fs;

fn main() {
    let input = fs::read_to_string("input/day8.txt")
        .expect("Could not read file");

    let lines: Vec<Vec<Vec<&str>>> = input
        .split("\n")
        .map(|x| x.split(" | ").map(|y| y.split(" ").collect()).collect())
        .collect();

    let part1: usize = lines
        .iter()
        .map(|x| x[1].iter().filter(|y| y.len() == 2 || y.len() == 4 || y.len() == 3 || y.len() == 7).count())
        .sum();

    println!("Day 8 part 1: {:?}", part1);

    let part2: u32 = lines.iter()
        .map(|x| entry(x))
        .sum();

    println!("Day 8 part 2: {:?}", part2);
}

fn entry(code: &Vec<Vec<&str>>) -> u32 {
    let top = find_top(&code[0]);
    let (left_top, middle) = find_left_top_and_middle(&code[0], &top);
    let (right_bottom, right_top) = find_right_top_and_right_bottom(&code[0], &middle);
    let (left_bottom, bottom) = find_left_bottom_and_bottom();

    let numbers = vec![
        vec![top, left_top, left_bottom, right_top, right_bottom, bottom],
        vec![right_top, right_bottom],
        vec![top, right_top, middle, left_bottom, bottom],
        vec![top, right_top, middle, right_bottom, bottom],
        vec![left_top, right_top, middle, right_bottom],
        vec![top, left_top, middle, right_bottom, bottom],
        vec![top, right_top, middle, right_bottom, left_bottom, bottom],
        vec![top, right_top, right_bottom],
        vec![top, right_top, left_top, middle, left_bottom, right_bottom, bottom],
        vec![top, left_top, right_top, middle, right_bottom, bottom],
    ];

    let translated: Vec<u32> = code[1].iter()
        .map(|x| translate(&numbers, x))
        .collect();
    
    translated[0] * 1000 + translated[1] * 100 + translated[2] * 10 + translated[3]
}

fn find_top(code: &Vec<&str>) -> char {// resolve t by comparing 1 and 7
    let one = code.iter().
    'a'
}

fn find_left_top_and_middle(code: &Vec<&str>, top: &char) -> (char, char) {// resolve lt and m by finding 3 by comparing with 1 and removing all 4
    ('b', 'c')
}

fn find_right_top_and_right_bottom(code: &Vec<&str>, middle: &char) -> (char, char) {// resolve rt and rl by finding 0 and 6 and seeing which is which
    ('d', 'e')
}

fn find_left_bottom_and_bottom() -> (char, char) {// resolve b by finding unknown in 3 // resolving lb by finding unknown from 8
    ('f', 'g')
}

fn translate(numbers: &Vec<Vec<char>>, chars: &str) -> u32 {
    1
}