use serde_json::{Result, Value};

fn main() {
    let data = include_str!("../src/input.txt");
    let json: Value = serde_json::from_str(data).unwrap();
    println!("{:?}", json);
}
