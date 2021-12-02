use std::fs;

#[derive(Debug, Clone)]
enum Input {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let input: Vec<Input> = fs::read_to_string("input/day2.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|x| x.split(' ').collect::<Vec<&str>>())
        .map(|x| match x[0] {
            "forward" => Input::Forward(x[1].parse().unwrap()),
            "down" => Input::Down(x[1].parse().unwrap()),
            "up" => Input::Up(x[1].parse().unwrap()),
            _ => panic!("Parsing problem"),
        })
        .collect();

    let part1 = input
        .iter()
        .cloned()
        .fold((0, 0), |acc, b| match b {
            Input::Forward(x) => (acc.0+x, acc.1),
            Input::Down(x) => (acc.0, acc.1+x),
            Input::Up(x) => (acc.0, acc.1-x),
        });

    println!("Day1 part 1: {:?}", (part1.0 * part1.1));

    let part2 = input
        .iter()
        .cloned()
        .fold((0, 0, 0), |acc, b| match b {
            Input::Forward(x) => (acc.0+x, acc.1+(acc.2*x), acc.2),
            Input::Down(x) => (acc.0, acc.1, acc.2+x),
            Input::Up(x) => (acc.0, acc.1, acc.2-x),
        });

    println!("Day2 part 2: {}", (part2.0 * part2.1));
}
