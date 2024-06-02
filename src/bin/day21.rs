use std::fs;

fn main() {
    let input = fs::read_to_string("input/day21.txt")
        .expect("Could not read file");

	let lines: Vec<_> = input.lines().collect();

	let first: u8 = lines[0].replace("Player 1 starting position: ", "").parse().unwrap();
	let second: u8 = lines[1].replace("Player 2 starting position: ", "").parse().unwrap();

	let part1 = deterministic_100_sided_die(first, second);

	println!("Day 21 part 1: {}", part1);
}

fn deterministic_100_sided_die(first: u8, second: u8) -> u64 {
	let mut first = first as u32;
	let mut second = second as u32;
	let mut dice = 1;
	let mut score1 = 0;
	let mut score2 = 0;
	let mut roll_count = 0;
	let mut round = 0;

	while score1 < 1000 && score2 < 1000 {
		let mut roll = 0;
		for _ in 0..3 {
			roll += dice;
			dice += 1;
			dice = (dice - 1) % 100 + 1;
			roll_count += 1;
		}

		if round % 2 == 0 {
			first += roll;
			first = (first - 1) % 10 + 1;
			score1 += first as u32;
		} else {
			second += roll;
			second = (second - 1) % 10 + 1;
			score2 += second as u32;
		}
		
		round += 1;
	}

	return roll_count as u64 * score1.min(score2) as u64;
}