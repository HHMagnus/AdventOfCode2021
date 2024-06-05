use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let input = fs::read_to_string("input/day19.txt")
        .expect("Could not read file");

	let lines = input
		.split("\n\n")
		.map(|x| x.lines().collect::<Vec<_>>()[1..].iter().map(|x| {
			let split = x.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
			(split[0], split[1], split[2])
		}).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let scan1 = &lines[0];
	let scan2 = &lines[1];

	let dist1 = distmap(scan1);
	let dist2 = distmap(scan2);
	
	let mut mapping = Vec::new();
	for dist in dist1 {
		if let Some(corresponding) = dist2.iter().find(|(_, x)| x.iter().filter(|y| dist.1.contains(y)).count() > 10).map(|(y, _)| y) {
			mapping.push((dist.0, *corresponding));
		}
	}

	let basis = find_basis(mapping[0], mapping[1]); 

	println!("{:?}", basis);

}

fn find_basis((p11, p12): ((i32, i32, i32), (i32, i32, i32)), (p21, p22): ((i32, i32, i32), (i32, i32, i32))) -> ([(usize, i32); 3], [i32; 3]) {
	let vec1 = [p21.0 - p11.0, p21.1 - p11.1, p21.2 - p11.2];
	let vec2 = [p22.0 - p12.0, p22.1 - p12.1, p22.2 - p12.2];

	let mut rotate = [(0, 0); 3];
	for i in 0..=2 {
		for j in 0..=2 {
			if vec1[i].abs() == vec2[j].abs() {
				rotate[i] = (j, vec1[i] / vec2[j]);
			}
		}
	}

	let original2 = [p12.0, p12.1, p12.2];
	let mut vec2r = [0; 3];

	for i in 0..=2 {
		vec2r[i] = original2[rotate[i].0] * rotate[i].1;
	}

	let original1 = [p11.0, p11.1, p11.2];
	let mut transform = [0; 3];

	for i in 0..=2 {
		transform[i] = original1[i] - vec2r[i];
	}

	(rotate, transform)
}

fn transform(p: (i32, i32, i32), basis: ([(usize, i32); 3], [i32; 3])) -> (i32, i32, i32) {
	let vec = [p.0, p.1, p.2];

	let mut res = [0; 3];

	for i in 0..=2 {
		let (rotation, sign) = basis.0[i];
		res[i] = vec[rotation] * sign + basis.1[i];
	}

	(res[0], res[1], res[2])
}

fn distmap(vs: &Vec<(i32, i32, i32)>) -> HashMap<(i32, i32, i32), Vec<i32>> {
	let mut map = HashMap::new();

	for v in vs {
		let mut vec = Vec::new();
		for o in vs {
			if v == o {
				continue;
			}
			vec.push(dist(v, o))
		}
		map.insert(v.clone(), vec);
	}

	map
}

fn dist(x: &(i32, i32, i32), y: &(i32, i32, i32)) -> i32 {
	(x.0 - y.0).abs() + (x.1 - y.1).abs() + (x.2 - y.2).abs()
}