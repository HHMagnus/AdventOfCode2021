use std::fs;

fn main() {
    let read = fs::read_to_string("input/day7.txt")
        .expect("Could not read file");
    
    let input: Vec<u64> = read.split(",").map(|x| x.parse().unwrap()).collect();

    //let input = vec![16,1,2,0,4,2,7,1,2,14];

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut part1 = 1000000;
    for i in min .. max+1 {
        let fuel = fuel1(&input, i);
        if part1 > fuel {
            part1 = fuel;
        }
    }

    println!("Day 7 part 1: {}", part1);

    let mut part2 = 1000000000;
    for i in min .. max+1 {
        let fuel = fuel2(&input, i);
        if part2 > fuel {
            part2 = fuel;
        }
    }

    println!("Day 7 part 2: {}", part2);
}

fn fuel1(input: &Vec<u64>, alignment: u64) -> u64 {
    input.iter().map(|&x| if x > alignment { x - alignment } else { alignment - x }).sum()
}

fn fuel2(input: &Vec<u64>, alignment: u64) -> u64 {
    input.iter()
        .map(|&x| if x > alignment { x - alignment } else { alignment - x })
        .filter(|&x| x > 0)
        .map(|x| x*(x+1)/2)
        .sum()
}