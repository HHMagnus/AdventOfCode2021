use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn main() {
    let read = fs::read_to_string("input/day13.txt")
        .expect("Could not read file");

    let input: Vec<Vec<&str>> = read
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect();

    let points_vector: Vec<(i32, i32)> = input[0].iter()
        .map(|x| {
            let s: Vec<&str> = x.split(",").collect();
            (s[0].parse().unwrap(), s[1].parse().unwrap())
        })
        .collect();

    let mut points: HashSet<(i32, i32)> = HashSet::from_iter(points_vector.iter().cloned());

    let folds: Vec<Fold> = input[1].iter()
        .map(|x| {
            let s: Vec<&str> = x.split("=").collect();
            match s[0] {
                "fold along x" => Fold::X(s[1].parse().unwrap()),
                _ => Fold::Y(s[1].parse().unwrap())
            }
        })
        .collect();

    let mut points1 = points.clone();

    fold(&mut points1, &folds[0]);

    println!("Day 13 part 1: {:?}", points1.len());

    for f in folds {
        fold(&mut points, &f);
    }

    let max_x = points.iter().map(|x| x.0).max().unwrap();
    let max_y = points.iter().map(|x| x.1).max().unwrap();

    println!("Day 13 part 2:");
    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            print!("{}", if points.contains(&(x,y)) { "#" } else { "." });
        }
        print!("\n");
    }
}

fn fold(points: &mut HashSet<(i32, i32)>, fold: &Fold) {
    match *fold {
        Fold::X(x) => fold_x(points, x),
        Fold::Y(y) => fold_y(points, y),
    }
}

fn fold_x(points: &mut HashSet<(i32, i32)>, x: i32) {
    let below_fold: Vec<(i32, i32)> = points.iter().cloned().filter(|z| z.0 > x).collect();

    for point in below_fold {
        points.remove(&point);
        points.insert((x - (point.0 - x), point.1));
    }
}

fn fold_y(points: &mut HashSet<(i32, i32)>, y: i32) {
    let below_fold: Vec<(i32, i32)> = points.iter().cloned().filter(|z| z.1 > y).collect();

    for point in below_fold {
        points.remove(&point);
        points.insert((point.0, y - (point.1 - y)));
    }
}