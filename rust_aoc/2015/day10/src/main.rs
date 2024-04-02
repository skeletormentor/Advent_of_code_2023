use indicatif::ProgressBar;

fn look_and_say(nums: String) -> String {
    let mut ans = String::new();
    let mut count = 1;
    let mut current_char = nums.chars().next().unwrap();
    for i in 1..nums.len() {
        let next_char = nums.chars().nth(i).unwrap();
        if next_char == current_char {
            count += 1;
        } else {
            ans.push_str(&format!("{count}{current_char}"));
            count = 1;
            current_char = next_char;
        }
    }
    ans.push_str(&format!("{count}{current_char}"));
    ans
}

fn generate_look_and_say_len(mut nums: String, iterations: u8) -> usize {
    let bar = ProgressBar::new(iterations.into());
    for _ in 0..iterations {
        bar.inc(1);
        nums = look_and_say(nums.to_string());
    }
    bar.finish();
    nums.len()
}

fn main() {
    let input = "3113322113".to_string();
    println!("{}", generate_look_and_say_len(input, 40));
    let input = "3113322113".to_string();
    println!("{}", generate_look_and_say_len(input, 50));
}
