use std::fs;

fn parse_box(s: &str) -> [i32; 3] {
    let mut boxes: Vec<i32> = s.split('x').map(|x| x.parse::<i32>().unwrap()).collect();
    boxes.sort();
    boxes.try_into().unwrap()
}

fn count_areas(dims: &[i32; 3]) -> [i32; 3] {
    [dims[0] * dims[1], dims[1] * dims[2], dims[2] * dims[0]]
}

fn total_area(dims: &[i32; 3]) -> i32 {
    let areas = count_areas(dims);
    areas.iter().map(|x| 2 * x).sum::<i32>() + areas[0]
}

fn total_ribbon(dims: &[i32; 3]) -> i32 {
    2 * (dims[0] + dims[1]) + dims.iter().product::<i32>()
}

fn main() {
    let mut area = 0;
    let mut ribbon = 0;

    let data = fs::read_to_string("src/input.txt").unwrap();

    let lines = data.lines().collect::<Vec<_>>();
    for line in lines {
        let parsed = parse_box(line);
        area += total_area(&parsed);
        ribbon += total_ribbon(&parsed);
    }

    println!("{area}\n{ribbon}")
}
