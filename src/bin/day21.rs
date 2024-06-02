use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/day21.txt")
        .expect("Could not read file");

	let lines: Vec<_> = input.lines().collect();

	let first: u8 = lines[0].replace("Player 1 starting position: ", "").parse().unwrap();
	let second: u8 = lines[1].replace("Player 2 starting position: ", "").parse().unwrap();

	let part1 = deterministic_100_sided_die(first, second);

	println!("Day 21 part 1: {}", part1);

	let mut memo = HashMap::new();

	let part2 = dirac_dice(&mut memo, first, second, 0, 0, true);

	println!("Day 21 part 2: {}", part2.0.max(part2.1));
}

fn dirac_dice(memo: &mut HashMap<(u8, u8, u64, u64, bool), (u128, u128)>, f: u8, s: u8, fs: u64, ss: u64, t: bool) -> (u128, u128) {
	let mut w1 = 0;
	let mut w2 = 0;

	let key = (f, s, fs, ss, t);
	if memo.contains_key(&key) {
		return memo[&(f, s, fs, ss, t)];
	}

	for r1 in 1..4 {
		for r2 in 1..4 {
			for r3 in 1..4 {
				let roll = r1 + r2 + r3;

				if t {
					let f = f + roll;
					let f = (f - 1) % 10 + 1;
					let fs = fs + f as u64;

					if fs >= 21 {
						w1 += 1;
					} else {
						let (e1, e2) = dirac_dice(memo, f, s, fs, ss, !t);
						w1 += e1;
						w2 += e2;
					}
				} else {
					let s = s + roll;
					let s = (s - 1) % 10 + 1;
					let ss = ss + s as u64;

					if ss >= 21 {
						w2 += 1;
					} else {
						let (e1, e2) = dirac_dice(memo, f, s, fs, ss, !t);
						w1 += e1;
						w2 += e2;
					}
				}
			}
		}
	}
	let value = (w1, w2);
	memo.insert(key, value);
	value
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