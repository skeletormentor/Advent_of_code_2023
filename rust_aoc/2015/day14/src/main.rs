const TIME: u16 = 2503;

struct Reindeer {
    speed: u16,
    fly_duration: u16,
    rest_duration: u16,
}

fn travel(reindeer: &Reindeer, time: u16) -> u16 {
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

fn get_max_distance(reindeers: &[Reindeer], time: u16) -> u16 {
    reindeers
        .iter()
        .map(|reindeer| travel(reindeer, time))
        .max()
        .unwrap()
}

fn get_max_points(reindeers: &[Reindeer], time: u16) -> u16 {
    let mut points = vec![0; reindeers.len()];

    for i in 1..=time {
        let round = reindeers
            .iter()
            .map(|reindeer| travel(reindeer, i))
            .collect::<Vec<_>>();
        let max_val = round.iter().max().unwrap();
        for j in 0..reindeers.len() {
            if round[j] == *max_val {
                points[j] += 1;
            }
        }
    }
    points.into_iter().max().unwrap()
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let mut reindeers = vec![];
    for line in input.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let speed = tokens[3].parse::<u16>().unwrap();
        let duration = tokens[6].parse::<u16>().unwrap();
        let resting_time = tokens[13].parse::<u16>().unwrap();
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

    println!("{}", get_max_distance(&reindeers, TIME));
    println!("{}", get_max_points(&reindeers, TIME));
}
