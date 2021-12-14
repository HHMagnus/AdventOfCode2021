use std::fs;

fn main() {
    let read = fs::read_to_string("input/day6.txt")
        .expect("Could not read file");
    
    let mut input: Vec<u32> = read.split(",").map(|x| x.parse().unwrap()).collect();

    //let mut input = vec![3,4,3,1,2];

    let part1 = run(80, &input);

    println!("Day 6 part 1: {}", part1);

    let part2 = run(256, &input);

    println!("Day 6 part 2: {}", part2);
}

fn run(days: u32, input: &Vec<u32>) -> u64{
    let mut track: Vec<u64> = vec![0,0,0,0,0,0,0,0,0];
    for i in input {
        track[*i as usize] += 1;
    }

    for _day in 0 .. days {
        let new = track.remove(0);
        track.push(new);
        track[6] += new;
    }

    track.iter().sum()
}