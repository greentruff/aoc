use crate::aoc::Solution;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution {
    let known = HashMap::from([
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

    let part1 = input
        .lines()
        .map(parse_line)
        .find(|s| {
            s.hints
                .iter()
                .all(|(hint_key, hint_value)| known.get(hint_key.as_str()) == Some(hint_value))
        })
        .unwrap()
        .id;

    let part2 = input
        .lines()
        .map(parse_line)
        .find(|s| {
            s.hints
                .iter()
                .all(|(hint_key, hint_value)| match hint_key.as_str() {
                    "cats" | "trees" => Some(hint_value) > known.get(hint_key.as_str()),
                    "pomeranians" | "goldfish" => Some(hint_value) < known.get(hint_key.as_str()),
                    _ => Some(hint_value) == known.get(hint_key.as_str()),
                })
        })
        .unwrap()
        .id;

    Solution::new(16, &part1, &part2)
}

struct Sue {
    id: u32,
    hints: HashMap<String, u32>,
}

fn parse_line(line: &str) -> Sue {
    // Sue 1: goldfish: 9, cars: 0, samoyeds: 9
    let parts = line.splitn(2, ": ").collect::<Vec<_>>();
    let id = parts[0].split_whitespace().last().unwrap().parse().unwrap();

    let hints = parts[1]
        .split(", ")
        .map(|h| {
            let mut hint = h.split(": ");
            let key = hint.next().unwrap().to_string();
            let value = hint.next().unwrap().parse::<u32>().unwrap();
            (key, value)
        })
        .collect::<HashMap<String, u32>>();

    Sue { id, hints }
}
