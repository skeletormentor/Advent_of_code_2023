use std::collections::{HashMap, HashSet};

struct Machine<'a> {
    replacements: Vec<(&'a str, &'a str)>,
    electrons: Vec<&'a str>,
    molecule: &'a str,
}

impl<'a> Machine<'a> {
    fn create_molecules(&self) -> HashSet<String> {
        let mut molecules = HashSet::new();
        let len = self.molecule.len();
        for i in 0..len {
            let current = &self.molecule[i..len];
            for (pat, to) in &self.replacements {
                let mut replaced = current.replacen(*pat, *to, 1);
                replaced.insert_str(0, &self.molecule[..i]);
                molecules.insert(replaced);
            }
        }
        molecules.remove(self.molecule);
        molecules
    }

    fn from(input: &'a str) -> Self {
        let (all_repl, molecule) = input.trim().split_once("\n\n").unwrap();
        let mut replacements: Vec<(&str, &str)> = vec![];
        let mut electrons: Vec<&str> = vec![];
        for repl in all_repl.lines() {
            let token: Vec<&str> = repl.split_whitespace().collect();
            let (left, right) = (token[0], token[2]);
            match left {
                "e" => electrons.push(right),
                _ => replacements.push((left, right)),
            };
        }
        Self {
            replacements,
            electrons,
            molecule,
        }
    }

    fn get_mapping(&self) -> HashMap<&str, &str> {
        self.replacements
            .iter()
            .fold(HashMap::new(), |mut map, (from, to)| {
                map.insert(*to, *from);
                map
            })
    }

    fn get_keys(&self) -> Vec<&str> {
        self.replacements.iter().map(|(_, to)| *to).collect()
    }

    fn count_steps(&self) -> i32 {
        let mapping = self.get_mapping();
        let keys = self.get_keys();
        let mut count = 0;
        let mut temp_molecule = self.molecule.to_string();
        loop {
            for k in keys.iter() {
                if temp_molecule.contains(k) {
                    let from_element = mapping.get(k).unwrap();
                    temp_molecule = temp_molecule.replacen(k, *from_element, 1);
                    count += 1;
                    break;
                }
            }
            if self.electrons.contains(&temp_molecule.as_str()) {
                count += 1;
                break;
            }
        }
        count
    }
}

fn main() {
    let input = include_str!("../src/input.txt");
    let m: Machine = Machine::from(input);
    println!("{:#?}", m.create_molecules().len());
    println!("{:#?}", m.count_steps())
}
