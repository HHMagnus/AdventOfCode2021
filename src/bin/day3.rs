use std::fs;

fn main() {
    let lines: Vec<Vec<i32>> = fs::read_to_string("input/day3.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|x| x.chars().map(|x| if x == '1' { 1 } else { -1 }).collect())
        .collect();

    let mut summed = vec![0; 12];

    lines.iter().for_each(|x| x.iter().enumerate().for_each(|(i, y)| summed[i.clone() as usize] += y));

    let most_common: Vec<bool> = summed.iter().map(|x| (x >= &0)).collect();
    let least_common: Vec<bool> = summed.iter().map(|x| (x < &0)).collect();

    let gamma = most_common.iter().fold(0, |acc, b| acc*2 + *b as i32);
    let epsilon = least_common.iter().fold(0, |acc, b| acc*2 + *b as i32);

    let part1 = gamma * epsilon;

    println!("Day3 part1: {}", part1);

    let mut oxygen_vectors = lines.clone();
    let mut co2_vectors = lines.clone();

    for i in 0..12 {
        if oxygen_vectors.len() > 1 {
            let mut summed = vec![0; 12];
            oxygen_vectors.iter().for_each(|x| x.iter().enumerate().for_each(|(i, y)| summed[i.clone() as usize] += y));

            oxygen_vectors = oxygen_vectors
                .iter()
                .filter(|x| x[i] == 1 && summed[i] >= 0 || x[i] == -1 && summed[i] < 0)
                .map(|x| x.clone())
                .collect();
        }

        if co2_vectors.len() > 1 {
            let mut summed = vec![0; 12];
            co2_vectors.iter().for_each(|x| x.iter().enumerate().for_each(|(i, y)| summed[i.clone() as usize] += y));
            co2_vectors = co2_vectors
                .iter()
                .filter(|x| x[i] == -1 && summed[i] >= 0 || x[i] == 1 && summed[i] < 0)
                .map(|x| x.clone())
                .collect();
        }
    }

    let oxygen_vector: Vec<i32> = oxygen_vectors[0].clone().iter().map(|x| (*x == 1) as i32).collect();
    let co2_vector: Vec<i32> = co2_vectors[0].clone().iter().map(|x| (*x == 1) as i32).collect();

    let oxygen = oxygen_vector.iter().fold(0, |acc, b| acc*2 + *b as i32);
    let co2 = co2_vector.iter().fold(0, |acc, b| acc*2 + *b as i32);

    let part2 = oxygen * co2;

    println!("Day3 part2: {}", part2);
}