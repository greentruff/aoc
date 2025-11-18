use crate::aoc::Solution;
use std::ptr;

pub fn solve(input: &str) -> Solution {
    let boss = parse(input.trim());

    let item_combinations = player_item_combinations();
    let part1 = part1(&item_combinations, &boss);
    let part2 = part2(&item_combinations, &boss);
    Solution::new(21, &part1, &part2)
}

fn player_item_combinations() -> Vec<Item> {
    let mut combinations = Vec::new();

    for weapon in &WEAPONS {
        combinations.push(weapon.clone());
        for armor in &ARMOR {
            combinations.push(Item::from(&[weapon, armor]));
            for ring1 in &RINGS {
                combinations.push(Item::from(&[weapon, armor, ring1]));
                for ring2 in &RINGS {
                    if ptr::eq(ring1, ring2) {
                        continue;
                    }
                    combinations.push(Item::from(&[weapon, armor, ring1, ring2]));
                }
            }
        }
    }

    combinations
}

fn part1(item_combinations: &[Item], boss: &Char) -> u32 {
    let item_set = item_combinations
        .iter()
        .filter(|item| {
            let player = Char {
                hitpoints: 100,
                damage: item.damage,
                armor: item.armor,
            };
            first_wins(&player, boss)
        })
        .min_by(|item1, item2| item1.cost.cmp(&item2.cost))
        .unwrap();

    item_set.cost
}

fn part2(item_combinations: &[Item], boss: &Char) -> u32 {
    let item_set = item_combinations
        .iter()
        .filter(|item| {
            let player = Char {
                hitpoints: 100,
                damage: item.damage,
                armor: item.armor,
            };
            !first_wins(&player, boss)
        })
        .max_by(|a, b| a.cost.cmp(&b.cost))
        .unwrap();

    item_set.cost
}

struct Char {
    hitpoints: u32,
    damage: u32,
    armor: u32,
}

const WEAPONS: [Item; 5] = [
    Item { cost: 8, damage: 4, armor: 0 },
    Item { cost: 10, damage: 5, armor: 0 },
    Item { cost: 25, damage: 6, armor: 0 },
    Item { cost: 40, damage: 7, armor: 0 },
    Item { cost: 74, damage: 8, armor: 0 },
];
const ARMOR: [Item; 6] = [
    Item { cost: 0, damage: 0, armor: 0 },
    Item { cost: 13, damage: 0, armor: 1 },
    Item { cost: 31, damage: 0, armor: 2 },
    Item { cost: 53, damage: 0, armor: 3 },
    Item { cost: 75, damage: 0, armor: 4 },
    Item { cost: 102, damage: 0, armor: 5 },
];
const RINGS: [Item; 6] = [
    Item { cost: 25, damage: 1, armor: 0 },
    Item { cost: 50, damage: 2, armor: 0 },
    Item { cost: 100, damage: 3, armor: 0 },
    Item { cost: 20, damage: 0, armor: 1 },
    Item { cost: 40, damage: 0, armor: 2 },
    Item { cost: 80, damage: 0, armor: 3 },
];

#[derive(Debug, Clone)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn from(items: &[&Item]) -> Self {
        let mut cost = 0;
        let mut damage = 0;
        let mut armor = 0;
        for item in items {
            cost += item.cost;
            damage += item.damage;
            armor += item.armor;
        }
        Self { cost, damage, armor }
    }
}

fn parse(input: &str) -> Char {
    let mut char = Char { hitpoints: 0, damage: 0, armor: 0 };

    for line in input.lines() {
        let mut split = line.split(": ");
        if let (Some(description), Some(n)) = (split.next(), split.next()) {
            match description {
                "Hit Points" => char.hitpoints = n.parse().unwrap(),
                "Damage" => char.damage = n.parse().unwrap(),
                "Armor" => char.armor = n.parse().unwrap(),
                _ => unimplemented!(),
            }
        }
    }
    char
}

fn first_wins(char1: &Char, char2: &Char) -> bool {
    fn dmg(damage: u32, armor: u32) -> u32 {
        if damage > armor { damage - armor } else { 1 }
    }
    let char1_damage = dmg(char1.damage, char2.armor);
    let char2_damage = dmg(char2.damage, char1.armor);

    let char1_turns = u32::div_ceil(char1.hitpoints, char2_damage);
    let char2_turns = u32::div_ceil(char2.hitpoints, char1_damage);

    char1_turns >= char2_turns
}
