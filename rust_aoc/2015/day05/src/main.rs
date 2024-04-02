use fancy_regex::Regex;
use once_cell::sync::Lazy;

fn is_fancy(word: &str) -> bool {
    static RE1: Lazy<Regex> = Lazy::new(|| Regex::new(r".*[aeiou].*[aeiou].*[aeiou].*").unwrap());
    static RE2: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.)(\1)").unwrap());
    static RE3: Lazy<Regex> = Lazy::new(|| Regex::new(r"ab|cd|pq|xy").unwrap());

    RE1.is_match(word).unwrap() && RE2.is_match(word).unwrap() && !RE3.is_match(word).unwrap()
}

fn is_fancy2(word: &str) -> bool {
    static RE4: Lazy<Regex> = Lazy::new(|| Regex::new(r"(..).*\1").unwrap());
    static RE5: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.).\1").unwrap());

    RE4.is_match(word).unwrap() && RE5.is_match(word).unwrap()
}

fn main() {
    let input = include_str!("../src/input.txt");

    let part1 = input.lines().filter(|word| is_fancy(word)).count();
    let part2 = input.lines().filter(|word| is_fancy2(word)).count();

    println!("{part1}\n{part2}")
}
