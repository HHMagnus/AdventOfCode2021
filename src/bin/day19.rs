use std::fs;

fn main() {
    let input = fs::read_to_string("input/day19.txt")
        .expect("Could not read file");

	let lines = input
		.split("\n\n")
		.map(|x| x.lines().collect::<Vec<_>>()[1..].iter().map(|x| {
			let split = x.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
			(split[0], split[1], split[2])
		}).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	println!("{:?}", lines);
}
