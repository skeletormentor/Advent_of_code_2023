use std::{collections::HashSet, fs};

fn count_visited_houses(directions: &str) -> HashSet<[i32; 2]> {
    let mut houses = HashSet::new();
    let mut loc = [0, 0];

    houses.insert(loc);

    for direction in directions.chars() {
        match direction {
            '>' => loc[0] += 1,
            '^' => loc[1] += 1,
            '<' => loc[0] -= 1,
            'v' => loc[1] -= 1,
            _ => panic!("invalid char: {direction}"),
        }
        houses.insert(loc);
    }
    houses
}
fn main() {
    let data = fs::read_to_string("src/input.txt").unwrap();

    let santa = &data.chars().step_by(2).collect::<String>();
    let robot = &data.chars().skip(1).step_by(2).collect::<String>();

    let mut santa_and_robot = count_visited_houses(santa);
    santa_and_robot.extend(count_visited_houses(robot));

    println!(
        "{}\n{}",
        count_visited_houses(&data).len(),
        santa_and_robot.len()
    )
}
