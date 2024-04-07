const TIME: u32 = 2503;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    fly_duration: u32,
    rest_duration: u32,
}

fn travel(reindeer: &Reindeer, time: u32) -> u32 {
    let mut distance_travelled = 0;
    let mut is_resting = false;
    let mut resting_time = 0;
    let mut flying_time = 0;
    for _ in 1..=time {
        if flying_time == reindeer.fly_duration {
            is_resting = true;
        }
        if resting_time == reindeer.rest_duration {
            is_resting = false;
            resting_time = 0;
            flying_time = 0;
        }
        if is_resting {
            resting_time += 1;
        } else {
            distance_travelled += reindeer.speed;
            flying_time += 1;
        }
    }
    distance_travelled
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let mut reindeers = vec![];
    for line in input.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let speed = tokens[3].parse::<u32>().unwrap();
        let duration = tokens[6].parse::<u32>().unwrap();
        let resting_time = tokens[13].parse::<u32>().unwrap();
        reindeers.push(Reindeer {
            speed,
            fly_duration: duration,
            rest_duration: resting_time,
        })
    }
    reindeers
}

fn main() {
    let input = include_str!("../src/input.txt");
    let reindeers = parse_input(input);
    let max_distance = reindeers
        .into_iter()
        .map(|reindeer| travel(&reindeer, TIME))
        .max()
        .unwrap();
    println!("{}", max_distance);
}
