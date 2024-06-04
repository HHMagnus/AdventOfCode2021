use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Var {
	W,
	X,
	Y,
	Z,
}

#[derive(Debug)]
enum VarNum {
	Var(Var),
	Num(i64),
}

#[derive(Debug)]
enum Operation {
	Add,
	Mul,
	Div,
	Mod,
	Eql,
}

#[derive(Debug)]
enum Op {
	Inp(Var),
	Op(Operation, Var, VarNum),
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Alu {
	w: i64,
	x: i64,
	y: i64,
	z: i64,
}

impl Alu {
	fn inp(&self, x: &Var, res: i64) -> Alu {
		match x {
			Var::W => Alu {
				w: res,
				x: self.x,
				y: self.y,
				z: self.z,
			},
			Var::X => Alu {
				w: self.w,
				x: res,
				y: self.y,
				z: self.z,
			},
			Var::Y => Alu {
				w: self.w,
				x: self.x,
				y: res,
				z: self.z,
			},
			Var::Z => Alu {
				w: self.w,
				x: self.x,
				y: self.y,
				z: res,
			},
		}
	}
	fn apply(&self, ope: &Operation, a: &Var, b: &VarNum) -> Alu {
		let av = match a {
			Var::W => self.w,
			Var::X => self.x,
			Var::Y => self.y,
			Var::Z => self.z,
		};

		let bv = match b {
			VarNum::Num(v) => *v,
			VarNum::Var(var) => match var {
				Var::W => self.w,
				Var::X => self.x,
				Var::Y => self.y,
				Var::Z => self.z,
			}
		};

		let res = match ope {
			Operation::Add => av + bv,
			Operation::Div => av / bv,
			Operation::Eql => if av == bv { 1 } else { 0 },
			Operation::Mod => av % bv,
			Operation::Mul => av * bv,
		};

		match a {
			Var::W => Alu {
				w: res,
				x: self.x,
				y: self.y,
				z: self.z,
			},
			Var::X => Alu {
				w: self.w,
				x: res,
				y: self.y,
				z: self.z,
			},
			Var::Y => Alu {
				w: self.w,
				x: self.x,
				y: res,
				z: self.z,
			},
			Var::Z => Alu {
				w: self.w,
				x: self.x,
				y: self.y,
				z: res,
			},
		}
	}
}

fn parse_var(x: &str) -> Var {
	match x {
		"w" => Var::W,
		"x" => Var::X,
		"y" => Var::Y,
		"z" => Var::Z,
		x => panic!("Unknown {}", x),
	}
}

fn parse_num(x: &str) -> VarNum {
	match x {
		"w" => VarNum::Var(Var::W),
		"x" => VarNum::Var(Var::X),
		"y" => VarNum::Var(Var::Y),
		"z" => VarNum::Var(Var::Z),
		x => VarNum::Num(x.parse().unwrap())
	}
}

fn main() {
    let input = fs::read_to_string("input/day24.txt")
        .expect("Could not read file");

	let mapped = input.lines()
		.map(|x| {
			let s = x.split(" ").collect::<Vec<_>>();
			match s[0] {
				"inp" => Op::Inp(parse_var(s[1])),
				"add" => Op::Op(Operation::Add, parse_var(s[1]), parse_num(s[2])),
				"mul" => Op::Op(Operation::Mul, parse_var(s[1]), parse_num(s[2])),
				"div" => Op::Op(Operation::Div, parse_var(s[1]), parse_num(s[2])),
				"mod" => Op::Op(Operation::Mod, parse_var(s[1]), parse_num(s[2])),
				"eql" => Op::Op(Operation::Eql, parse_var(s[1]), parse_num(s[2])),
				x => panic!("Did not know {}", x)
			}
		}).collect::<Vec<_>>();

	let (part1, part2) = find_monad(&mapped);

	println!("Day 24 part 1: {}", part1);
	println!("Day 24 part 2: {}", part2);
}

fn find_monad(ops: &Vec<Op>) -> (i64, i64) {
	
	let mut alus: Vec<(Alu, i64, i64)> = vec![(Alu {x: 0, y: 0, w: 0, z: 0}, 0, 0)];

	for op in ops {
		match op {
			Op::Inp(var) => {
				let mut map = HashMap::new();
				for (alu, max, min) in &alus {
					for i in 1..10 {
						let nalu = alu.inp(var, i);
						let max = *max * 10 + i;
						let min = *min * 10 + i;
						if let Some((omax, omin)) = map.get(&nalu) {
							let max = max.max(*omax);
							let min = min.min(*omin);
							*map.get_mut(&nalu).unwrap() = (max, min);
						} else {
							map.insert(nalu, (max, min));
						}
					}
				}
				alus = map.into_iter().map(|x| (x.0, x.1.0, x.1.1)).collect();
				println!("Inserting with {} states", alus.len());
			},
			Op::Op(ope, a, b) => {
				alus = alus.into_iter().map(|x| (x.0.apply(ope, a, b), x.1, x.2)).collect();
			}
		}
	}

	let valid_answers: Vec<_> = alus.into_iter().filter(|x| x.0.z == 0).collect();
	
	let max = valid_answers.iter().map(|x| x.1).max().unwrap();
	let min = valid_answers.iter().map(|x| x.2).min().unwrap();

	(max, min)
}