use std::collections::HashMap;

use crate::aoc::Solution;

pub fn solve(input: &str) -> Solution {
    let (replacements, molecule) = parse(input);
    // let modified = HashSet::new();

    for (key, targets) in replacements.iter() {
        for target in targets {}
    }

    let part1 = "";
    let part2 = "";
    Solution::new(19, &part1, &part2)
}

fn parse(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let mut replacements = HashMap::<String, Vec<String>>::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let split = line.splitn(2, " => ").collect::<Vec<_>>();
        if split.len() < 2 {
            break;
        }
        replacements
            .entry(split[0].into())
            .or_default()
            .push(split[1].into());
    }

    let molecule = lines.next().unwrap().into();

    (replacements, molecule)
}
