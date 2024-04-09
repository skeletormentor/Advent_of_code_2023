use itertools::Itertools;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Debug)]

struct Cookie {
    ingredients: Vec<Ingredient>,
}

impl From<&str> for Ingredient {
    fn from(input: &str) -> Self {
        let tokens = input.split_whitespace().collect::<Vec<_>>();
        Self {
            capacity: tokens[2].strip_suffix(',').unwrap().parse().unwrap(),
            durability: tokens[4].strip_suffix(',').unwrap().parse().unwrap(),
            flavor: tokens[6].strip_suffix(',').unwrap().parse().unwrap(),
            texture: tokens[8].strip_suffix(',').unwrap().parse().unwrap(),
            calories: tokens[10].parse().unwrap(),
        }
    }
}

impl Cookie {
    fn get_capacity(&self, multipliers: &[i32]) -> i32 {
        let mut s = 0;
        for (i, ingredient) in self.ingredients.iter().enumerate() {
            s += multipliers[i] * ingredient.capacity
        }
        if s < 0 {
            0
        } else {
            s
        }
    }

    fn get_durability(&self, multipliers: &[i32]) -> i32 {
        let mut s = 0;
        for (i, ingredient) in self.ingredients.iter().enumerate() {
            s += multipliers[i] * ingredient.durability
        }
        if s < 0 {
            0
        } else {
            s
        }
    }

    fn get_flavor(&self, multipliers: &[i32]) -> i32 {
        let mut s = 0;
        for (i, ingredient) in self.ingredients.iter().enumerate() {
            s += multipliers[i] * ingredient.flavor
        }
        if s < 0 {
            0
        } else {
            s
        }
    }

    fn get_texture(&self, multipliers: &[i32]) -> i32 {
        let mut s = 0;
        for (i, ingredient) in self.ingredients.iter().enumerate() {
            s += multipliers[i] * ingredient.texture
        }
        if s < 0 {
            0
        } else {
            s
        }
    }

    fn get_calories(&self, multipliers: &[i32]) -> i32 {
        let mut s = 0;
        for (i, ingredient) in self.ingredients.iter().enumerate() {
            s += multipliers[i] * ingredient.calories
        }
        if s < 0 {
            0
        } else {
            s
        }
    }

    fn get_score(&self, multipliers: &[i32]) -> i32 {
        let capacity = self.get_capacity(multipliers);
        let durability = self.get_durability(multipliers);
        let flavor = self.get_flavor(multipliers);
        let texture = self.get_texture(multipliers);
        if capacity == 0 || durability == 0 || flavor == 0 || texture == 0 {
            0
        } else {
            capacity * durability * flavor * texture
        }
    }
}

impl From<&str> for Cookie {
    fn from(input: &str) -> Self {
        Self {
            ingredients: input.lines().map(Ingredient::from).collect::<Vec<_>>(),
        }
    }
}

fn main() {
    let input = include_str!("../src/input.txt");
    let cookie = Cookie::from(input);
    let max_value = (0..101)
        .permutations(4)
        .filter(|comb| comb.iter().sum::<i32>() == 100)
        .map(|combination| cookie.get_score(&combination))
        .max()
        .unwrap();
    println!("{}", max_value);
}
