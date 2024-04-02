use once_cell::sync::Lazy;
use regex::Regex;

fn count_chars(input: &str) -> usize {
    static ESCAPE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\\[\"\\]"#).unwrap());
    static ASCII_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\\x[a-f0-9]{2}"#).unwrap());
    let escape_count = ESCAPE_PATTERN.captures_iter(input).count();
    let ascii_count = ASCII_PATTERN.captures_iter(input).count();
    input.len() - 2 - escape_count - 3 * ascii_count
}

fn count_chars2(input: &str) -> usize {
    static ESCAPE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\\[\"\\]"#).unwrap());
    static ASCII_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\\x[a-f0-9]{2}"#).unwrap());
    let escape_count = ESCAPE_PATTERN.captures_iter(input).count();
    let ascii_count = ASCII_PATTERN.captures_iter(input).count();
    input.len() + 4 + 2 * escape_count + ascii_count
}

fn main() {
    let input = include_str!("../src/input.txt").trim_end();
    let mut total_length = 0;
    let mut total_length2 = 0;
    input.split('\n').map(|line| line.trim()).for_each(|line| {
        total_length += line.len() - count_chars(line);
        total_length2 += count_chars2(line) - line.len();
    });
    println!("{}\n{}", total_length, total_length2)
}
