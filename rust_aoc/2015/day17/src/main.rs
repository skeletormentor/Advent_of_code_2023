use itertools::Itertools;

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
            .filter(|c| c.iter().sum::<usize>() == 150)
            .for_each(|vec| all_combinations.push(vec));
    }
    let min_count_combinations = all_combinations
        .iter()
        .map(|combination| combination.len())
        .min()
        .unwrap();
    println!("{}", all_combinations.len());
    println!(
        "{}",
        all_combinations
            .iter()
            .filter(|c| c.len() == min_count_combinations)
            .count()
    );
}
