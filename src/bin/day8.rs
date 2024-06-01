use std::fs;

fn main() {
    let input = fs::read_to_string("input/day8.txt")
        .expect("Could not read file");

    let lines: Vec<Vec<Vec<&str>>> = input
        .split("\n")
        .map(|x| x.split(" | ").map(|y| y.split(" ").collect()).collect())
        .collect();

    let part1: usize = lines
        .iter()
        .map(|x| x[1].iter().filter(|y| y.len() == 2 || y.len() == 4 || y.len() == 3 || y.len() == 7).count())
        .sum();

    println!("Day 8 part 1: {:?}", part1);

    let part2: u32 = lines.iter()
        .map(|x| entry(x))
        .sum();

    println!("Day 8 part 2: {:?}", part2);
}

fn entry(code: &Vec<Vec<&str>>) -> u32 {
    let top = find_top(&code[0]);
    let (left_top, middle) = find_left_top_and_middle(&code[0]);
    let (right_bottom, right_top) = find_right_top_and_right_bottom(&code[0], &middle);
    let bottom = find_bottom(&code[0], &top, &right_top, &right_bottom, &middle);
	let left_bottom = find_left_bottom(&code[0], &top, &right_top, &right_bottom, &middle, &bottom, &left_top);

    let numbers = vec![
        vec![top, left_top, left_bottom, right_top, right_bottom, bottom],
        vec![right_top, right_bottom],
        vec![top, right_top, middle, left_bottom, bottom],
        vec![top, right_top, middle, right_bottom, bottom],
        vec![left_top, right_top, middle, right_bottom],
        vec![top, left_top, middle, right_bottom, bottom],
        vec![top, left_top, middle, right_bottom, left_bottom, bottom],
        vec![top, right_top, right_bottom],
        vec![top, right_top, left_top, middle, left_bottom, right_bottom, bottom],
        vec![top, left_top, right_top, middle, right_bottom, bottom],
    ];

    let translated: Vec<u32> = code[1].iter()
        .map(|x| translate(&numbers, x))
        .collect();
    
    translated[0] * 1000 + translated[1] * 100 + translated[2] * 10 + translated[3]
}

fn find_top(code: &Vec<&str>) -> char {// resolve t by comparing 1 and 7
    let one = code.iter().find(|x| x.len() == 2).unwrap();
	let seven = code.iter().find(|x| x.len() == 3).unwrap();
	seven.chars().find(|&x| !one.contains(x)).unwrap()
}

fn find_left_top_and_middle(code: &Vec<&str>) -> (char, char) {// resolve lt and m by finding 3 by comparing with 1 and removing all 4
	let one = code.iter().find(|x| x.len() == 2).unwrap();
	let four = code.iter().find(|x| x.len() == 4).unwrap();
	let three = code.iter().find(|x| x.len() == 5 && one.chars().all(|y| x.contains(y))).unwrap();
	let lt = four.chars().find(|&x| !three.contains(x)).unwrap();
	let m = three.chars().filter(|&x| four.contains(x)).find(|&x| !one.contains(x)).unwrap();
    (lt, m)
}

fn find_right_top_and_right_bottom(code: &Vec<&str>, middle: &char) -> (char, char) {// resolve rt and rb by finding 6 and comparing it to 1
	let one = code.iter().find(|x| x.len() == 2).unwrap();
	let six = code.iter().find(|x| x.len() == 6 && x.contains(*middle) && !one.chars().all(|y| x.contains(y))).unwrap();
	let one = code.iter().find(|x| x.len() == 2).unwrap();
	let rt = one.chars().find(|&x| !six.contains(x)).unwrap();
	let rb = one.chars().find(|&x| six.contains(x)).unwrap();
    (rb, rt)
}

fn find_bottom(code: &Vec<&str>, top: &char, rt: &char, rb: &char, middle: &char) -> char {// resolve b by finding unknown in 3
	let three = code.iter().find(|x| x.len() == 5 && x.contains(*rt) && x.contains(*rb)).unwrap();
	three.chars().find(|x| x != top && x != rt && x != rb && x != middle).unwrap()
}

fn find_left_bottom(code: &Vec<&str>, top: &char, rt: &char, rb: &char, middle: &char, bottom: &char, left_top: &char) -> char {
	let eight = code.iter().find(|x| x.len() == 7).unwrap();
	eight.chars().find(|x| x != top && x != rt && x != rb && x != middle && x != bottom && x != left_top).unwrap()
}

fn translate(numbers: &Vec<Vec<char>>, chars: &str) -> u32 {
	for i in 0..10 {
		let contains_all = chars.chars().all(|x| numbers[i].contains(&x));
		let all_contained = numbers[i].iter().all(|&x| chars.contains(x));
		if contains_all && all_contained {
			return i as u32;
		}
	}
	panic!("Could not trnslate");
}