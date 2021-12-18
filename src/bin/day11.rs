use std::fs;

fn main() {
    let mut lines: Vec<Vec<u32>> = fs::read_to_string("input/day11.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();

    let mut part1 = 0;
    
    for _step in 0..100 {
        part1 += flashes(&mut lines);
    }
    
    println!("Day 11 part 1: {}", part1);

    let mut part2 = 100;
    loop {
        part2 += 1;
        if flashes(&mut lines) == 100{
            break;
        }
    }

    println!("Day 11 part 1: {}", part2);
}

fn flashes(lines: &mut Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    for x in 0..lines.len() {
        for y in 0..lines[0].len() {
            lines[x][y] += 1
        }
    }

    let mut hit = true;
    while hit {
        hit = false;

        for x in 0..lines.len() {
            for y in 0..lines[0].len() {
                if lines[x][y] > 9 {
                    total += 1;
                    hit = true;
                    
                    lines[x][y] = 0;
                    if x > 0 && lines[x-1][y] != 0 {
                        lines[x-1][y] += 1;
                    }
                    if y > 0 && lines[x][y-1] != 0 {
                        lines[x][y-1] += 1;
                    }
                    if x+1 < lines.len() && lines[x+1][y] != 0 {
                        lines[x+1][y] += 1;
                    }
                    if y+1 < lines[0].len() && lines[x][y+1] != 0 {
                        lines[x][y+1] += 1;
                    }

                    if x > 0 && y > 0 && lines[x-1][y-1] != 0 {
                        lines[x-1][y-1] += 1;
                    }
                    if x > 0 && y+1 < lines[0].len() && lines[x-1][y+1] != 0 {
                        lines[x-1][y+1] += 1;
                    }
                    if x+1 < lines.len() && y > 0 && lines[x+1][y-1] != 0 {
                        lines[x+1][y-1] += 1;
                    }
                    if x+1 < lines.len() && y+1 < lines[0].len() && lines[x+1][y+1] != 0 {
                        lines[x+1][y+1] += 1;
                    }
                }
            }
        }
    }
    total
}