use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

fn create_graph(input: &str) -> (HashMap<(&str, &str), u16>, HashSet<&str>) {
    let mut distances = HashMap::new();
    let mut cities = HashSet::new();

    input.lines().for_each(|line| {
        let (city1, city2_and_distance) = line.split_once(" to ").unwrap();
        let (city2, distance) = city2_and_distance.split_once(" = ").unwrap();
        let distance = distance.parse::<u16>().unwrap();

        distances.insert((city1, city2), distance);
        distances.insert((city2, city1), distance);

        cities.insert(city1);
        cities.insert(city2);
    });
    (distances, cities)
}

fn main() {
    let input = include_str!("../src/input.txt");
    let (distances, cities) = create_graph(input);

    let mut min_distance = u16::MAX;
    let mut max_distance = 0_u16;
    let n = cities.len();
    for route in cities.into_iter().permutations(n) {
        let dist = route
            .into_iter()
            .tuple_windows()
            .fold(0, |prev, (c1, c2)| prev + distances[&(c1, c2)]);
        min_distance = min(min_distance, dist);
        max_distance = max(max_distance, dist);
    }
    println!("{}", min_distance);
    println!("{}", max_distance);
}
