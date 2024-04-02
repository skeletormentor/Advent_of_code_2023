fn is_increasing(line: &str) -> bool {
    for window in line.as_bytes().windows(3) {
        let third = window[2] as i8;
        let second = window[1] as i8;
        let first = window[0] as i8;
        if third - second == 1 && second - first == 1 {
            return true;
        }
    }
    false
}

fn has_iol(line: &str) -> bool {
    line.contains('i') || line.contains('o') || line.contains('l')
}

fn has_doubles(line: &str) -> bool {
    let mut count = 0;
    let mut indexes = vec![];
    for (i, window) in line.as_bytes().windows(2).enumerate() {
        if count == 2 {
            return true;
        }
        if window[0] == window[1] && !indexes.contains(&(i - 1)) {
            count += 1;
            indexes.push(i);
        }
    }
    count == 2
}

fn is_good_password(line: &str) -> bool {
    !has_iol(line) && is_increasing(line) && has_doubles(line)
}

fn increment(line: String) -> String {
    let mut chars = line.chars().rev().collect::<Vec<char>>();
    for (i, c) in chars.clone().iter().enumerate() {
        if *c == 'z' {
            chars[i] = 'a';
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            return chars.iter().rev().collect::<String>();
        }
    }
    "".to_string()
}

fn next_password(pass: String) -> String {
    let mut temp = pass;
    while !is_good_password(&temp) {
        temp = increment(temp);
    }
    temp
}

fn main() {
    let input = "vzbxkghb".to_string();
    let next_pass = next_password(input);
    println!("{next_pass}");
    println!("{}", next_password(increment(next_pass)));
}
