use std::fs;

fn main() {
    let input = fs::read_to_string("input/day25.txt")
        .expect("Could not read file");

	let lines = input.lines().map(|x| x.chars().enumerate().collect::<Vec<_>>()).enumerate().collect::<Vec<_>>();

	let max_y =lines.iter().map(|x| x.0).max().unwrap();
	let max_x = lines.iter().map(|x| x.1.iter().map(|x| x.0).max().unwrap()).max().unwrap();

	let east_initial = lines.iter().flat_map(|(y, cs)| cs.iter().filter_map(move |(x, c)| if c == &'>' { Some((*y,*x))} else { None })).collect::<Vec<_>>();
	let south_initial = lines.iter().flat_map(|(y, cs)| cs.iter().filter_map(move |(x, c)| if c == &'v' { Some((*y,*x))} else { None })).collect::<Vec<_>>();

	let mut step = 0;
	let mut east = east_initial.clone();
	let mut south = south_initial.clone();
	loop {
		step += 1;

		let east_moved = east.iter().map(|&(y, x)| {
			let next = if x == max_x {
				(y, 0)
			} else {
				(y, x+1)
			};
			if east.contains(&next) {
				return (y, x);
			}
			if south.contains(&next) {
				return (y, x);
			}
			next
		}).collect::<Vec<_>>();

		let south_moved = south.iter().map(|&(y, x)| {
			let next = if y == max_y {
				(0, x)
			} else {
				(y+1, x)
			};
			if east_moved.contains(&next) {
				return (y, x);
			}
			if south.contains(&next) {
				return (y, x);
			}
			next
		}).collect::<Vec<_>>();

		if east_moved == east && south_moved == south {
			break;
		}

		east = east_moved;
		south = south_moved;
	}

	println!("Day 25 part 1: {}", step);
}