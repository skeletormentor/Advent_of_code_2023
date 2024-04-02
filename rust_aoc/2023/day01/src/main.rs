use std::fs;

fn read_lines(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read file")
}

fn parse_digits(s: &str) -> Vec<u32> {
    s.chars().filter_map(|d| d.to_digit(10)).collect()
}

fn main() {
    let filename = "../../input/input1.txt";
    let lines = read_lines(filename);
    let parsed = lines.lines().map(parse_digits).collect::<Vec<_>>();
    let sum = parsed
        .iter()
        .map(|line| line.first().unwrap().to_string() + &line.last().unwrap().to_string())
        .map(|strnum| strnum.parse::<u32>().unwrap())
        .sum::<u32>();
    println!("{}", sum);
}
