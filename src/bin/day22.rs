use std::fs;

#[derive(Debug, Clone)]
struct Range {
	x: (i64, i64),
	y: (i64, i64),
	z: (i64, i64),
}

#[derive(Debug, Clone)]
enum Signal {
	On(Range),
	Off(Range),
}

impl Signal {
	fn range(&self) -> &Range{
		match self {
			Signal::Off(r) => r,
			Signal::On(r) => r
		}
	}
}

fn main() {
    let input = fs::read_to_string("input/day22.txt")
        .expect("Could not read file");

	let parsed: Vec<Signal> = input.lines()
		.map(|line| {
			let replaced = line.replace("on x=", "").replace("y=", "").replace("z=", "").replace("off x=", "");
			let s1 = replaced.split(",")
				.map(|x| x.split("..").collect::<Vec<_>>())
				.collect::<Vec<_>>();
			let x0: i64 = s1[0][0].parse().unwrap();
			let x1: i64 = s1[0][1].parse().unwrap();
			let y0: i64 = s1[1][0].parse().unwrap();
			let y1: i64 = s1[1][1].parse().unwrap();
			let z0: i64 = s1[2][0].parse().unwrap();
			let z1: i64 = s1[2][1].parse().unwrap();
			let range = Range {
				x: (x0, x1),
				y: (y0, y1),
				z: (z0, z1)
			};
			if line.starts_with("on") {
				Signal::On(range)
			} else {
				Signal::Off(range)
			}
		}).collect();

	let part1 = day22(parsed[..20].to_vec());
	println!("Day 22 part 1: {}", part1);

	let part2 = day22(parsed);
	println!("Day 22 part 2: {}", part2);
}

fn day22(input: Vec<Signal>) -> i64 {
	let mut signals: Vec<Signal> = Vec::new();

	for signal in input {
		let mut adds = Vec::new();
		for other in signals.iter() {
			if let Some(range) = intersection(signal.range(), other.range()) {
				let n = match other {
					Signal::Off(_) => Signal::On(range),
					Signal::On(_) => Signal::Off(range),
				};
				adds.push(n);
			}
		}
		match signal.clone() {
			Signal::On(_) => adds.push(signal),
			_ => ()
		};

		for s in adds {
			signals.push(s);
		}
	}

	return signals.iter()
		.map(|x| match x {
			Signal::On(r) => r.cubes(),
			Signal::Off(r) => -r.cubes()
		}).sum();
}

impl Range {
	fn cubes(&self) -> i64{
		(self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
	}
}

fn intersection(r1: &Range, r2: &Range) -> Option<Range> {
	let x0 = r1.x.0.max(r2.x.0);
	let x1 = r1.x.1.min(r2.x.1);
	let y0 = r1.y.0.max(r2.y.0);
	let y1 = r1.y.1.min(r2.y.1);
	let z0 = r1.z.0.max(r2.z.0);
	let z1 = r1.z.1.min(r2.z.1);

	if x0 <= x1 && y0 <= y1 && z0 <= z1 {
		return Some(Range {
			x: (x0, x1),
			y: (y0, y1),
			z: (z0, z1)
		})
	}

	None
}