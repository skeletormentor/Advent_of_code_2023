use rand::prelude::*;

#[derive(Debug, PartialEq)]
enum Type {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug)]
struct Item {
    o_type: Type,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new(o_type: Type, cost: u32, damage: u32, armor: u32) -> Self {
        Self {
            o_type,
            cost,
            damage,
            armor,
        }
    }
}

#[derive(Debug)]
struct Player {
    items: Vec<Item>,
    total_cost: u32,
    total_damage: u32,
    total_armor: u32,
}

impl Player {
    fn new_rand(items_to_choose: &Vec<Item>) -> Self {
        let mut rng = thread_rng();
        let mut items = vec![];
        let weapon = items_to_choose
            .iter()
            .filter(|item| item.o_type == Type::Weapon)
            .choose(&mut rng)
            .unwrap();
        let armor = items_to_choose
            .iter()
            .filter(|item| item.o_type == Type::Armor)
            .choose(&mut rng)
            .unwrap();
        items.push(*weapon);
        items.push(*armor);
        let amount_of_rings = thread_rng().gen_range(0..=2);
        let mut rings = items_to_choose
            .into_iter()
            .filter(|item| item.o_type == Type::Ring)
            .choose_multiple(&mut rng, amount_of_rings);
        items.extend(rings);
        Self {
            items,
            total_cost: 0,
            total_damage: 0,
            total_armor: 0,
        }
    }
}

fn main() {
    let items = vec![
        Item::new(Type::Weapon, 8, 4, 0),
        Item::new(Type::Weapon, 10, 5, 0),
        Item::new(Type::Weapon, 25, 6, 0),
        Item::new(Type::Weapon, 40, 7, 0),
        Item::new(Type::Weapon, 74, 8, 0),
        Item::new(Type::Armor, 0, 0, 0),
        Item::new(Type::Armor, 13, 0, 1),
        Item::new(Type::Armor, 31, 0, 2),
        Item::new(Type::Armor, 53, 0, 3),
        Item::new(Type::Armor, 75, 0, 4),
        Item::new(Type::Armor, 102, 0, 5),
        Item::new(Type::Ring, 25, 1, 0),
        Item::new(Type::Ring, 50, 2, 0),
        Item::new(Type::Ring, 100, 3, 0),
        Item::new(Type::Ring, 20, 0, 1),
        Item::new(Type::Ring, 40, 0, 2),
        Item::new(Type::Ring, 80, 0, 3),
    ];
    let mut rng = thread_rng();
    println!("{:?}", Player::new_rand(&items));
}
