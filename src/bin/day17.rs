use std::fs;

fn main() {
    let input = fs::read_to_string("input/day17.txt")
        .expect("Could not read file");

	let split: Vec<_> = input.split(", y=").collect();

	let xsplit: Vec<_> = split[0].split("..").collect();
	let ysplit: Vec<_> = split[1].split("..").collect();

	let min_x: i32 = xsplit[0].replace("target area: x=", "").parse().unwrap();
	let max_x: i32 = xsplit[1].parse().unwrap();
	
	let min_y: i32 = ysplit[0].parse().unwrap();
	let max_y: i32 = ysplit[1].parse().unwrap();

	let mut possibles = Vec::new();

	for x in 0..max_x+100 {
		for y in min_y..100 {
			possibles.push((x, y));
		}
	}

	let res: Vec<_> = possibles.into_iter().filter_map(|(initial_x, initial_y)| {
		let mut pos = (0, 0);
		let mut x = initial_x;
		let mut y = initial_y;
		let mut reached_y = 0;

		while pos.0 < max_x && pos.1 > min_y {
			pos = (pos.0 + x, pos.1 + y);
			if x > 0 {
				x -= 1;
			}
			y -= 1;

			if pos.1 > reached_y {
				reached_y = pos.1;
			}

			if min_x <= pos.0 && max_x >= pos.0 && min_y <= pos.1 && max_y >= pos.1 {
				return Some(reached_y);
			}
		}
		None
	}).collect();

	let part1 = res.iter().max().unwrap();
	let part2 = res.len();

	println!("Day 17 part 1: {}", part1);
	println!("Day 17 part 2: {}", part2);
}