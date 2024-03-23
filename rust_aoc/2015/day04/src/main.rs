const KEY: &str = "yzbqklnj";

fn get_hash(key: &str, number_of_zeros: usize) -> u32 {
    let mut i = 1;
    let zeros = format!("{:0number_of_zeros$}", 0);
    loop {
        let concat_key = format!("{}{}", key, i);
        let hash = md5::compute(&concat_key);
        let hash = format!("{:?}", hash);
        if hash.starts_with(&zeros) {
            break i;
        }
        i += 1;
    }
}

fn main() {
    println!("{}", get_hash(KEY, 5));
    println!("{}", get_hash(KEY, 6));
}
