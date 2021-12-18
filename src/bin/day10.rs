use std::fs;

enum Score {
    Illegal(u32),
    Incomplete(Vec<char>),
}

fn main() {
    let input = fs::read_to_string("input/day10.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = input.split("\n").collect();

    let scores: Vec<Score> = lines.iter()
        .map(|x| score(x))
        .collect();

    let part1: u32 = scores.iter()
        .map(|x| match x {
            Score::Illegal(x) => *x,
            _ => 0
        })
        .sum();

    println!("Day 10 part 1: {}", part1);

    let mut part2: Vec<u64> = scores.iter()
        .filter(|x| match x {
            Score::Incomplete(_x) => true,
            _ => false,
        })
        .map(|x| match x {
            Score::Incomplete(chars) => incomplete_points(chars),
            _ => 0
        })
        .collect();

    part2.sort();

    println!("Day 10 part 2: {}", part2[part2.len()/2]);
}

fn score(line: &str) -> Score {
    let mut stack: Vec<char> = vec![];

    for x in line.chars() {
        if is_open(x) {
            stack.push(reverse(x))
        } else {
            let last = stack.pop().unwrap();
            if last != x {
                return Score::Illegal(illegal_points(x));
            }
        }
    }

    Score::Incomplete(stack)
}

fn illegal_points(x: char) -> u32 {
    match x {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("vrror")
    }
}

fn incomplete_points(chars: &Vec<char>) -> u64 {
    let mut total = 0;
    for &c in chars.iter().rev() {
        let point = incomplete_point(c);
        total *= 5;
        total += point;
    }
    total
}

fn incomplete_point(x: char) -> u64 {
    match x {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("qrror")
    }
}

fn is_open(x: char) -> bool {
    match x {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false
    }
}

fn reverse(x: char) -> char {
    match x {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("wrror")
    }
}