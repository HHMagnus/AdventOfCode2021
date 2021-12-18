use std::fs;

fn main() {
    let lines: Vec<Vec<u32>> = fs::read_to_string("input/day9.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();

    let mut part1 = 0;

    for x in 0..lines.len() {
        for y in 0..lines[0].len() {
            let up = if y+1 >= lines[0].len() { 10 } else { lines[x][y+1] };
            let down = if y == 0 { 10 } else { lines[x][y-1] };
            let left = if x+1 >= lines.len() { 10 } else { lines[x+1][y] };
            let right = if x == 0 { 10 } else { lines[x-1][y] };

            let curr = lines[x][y];

            if curr < up && curr < down && curr < left && curr < right {
                part1 += curr+1;
            }
        }
    }

    println!("Day 9 part 1: {:?}", part1);

    let mut check: Vec<Vec<bool>> = lines.iter()
        .map(|x| x.iter().map(|&y| y == 9).collect())
        .collect();

    let mut basins: Vec<u32> = vec![];

    for x in 0..lines.len() {
        for y in 0..lines[0].len() {
            if check[x][y] {
                continue;
            }
            let basin = search_and_mark(x, y, &mut check);
            basins.push(basin);
        }
    }
    basins.sort();
    basins.reverse();
    let part2 = basins[0] * basins[1] * basins[2];

    println!("Day 9 part 2: {:?}", part2);
}

fn search_and_mark(x: usize, y: usize, check: &mut Vec<Vec<bool>>) -> u32 {
    if check[x][y] {
        return 0;
    }
    check[x][y] = true;
    let mut result = 1;
    if x+1 < check.len() {
        result += search_and_mark(x+1, y, check);
    }
    if y+1 < check[0].len() {
        result += search_and_mark(x, y+1, check);
    }
    if x > 0 {
        result += search_and_mark(x-1, y, check);
    }
    if y > 0 {
        result += search_and_mark(x, y-1, check);
    }
    result
}