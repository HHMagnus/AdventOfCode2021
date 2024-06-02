use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/day20.txt")
        .expect("Could not read file");

	let lines: Vec<_> = input.lines().collect();
	let translator = lines[0];

	let matrix: Vec<_> = lines[2..].iter().collect();

	let mut coords = HashSet::new();

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix[y].chars().nth(x) == Some('#') {
				coords.insert((y as i32, x as i32));
			}
		}
	}

	println!("{:?}", coords.len());

	let part1 = process(process(coords, translator, "0"), translator, "1").len();

	println!("Day 20 part 1: {}", part1);
}

fn process(coords: HashSet<(i32, i32)>, translator: &str, outside_range: &str) -> HashSet<(i32, i32)>{
	let mut crds = HashSet::new();

	let min_x = coords.iter().map(|(_, x)| *x).min().unwrap();
	let max_x = coords.iter().map(|(_, x)| *x).max().unwrap();
	let min_y = coords.iter().map(|(y, _)| *y).min().unwrap();
	let max_y = coords.iter().map(|(y, _)| *y).max().unwrap();

	println!("Round {}, {}, {}, {}", min_x, max_x, min_y, max_y);


	for y in min_y-2..max_y+2 {
		for x in min_x-2..max_x+2 {
			let binc = [
				(y-1, x-1),
				(y-1, x),
				(y-1, x+1),
				(y,  x-1),
				(y,  x),
				(y,  x+1),
				(y+1, x-1),
				(y+1, x),
				(y+1, x+1)
			].map(|h| {
				if h.0 < min_y || h.0 > max_y || h.1 < min_x || h.1 > max_x { return outside_range }
				if coords.contains(&h) { "1" } else { "0" }}
			).join("");
			let num = i32::from_str_radix(&binc, 2).unwrap();
			//println!("{},{}: {} {}", x, y, binc, num);
			if translator.chars().nth(num as usize) == Some('#') {
				crds.insert((y, x));
			}
		}
	}

	crds
}