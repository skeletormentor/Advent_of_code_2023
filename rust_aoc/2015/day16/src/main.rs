use std::collections::HashMap;

#[derive(Debug)]
struct Sue {
    num: usize,
    compounds: HashMap<String, Option<usize>>,
}

#[derive(Debug)]
struct Aunts {
    sues: Vec<Sue>,
}

impl Sue {
    fn get_compound(&self, compound: String) -> &Option<usize> {
        self.compounds.get(&compound).unwrap()
    }

    fn is_valid(&self, compound: String, value: usize) -> bool {
        match self.get_compound(compound) {
            Some(val) => val == &value,
            None => true,
        }
    }

    fn is_valid2(&self, compound: String, value: usize) -> bool {
        match compound.as_str() {
            "cats" | "trees" => match self.get_compound(compound) {
                Some(val) => val > &value,
                None => true,
            },
            "pomeranians" | "goldfish" => match self.get_compound(compound) {
                Some(val) => val < &value,
                None => true,
            },
            _ => self.is_valid(compound, value),
        }
    }
}

impl From<&str> for Aunts {
    fn from(input: &str) -> Self {
        let mut aunts: Vec<Sue> = vec![];
        for (i, line) in input.lines().enumerate() {
            let mut comp = HashMap::from([
                ("children".to_string(), None),
                ("cats".to_string(), None),
                ("samoyeds".to_string(), None),
                ("pomeranians".to_string(), None),
                ("akitas".to_string(), None),
                ("vizslas".to_string(), None),
                ("goldfish".to_string(), None),
                ("trees".to_string(), None),
                ("cars".to_string(), None),
                ("perfumes".to_string(), None),
            ]);
            let (_, compunds) = line.split_once(": ").unwrap();
            let compounds = compunds.split(',').collect::<Vec<_>>();
            for c in compounds {
                let (k, v) = c.split_once(": ").unwrap();
                let k = k.trim().to_string();
                let v = v.parse::<usize>().unwrap();
                comp.insert(k, Some(v));
            }
            aunts.push(Sue {
                num: i + 1,
                compounds: comp,
            })
        }
        Self { sues: aunts }
    }
}

fn main() {
    let input = include_str!("../src/input.txt");
    let aunts: Aunts = input.into();
    let aunt_gift: HashMap<&str, usize> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    for sue in &aunts.sues {
        let mut is_valid = true;
        for (gift, value) in &aunt_gift {
            if !sue.is_valid(gift.to_string(), *value) {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            println!("part 1: {}", sue.num)
        }
    }

    for sue in aunts.sues {
        let mut is_valid = true;
        for (gift, value) in &aunt_gift {
            if !sue.is_valid2(gift.to_string(), *value) {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            println!("part 2: {}", sue.num)
        }
    }
}
