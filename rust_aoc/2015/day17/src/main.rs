// const CAPACITY: isize = 150;

fn main() {
    let candidates = include_str!("../src/input.txt");
    let mut candidates = candidates
        .lines()
        .map(|candidate| candidate.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut candidates = vec![20, 15, 10, 5, 5];
    candidates.sort();

    let mut combinations: Vec<Vec<usize>> = vec![vec![]];
    let mut current = vec![];
    find_combinations(&mut combinations, &mut current, &candidates, 25, 0);
    println!("{:?}", combinations)
}

fn find_combinations(
    combinations: &mut Vec<Vec<usize>>,
    current_combination: &mut Vec<usize>,
    candidates: &Vec<usize>,
    remaining_sum: isize,
    start: usize,
) {
    if remaining_sum < 0 {
        return;
    } else if remaining_sum == 0 {
        combinations.push(current_combination.to_vec())
    } else {
        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }
            current_combination.push(candidates[i]);
            find_combinations(
                combinations,
                current_combination,
                candidates,
                remaining_sum - candidates[i] as isize,
                i + 1,
            );
            current_combination.pop();
        }
    }
}
