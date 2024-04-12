use std::collections::HashSet;

use itertools::Itertools;

const CAPACITY: usize = 150;

fn main() {
    let candidates: Vec<usize> = include_str!("../src/input.txt")
        .lines()
        .map(|candidate| candidate.parse().unwrap())
        .collect();

    let mut all_combinations = vec![];
    for i in 1..candidates.len() {
        candidates
            .clone()
            .into_iter()
            .combinations(i)
            .filter(|c| c.iter().sum::<usize>() == CAPACITY)
            .for_each(|vec| all_combinations.push(vec));
    }

    println!("{}", all_combinations.len());
    println!(
        "{}",
        all_combinations.iter().filter(|c| c.len() == 4).count()
    );
}
