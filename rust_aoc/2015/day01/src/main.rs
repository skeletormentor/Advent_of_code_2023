use std::fs;

fn mv(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("wrong char"),
        }
    }
    floor
}

fn mv2(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("wrong char"),
        }
        dbg!(&floor);
        if floor == -1 {
            return i + 1;
        }
    }
    0
}

fn get_input() -> String {
    fs::read_to_string("src/input.txt").unwrap()
}

fn main() {
    println!("{}", mv(get_input().as_str()));
    println!("{}", mv2(get_input().as_str()));
}
