use std::fs;

#[derive(Debug, Clone)]
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

	let res = handle(mapped[0].clone());

	println!("{:?}", res);
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

fn can_explode(x: &V, depth: i32) -> bool {
	match x {
		V::Value(_) => false,
		V::Pair(x) => {
			let (x1, x2) = x.as_ref();
			if depth >= 4 { return true };
			return can_explode(x1, depth+1) || can_explode(x2, depth+1);
		}
	}
}

fn can_split(x: &V) -> bool {
	match x {
		V::Value(x) => if x >= &10 { true } else { false },
		V::Pair(x) => {
			let (x1, x2) = x.as_ref();
			return can_split(x1) || can_split(x2);
		}
	}
}

fn explode(x: V, depth: i32) -> (V, i32, i32, bool) {
	match x {
		V::Value(x) => (V::Value(x), 0, 0, false),
		V::Pair(x) => {
			let (x1, x2) = x.as_ref();
			let vi1 = match x1 { V::Value(_) => true, _ => false };
			let vi2 = match x2 { V::Value(_) => true, _ => false };
			if depth >= 4 && vi1 && vi2 {
				let v1 = match x1 { V::Value(v) => *v, _ => 0 };
				let v2 = match x2 { V::Value(v) => *v, _ => 0 };
				println!("Exploded: {}, {}", v1, v2);
				return (V::Value(0), v1, v2, true);
			}
			
			let (l, v1, v2, b) = explode(x1.clone(), depth+1);
			if b {
				let r = exploded_left(x2.clone(), v2);
				return (V::Pair(Box::new((l, r))), v1, 0, true)
			}
			let (r, v1, v2, b) = explode(x2.clone(), depth+1);
			if b {
				let l = exploded_right(l, v1);
				return (V::Pair(Box::new((l, r))), 0, v2, true)
			}

			(V::Pair(Box::new((l, r))), 0, 0, false)
		}
	}
}

fn split(x: V) -> (V, bool) {
	match x {
		V::Value(x) => if x > 9 {
			println!("Split {}", x);
			(V::Pair(Box::new((V::Value(x/2), V::Value(x - x/2)))), true)
		} else {
			(V::Value(x), false)
		},
		V::Pair(x) => {
			let (x1, x2) = x.as_ref();
			let (n1, b) = split(x1.clone());
			if b {
				return (V::Pair(Box::new((n1, x2.clone()))), true);
			}
			let (n2, b) = split(x2.clone());
			return (V::Pair(Box::new((n1, n2))), b)
		}
	}
}

fn handle(mut x: V) -> V {
	loop {
		if can_explode(&x, 0) {
			let (nx, _, _, _) = explode(x, 0);
			x = nx;
			continue;
		}

		if can_split(&x) {
			let (nx, _) = split(x);
			x = nx;
			continue;
		}
		break;
	}
	x
}

fn exploded_right(x: V, value: i32) -> V {
	match x {
		V::Value(x) => V::Value(x + value),
		V::Pair(x) => {
			let (x1, x2) = x.as_ref();
			V::Pair(Box::new((x1.clone(), exploded_right(x2.clone(), value))))
		}
	}
}

fn exploded_left(x: V, value: i32) -> V {
	match x {
		V::Value(x) => V::Value(x + value),
		V::Pair(x) => {
			let (x1, x2) = x.as_ref();
			V::Pair(Box::new((exploded_left(x1.clone(), value), x2.clone())))
		}
	}
}