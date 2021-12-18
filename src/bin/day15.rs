use std::fs;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Move {
    pos: (u32, u32),
    cost: u32,
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mace: Vec<Vec<u32>> = fs::read_to_string("input/day15.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();
        
    let part1 = shortest_to_end(&mace, (mace.len() as u32, mace[0].len() as u32));

    println!("Day 15 part 1: {:?}", part1);
        
    let part2 = shortest_to_end(&mace, ((mace.len()*5) as u32, (mace[0].len()*5) as u32));

    println!("Day 15 part 2: {:?}", part2);

}

fn shortest_to_end(mace: &Vec<Vec<u32>>, end: (u32, u32)) -> u32 {
    let mut visited: HashSet<(u32, u32)> = HashSet::new();

    let mut heap = BinaryHeap::new();

    visited.insert((0,0));
    heap.push(Move { pos: (0,0), cost: 0 });

    while let Some(Move { pos, cost }) = heap.pop() {
        if pos == (end.0-1,end.1-1) {
            return cost;
        }

        if pos.0 > 0 && !visited.contains(&(pos.0-1,pos.1)){
            let new_pos = (pos.0-1, pos.1);
            visited.insert(new_pos);
            let added_cost = calc_cost(mace, &new_pos);
            heap.push(Move { pos: new_pos, cost: cost + added_cost });
        }
        if pos.1 > 0 && !visited.contains(&(pos.0,pos.1-1)) {
            let new_pos = (pos.0, pos.1-1);
            visited.insert(new_pos);
            let added_cost = calc_cost(mace, &new_pos);
            heap.push(Move { pos: new_pos, cost: cost + added_cost });
        }
        if pos.0+1 < end.0 && !visited.contains(&(pos.0+1,pos.1)) {
            let new_pos = (pos.0+1, pos.1);
            visited.insert(new_pos);
            let added_cost = calc_cost(mace, &new_pos);
            heap.push(Move { pos: new_pos, cost: cost + added_cost });
        }
        if pos.1+1 < end.1 && !visited.contains(&(pos.0,pos.1+1)) {
            let new_pos = (pos.0, pos.1+1);
            visited.insert(new_pos);
            let added_cost = calc_cost(mace, &new_pos);
            heap.push(Move { pos: new_pos, cost: cost + added_cost });
        }
    }

    panic!("No end");
}

fn calc_cost(mace: &Vec<Vec<u32>>, pos: &(u32, u32)) -> u32 {
    let added = (pos.0/mace.len() as u32) + (pos.1/mace[0].len() as u32);
    let risk = mace[pos.0 as usize % mace.len()][pos.1 as usize % mace[0].len()];
    if added > 0 {
        let new_risk = risk + added;
        if new_risk > 9 {
            new_risk % 10 + 1
        } else {
            new_risk
        }
    } else {
        risk
    }
}