use std::fs;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2
}

fn main() {
    let input = fs::read_to_string("input/day12.txt")
        .expect("Could not read file");
    
    let lines: Vec<(&str, &str)> = input.split("\n")
        .map(|x| {
            let split: Vec<&str> = x.split("-").collect();
            (split[0], split[1])
        })
        .collect();
    
    let mut graph = HashMap::new();

    for line in lines.iter() {
        let node1 = graph.entry(line.0).or_insert(vec![]);
        node1.push(line.1);

        let node2 = graph.entry(line.1).or_insert(vec![]);
        node2.push(line.0);
    }

    let part1 = search(&graph, "start", vec![], &Part::Part1, false);

    println!("Day 12 part 1: {:?}", part1);

    let part2 = search(&graph, "start", vec![], &Part::Part2, false);

    println!("Day 12 part 2: {:?}", part2);
    
}

fn search(graph: &HashMap<&str, Vec<&str>>, target: &str, mut path: Vec<String>, part: &Part, visited_small_cave_twice: bool) -> u32 {
    if target == "end" {
        return 1;
    }

    path.push(target.to_string());

    let out = graph.get(target).unwrap();

    let mut result = 0;

    for next in out {
        let contains_next = path.contains(&next.to_string());
        let is_big = is_big(next);
        if is_big || !contains_next || (part == &Part::Part2 && !visited_small_cave_twice && next != &"start") {
            result += match contains_next && !is_big {
                true => search(graph, next, path.clone(), part, true),
                _ => search(graph, next, path.clone(), part, visited_small_cave_twice)
            }
        }
    }

    result
}

fn is_big(target: &str) -> bool {
    target.to_uppercase() == target
}