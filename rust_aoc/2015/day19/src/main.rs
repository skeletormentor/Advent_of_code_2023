use std::collections::HashSet;

struct Machine<'a> {
    replacements: Vec<(&'a str, &'a str)>,
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

    fn new(input: &'a str) -> Self {
        let (all_repl, mut molecule) = input.split_once("\n\n").unwrap();
        molecule = molecule.trim();
        let mut replacements: Vec<(&str, &str)> = vec![];
        for repl in all_repl.lines() {
            let token: Vec<&str> = repl.split_whitespace().collect();
            replacements.push((token[0], token[2]));
        }
        Self {
            replacements,
            molecule,
        }
    }
}

fn main() {
    // let test = "HOH";
    // let mapping = vec![("H", "HO"), ("H", "OH"), ("O", "HH")];
    // let m = Machine {
    //     replacements: mapping,
    //     molecule: test,
    // };
    let input = include_str!("../src/input.txt");
    let m: Machine = Machine::new(input);
    println!("{:#?}", m.create_molecules().len());
}
