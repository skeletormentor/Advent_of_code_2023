use serde_json::Value;

fn solve(v: Value, part2: bool) -> i64 {
    if v.is_number() {
        return v.as_i64().unwrap();
    } else if v.is_string() {
        return 0;
    }

    let mut sum = 0;
    if v.is_array() {
        for x in v.as_array().unwrap() {
            sum += solve(x.clone(), part2);
        }
    } else if v.is_object() {
        for (_, x) in v.as_object().unwrap() {
            if part2 && x.is_string() && x.as_str() == Some("red") {
                return 0;
            }
            sum += solve(x.clone(), part2);
        }
    } else {
        panic!("Unexpected type of value");
    }
    sum
}


fn main() {
    let data = include_str!("../src/input.txt");
    let json: Value = serde_json::from_str(data).unwrap();
    let sum = solve(json.clone(), false);
    println!("{}", sum);
    let sum = solve(json, true);
    println!("{}", sum);
}
