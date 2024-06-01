use std::fs;

fn main() {
    let input = fs::read_to_string("input/day16.txt")
        .expect("Could not read file")
        .chars()
		.map(to_binary)
        .collect::<Vec<&str>>()
		.join("");

	let (_, versions, part2) = pck(&input);

	println!("Day 16 part 1: {}", versions.iter().sum::<i32>());
	println!("Day 16 part 2: {}", part2);
}

fn math_op(results: &Vec<usize>, type_id: i32) -> usize {
	if type_id == 0 {
		return results.iter().sum();
	}
	if type_id == 1 {
		return results.iter().product();
	}
	if type_id == 2 {
		return *results.iter().min().unwrap();
	}
	if type_id == 3 {
		return *results.iter().max().unwrap();
	}
	if type_id == 5 {
		let first = results[0];
		let second = results[1];
		return if first > second { 1 } else { 0 };
	}
	if type_id == 6 {
		let first = results[0];
		let second = results[1];
		return if first < second { 1 } else { 0 };
	}
	if type_id == 7 {
		let first = results[0];
		let second = results[1];
		return if first == second { 1 } else { 0 };
	}
	panic!("Uknown type");
}

fn pck(input: &str) -> (usize, Vec<i32>, usize) {
	let version_str = &input[..3];
	let type_id_str = &input[3..6];
	let version = i32::from_str_radix(version_str, 2).unwrap();
	let type_id = i32::from_str_radix(type_id_str, 2).unwrap();
	
	if type_id == 4 {
		let mut next = 6;
		let mut total = String::new();
		loop {
			let str = &input[next..next+5];
			next += 5;
			total += &str[1..5];

			if str.starts_with("0") {
				break;
			}
		}

		let literal = usize::from_str_radix(&total, 2).unwrap();

		return (next, vec![version], literal);
	}

	let length_type_id = usize::from_str_radix(&input[6..7], 2).unwrap();

	if length_type_id == 0 {
		let extra_bits = usize::from_str_radix(&input[7..22], 2).unwrap();
		let mut cns = vec![version];
		let mut next = 22;
		let mut results = Vec::new();
		loop {
			let (idx, versions, res) = pck(&input[next..]);
			cns.extend(versions);
			next += idx;
			results.push(res);
			if next-22 >= extra_bits {
				break;
			}
		}
		return (next, cns, math_op(&results, type_id));
	}

	let extra_packets = usize::from_str_radix(&input[7..18], 2).unwrap();
	let mut cns = vec![version];
	let mut next = 18;
	let mut results = Vec::new();
	for _ in 0..extra_packets {
		let (idx, versions, res) = pck(&input[next..]);
		cns.extend(versions);
		next += idx;
		results.push(res);
	}
	
	(next, cns, math_op(&results, type_id))
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("unknown"),
    }
}