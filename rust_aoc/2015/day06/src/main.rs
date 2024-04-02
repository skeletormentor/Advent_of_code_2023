use regex::Regex;

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}
#[derive(Debug)]
struct Instruction {
    action: Action,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn parse_lines(input: &str) -> Vec<Instruction> {
    let lines = input.lines();

    let re =
        Regex::new(r"(?<act>on|off|toggle) (?<x1>\d+),(?<y1>\d+) through (?<x2>\d+),(?<y2>\d+)")
            .unwrap();

    lines
        .map(|line| {
            let captures = re.captures(line).expect("not valid line");
            let action = match &captures["act"] {
                "on" => Action::On,
                "off" => Action::Off,
                "toggle" => Action::Toggle,
                _ => panic!("invalid action"),
            };
            Instruction {
                action,
                x1: captures["x1"].parse::<usize>().unwrap(),
                y1: captures["y1"].parse::<usize>().unwrap(),
                x2: captures["x2"].parse::<usize>().unwrap(),
                y2: captures["y2"].parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn change_lights(grid: &mut [Vec<bool>], inst: &Instruction) {
    match inst.action {
        Action::On => {
            for y in grid[inst.y1..=inst.y2].as_mut() {
                for light in y[inst.x1..=inst.x2].as_mut() {
                    *light = true;
                }
            }
        }
        Action::Off => {
            for y in grid[inst.y1..=inst.y2].as_mut() {
                for light in y[inst.x1..=inst.x2].as_mut() {
                    *light = false;
                }
            }
        }
        Action::Toggle => {
            for y in grid[inst.y1..=inst.y2].as_mut() {
                for light in y[inst.x1..=inst.x2].as_mut() {
                    *light = !(*light);
                }
            }
        }
    }
}

fn change_lights2(grid: &mut [Vec<u8>], inst: &Instruction) {
    match inst.action {
        Action::On => {
            for y in grid[inst.y1..=inst.y2].as_mut() {
                for light in y[inst.x1..=inst.x2].as_mut() {
                    *light += 1;
                }
            }
        }
        Action::Off => {
            for y in grid[inst.y1..=inst.y2].as_mut() {
                for light in y[inst.x1..=inst.x2].as_mut() {
                    if *light > 0 {
                        *light -= 1;
                    }
                }
            }
        }
        Action::Toggle => {
            for y in grid[inst.y1..=inst.y2].as_mut() {
                for light in y[inst.x1..=inst.x2].as_mut() {
                    *light += 2;
                }
            }
        }
    }
}

fn main() {
    let lines = include_str!("../src/input.txt");
    let instructions = parse_lines(lines);
    let mut grid = vec![vec![false; 1000]; 1000];
    let mut grid2 = vec![vec![0_u8; 1000]; 1000];

    for inst in instructions {
        change_lights(&mut grid, &inst);
        change_lights2(&mut grid2, &inst);
    }

    let mut num_of_lights = 0;
    let mut total_brightness: u32 = 0;

    for row in grid {
        for is_on in row {
            if is_on {
                num_of_lights += 1;
            }
        }
    }

    for row in grid2 {
        for brightness in row {
            total_brightness += brightness as u32;
        }
    }

    println!("{}\n{}", num_of_lights, total_brightness)
}
