fn get_present_amount(num: u32) -> u32 {
    let mut nums = vec![];
    let limit = (num as f32).sqrt().round() as u32;
    let mut i = 1;
    while i <= limit {
        if num % i == 0 {
            if num / i != i {
                nums.push(num / i);
            }
            nums.push(i);
        }
        i += 1;
    }
    nums.into_iter().map(|num| 10 * num).sum()
}

fn deliver(most_presents: usize) -> Vec<usize> {
    let mut houses = vec![0; most_presents / 11];
    for elf_id in 1..houses.len() {
        let mut presents_delivered = 0;
        let mut house_id = elf_id;
        while house_id < houses.len() && presents_delivered < 50 {
            houses[house_id] += elf_id * 11;
            house_id += elf_id;
            presents_delivered += 1;
        }
    }
    houses
}

fn main() {
    let mut presents = 0;
    let input = 33_100_000;
    let mut house_number = 1;
    while presents <= input {
        presents = get_present_amount(house_number);
        house_number += 1;
    }
    println!("{}", house_number - 1);
    println!(
        "{:?}",
        deliver(input as usize)
            .iter()
            .position(|x| x >= &(input as usize))
            .unwrap()
    );
}
