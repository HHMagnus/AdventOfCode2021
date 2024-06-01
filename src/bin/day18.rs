use std::fs;

#[derive(Debug)]
enum V {
	Pair(Box<(V, V)>),
	Value(i32)
}

fn main() {
    let input = fs::read_to_string("input/day18.txt")
        .expect("Could not read file");

	let mapped: Vec<_> = input.lines()
		.map(|x| p(x).0)
		.collect();

	println!("{:?}", mapped[0]);
}

fn p(input: &str) -> (V, usize) {
	let (v1, idx1) = v(&input);

	if idx1 >= input.len() {
		return (v1, idx1);
	}

	let (v2, idx2) = v(&input[idx1+1..]);

	(V::Pair(Box::new((v1, v2))), idx1+idx2+2)
}

fn v(input: &str) -> (V, usize) {
	if input.starts_with("[") {
		let (v, idx) = p(&input[1..]);
		return (v, idx+1)
	}

	let mut r = 0;
	while input[r..r+1].starts_with(",") && &input[r..r+1] != "]" {
		r += 1;
	}

	let value: i32 = input[0..r+1].parse().unwrap();

	(V::Value(value), r+1)
}