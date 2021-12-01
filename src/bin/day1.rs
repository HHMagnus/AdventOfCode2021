use std::env;
use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input/day1.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|c| c.parse().unwrap())
        .collect();

    let part1: i32 = input
        .as_slice()
        .windows(2)
        .map(|a| (a[1] > a[0]) as i32)
        .sum();

    println!("Day1 part 1: {}", part1);

    let part2: i32 = input
        .as_slice()
        .windows(3)
        .map(|a| a[0] + a[1] + a[2])
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(2)
        .map(|a| (a[1] > a[0]) as i32)
        .sum();

    println!("Day2 part 2: {}", part2);
}
