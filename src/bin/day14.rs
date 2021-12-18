use std::fs;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let read = fs::read_to_string("input/day14.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = read
        .split("\n")
        .collect();

    let template: Vec<char> = lines[0].chars().collect();

    let rules: Vec<((char,char), char)> = lines[2..].iter()
        .map(|x| {
            let s: Vec<&str> = x.split(" -> ").collect();
            let c0: Vec<char> = s[0].chars().collect();
            let c1: Vec<char> = s[1].chars().collect();
            ((c0[0], c0[1]), c1[0])
        })
        .collect();

    let rules: HashMap<(char,char), char> = HashMap::from_iter(rules);

    let mut result: HashMap<(char,char), u64> = HashMap::new();

    for window in template.as_slice().windows(2) {
        let r = result.entry((window[0], window[1])).or_insert(0);
        *r += 1;
    }

    let mut part1_result = result.clone();

    for _step in 0..10 {
        part1_result = apply_rules(&part1_result, &rules);
    }

    let part1 = get_result(&part1_result, &template);

    println!("Day 14 part 1: {:?}", part1);

    let mut part2_result = result.clone();

    for _step in 0..40 {
        part2_result = apply_rules(&part2_result, &rules);
    }

    let part2 = get_result(&part2_result, &template);

    println!("Day 14 part 2: {:?}", part2);
}

fn apply_rules(start: &HashMap<(char,char), u64>, rules: &HashMap<(char,char), char>) -> HashMap<(char,char), u64>{
    let mut result = HashMap::new();

    for x in start.iter() {
        let rule = rules.get(x.0).unwrap();
        let new1 = result.entry((x.0.0, *rule)).or_insert(0);
        *new1 += x.1;
        let new2 = result.entry((*rule, x.0.1)).or_insert(0);
        *new2 += x.1;
    }

    result
}

fn get_result(res: &HashMap<(char,char), u64>, template: &Vec<char>) -> u64 {
    let mut chars: HashMap<char, u64> = HashMap::new();
    for x in res {
        let char1 = chars.entry(x.0.0).or_insert(0);
        *char1 += x.1;
        let char2 = chars.entry(x.0.1).or_insert(0);
        *char2 += x.1;
    }
    let start = template[0];
    let end = template[template.len() - 1];

    let mut values: Vec<u64> = chars.iter().map(|(c,l)| {
        match c {
            x if *x == start || *x == end => (l-1)/2+1,
            _ => l/2
        }
    }).collect();
    values.sort();
    values[values.len()-1] - values[0]
}