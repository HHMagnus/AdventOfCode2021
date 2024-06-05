use std::{collections::HashSet, fs}; 

fn main() {
    let input = fs::read_to_string("input/day23.txt")
        .expect("Could not read file");

	let lines: Vec<_> = input.lines().collect();

	let r1a = lines[2].chars().nth(3).unwrap();
	let r1b = lines[3].chars().nth(3).unwrap();
	let r2a = lines[2].chars().nth(5).unwrap();
	let r2b = lines[3].chars().nth(5).unwrap();
	let r3a = lines[2].chars().nth(7).unwrap();
	let r3b = lines[3].chars().nth(7).unwrap();
	let r4a = lines[2].chars().nth(9).unwrap();
	let r4b = lines[3].chars().nth(9).unwrap();

	let amphipods = [
		(r1a, 1, 2, 0),
		(r1b, 2, 2, 0),
		(r2a, 1, 4, 0),
		(r2b, 2, 4, 0),
		(r3a, 1, 6, 0),
		(r3b, 2, 6, 0),
		(r4a, 1, 8, 0),
		(r4b, 2, 8, 0),
	].to_vec();

	let part1 = part1(amphipods);
	
	println!("Day 23 part 1: {}", if part1 == i32::max_value() { 0 } else { part1 });
}

fn part1(amphipods: Vec<(char, i32, i32, i32)>) -> i32 {
	let mut states = HashSet::new();
	states.insert(amphipods);

	let mut min_score = i32::max_value();

	loop {
		let mut new_states = HashSet::new();

		for state in states {
			let mut win = true;
			let mut total = 0;
			for (i, (c, y, x, s)) in state.clone().into_iter().enumerate() {
				let multiplier = match c {
					'A' => 1,
					'B' => 10,
					'C' => 100,
					'D' => 1000,
					_ => panic!("Unknown c"),
				};
				let endx = match c {
					'A' => 2,
					'B' => 4,
					'C' => 6,
					'D' => 8,
					_ => panic!("Unknown c"),
				};

				if endx == x && (y == 2 || state.iter().any(|(c0, y0, x0, _)| x0 == &endx && y0 == &2 && c0 == &c)) {
					total += s;
					continue;
				}
				win = false;

				if y == 0 {
					if state.iter().any(|(_, y0, x0, _)| y0 == &1 && x0 == &endx) {
						continue;
					}

					if state.iter().any(|(c0, y0, x0, _)| y0 == &2 && x0 == &endx && &c != c0) {
						continue;
					}

					let mut currx = x;
					let mut moves = 0;

					while currx != endx {
						moves += 1;
						let next = if endx > currx { currx + 1 } else { currx - 1 };
						if state.iter().any(|(_, y0, x0, _)| y0 == &0 && x0 == &next) {
							break;
						}
						currx = next;
					}

					if currx != endx {
						continue;
					}
					
					let ny = if state.iter().any(|(_, y0, x0, _)| y0 == &2 && x0 == &endx) {
						moves += 1;
						1
					} else {
						moves += 2;
						2
					};

					let n_amp = (c, ny, endx, s + moves * multiplier);

					let mut n_state = Vec::new();
					for j in 0..state.len() {
						if i == j {
							n_state.push(n_amp);
						} else {
							n_state.push(state[j]);
						}
					}
					new_states.insert(n_state);

					continue;
				}

				if y == 2 && state.iter().any(|(_, y0, x0, _)| y0 == &1 && x0 == &x) {
					continue;
				}

				let moves = y;

				let mut possible_x = Vec::new();

				let mut left = x;
				while left > 0 {
					left -= 1;

					if state.iter().any(|(_, y0, x0, _)| y0 == &0 && x0 == &left) {
						break;
					}

					if left == 2 || left == 4 || left == 6 || left == 8 {
						continue;
					}

					possible_x.push(left);
				}

				let mut right = x;
				while right < 10 {
					right += 1;

					if state.iter().any(|(_, y0, x0, _)| y0 == &0 && x0 == &right) {
						break;
					}

					if right == 2 || right == 4 || right == 6 || right == 8 {
						continue;
					}

					possible_x.push(right);
				}

				for posx in possible_x {
					let moves = moves + if x - posx < 0 { posx - x } else { x - posx };
					let n_amp: (char, i32, i32, i32) = (c, 0, posx, s + moves * multiplier);

					let mut n_state = Vec::new();
					for j in 0..state.len() {
						if i == j {
							n_state.push(n_amp);
						} else {
							n_state.push(state[j]);
						}
					}

					new_states.insert(n_state);
				}
			}

			if win {
				if min_score > total {
					min_score = total;
				}
			}
		}

		states = new_states;

		println!("Current states: {:?}", states.len());

		if states.len() == 0 {
			return min_score;
		}

		/*for state in states.iter() {
			println!("Queue:");
			for y in 0..=2 {
				for x in 0..=10 {
					if let Some((c, _, _, _)) = state.iter().find(|(_, y0, x0, _)| y0 == &y && x0 == &x) {
						print!("{}", c);
					} else { 
						if (y == 1 || y == 2) && (x == 0 || x == 1 || x == 3 || x == 5 || x == 7 || x == 9 || x == 10) {
							print!(" ")
						} else {
							print!(".")
						}
					 }
				}
				println!("");
			}
		}*/
	}
}