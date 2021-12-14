use std::fs;
use std::collections::HashMap;
use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord(u32, u32);
#[derive(Debug)]
struct Line(Coord, Coord);

fn main() {
    let read = fs::read_to_string("input/day5.txt")
        .expect("Could not read file");

    let lines: Vec<Line> = read
        .split("\n")
        .map(|x| x.split(" -> ")
            .map(|y| y.split(',')
                .collect::<Vec<&str>>())
            .map(|y| Coord(y[0].parse().unwrap(), y[1].parse().unwrap()))
            .collect::<Vec<Coord>>())
        .map(|x| Line(x[0], x[1]))
        .collect();

    let vertical: Vec<Coord> = lines
        .iter()
        .filter(|&x| x.0.1 == x.1.1)
        .map(|x| {
            let mut coords: Vec<Coord> = vec![];
            for i in cmp::min(x.0.0, x.1.0) .. cmp::max(x.0.0, x.1.0)+1 { 
                coords.push(Coord(i, x.0.1))
            }
            coords
        })
        .flatten()
        .collect();

    let horizontal: Vec<Coord> = lines
        .iter()
        .filter(|&x| x.0.0 == x.1.0)
        .map(|x| {
            let mut coords: Vec<Coord> = vec![];
            for i in cmp::min(x.0.1, x.1.1) .. cmp::max(x.0.1, x.1.1)+1 { 
                coords.push(Coord(x.0.0, i))
            }
            coords
        })
        .flatten()
        .collect();
    let coords: Vec<Coord> = [vertical, horizontal].concat();
    
    let mut map = HashMap::new();

    for coord in coords {
        let counter = map.entry(coord).or_insert(0);
        *counter += 1;
    }

    let part1 = map.values().filter(|&&x| x > 1).count();

    println!("Day 5 part 1: {}", part1);

    let diagonal_coords: Vec<Coord> = lines
        .iter()
        .filter(|&x| x.0.1 != x.1.1 && x.0.0 != x.1.0)
        .map(|line| {
            let mut coords: Vec<Coord> = vec![];
            let mut x = cmp::min(line.0.0, line.1.0);
            let mut y = if line.0.0 == x { line.0.1 } else { line.1.1 };
            let add_y = cmp::min(line.0.1, line.1.1) == y;
            let end_x = cmp::max(line.0.0, line.1.0);
            while x <= end_x {
                coords.push(Coord(x,y));
                x += 1;
                if add_y {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
            coords
        })
        .flatten()
        .collect();
    
    for coord in diagonal_coords {
        let counter = map.entry(coord).or_insert(0);
        *counter += 1;
    }
    
    let part2 = map.values().filter(|&&x| x > 1).count();

    println!("Day 5 part 2: {}", part2);
}