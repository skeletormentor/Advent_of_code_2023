use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn count_happiness(names: &[&str], pairs: &HashMap<(&str, &str), i16>) -> i16 {
    names
        .iter()
        .circular_tuple_windows()
        .fold(0, |acc, (current, next)| {
            acc + pairs[&(*current, *next)] + pairs[&(*next, *current)]
        })
}

fn parse_input(input: &str) -> (HashMap<(&str, &str), i16>, HashSet<&str>) {
    let mut pairs = HashMap::new();
    let mut names = HashSet::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let name = parts[0];
        let gain_or_lose = parts[2];
        let mut num = parts[3].parse::<i16>().unwrap();
        if gain_or_lose == "lose" {
            num *= -1;
        }
        let next = parts[10];
        let next = &next[..(next.len() - 1)];
        pairs.insert((name, next), num);
        names.insert(name);
    }
    (pairs, names)
}

fn max_happiness_all_permutations(
    pairs: &HashMap<(&str, &str), i16>,
    names: &HashSet<&str>,
) -> i16 {
    names
        .clone()
        .into_iter()
        .permutations(names.len())
        .map(|conf| count_happiness(&conf, pairs))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../src/input.txt");
    let (mut pairs, mut names) = parse_input(input);

    println!("{}", max_happiness_all_permutations(&pairs, &names));

    for name in names.iter() {
        pairs.insert(("You", *name), 0);
        pairs.insert((*name, "You"), 0);
    }
    names.insert("You");

    println!("{}", max_happiness_all_permutations(&pairs, &names));
}
