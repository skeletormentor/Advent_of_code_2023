use std::{collections::HashSet, str::FromStr, string::ParseError};

struct Machine<'a> {
    replacements: Vec<(&'a str, &'a str)>,
    molecule: &'a str,
}

impl<'a> Machine<'a> {
    fn create_molecules(&self) -> HashSet<String> {
        let mut molecules = HashSet::new();
        let len = self.molecule.len();
        for i in 0..(len - 1) {
            let current = &self.molecule[i..len];
            for (pat, to) in &self.replacements {
                let mut replaced = current.replacen(*pat, *to, 1);
                replaced.insert_str(0, &self.molecule[..i]);
                molecules.insert(replaced);
            }
        }
        molecules
    }
}

impl FromStr for Machine<'_> {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (all_repl, molecule) = input.split_once("\n\n").unwrap();
        let mut replacements: Vec<(&str, &str)> = vec![];
        for repl in all_repl.lines() {
            let token: Vec<&str> = repl.split_whitespace().collect();
            replacements.push((token[0], token[2]));
        }
        Ok(Machine {
            replacements,
            molecule,
        })
    }
}

fn main() {
    // let test = "HOHOHO";
    // let mapping = vec![("H", "HO"), ("H", "OH"), ("O", "HH")];
    // let m = Machine {
    //     replacements: mapping,
    //     molecule: test,
    // };
    // println!("{:#?}", m.create_molecules());
    let input = include_str!("../src/input.txt");
    let m: Machine = input.parse().unwrap();
}
