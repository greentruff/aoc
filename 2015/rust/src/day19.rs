use crate::aoc::Solution;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Solution {
    let (mapping, molecule) = parse(input);

    let mutations = single_mutations(&molecule, &mapping);
    let part1 = mutations.len();

    let part2 = count(&molecule);
    Solution::new(19, &part1, &part2)
}

fn parse(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let mut replacements = HashMap::<String, Vec<String>>::new();

    let mut lines = input.lines();

    for line in lines.by_ref() {
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

fn split_atoms(input: &str) -> Vec<&str> {
    let mut output = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        let mut chars = remaining.chars();
        let (a, b) = (chars.next(), chars.next());

        match (a, b) {
            (Some(_first), Some(second)) => {
                if second.is_lowercase() {
                    output.push(&remaining[0..2]);
                    remaining = &remaining[2..];
                } else {
                    output.push(&remaining[0..1]);
                    remaining = &remaining[1..];
                }
            }
            (Some(_), None) => {
                output.push(&remaining[0..1]);
                remaining = &remaining[1..];
            }
            _ => break,
        }
    }

    output
}

fn single_mutations(
    molecule: &str,
    replacements: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    let mut mutations = HashSet::new();
    for (key, targets) in replacements {
        for target in targets {
            molecule.match_indices(key).for_each(|(idx, _)| {
                let left = &molecule[0..idx];
                let right = &molecule[idx + key.len()..];

                mutations.insert(format!("{left}{target}{right}"));
            });
        }
    }

    mutations
}

const OPEN: &str = "Rn";
const SEPARATOR: &str = "Y";
const CLOSE: &str = "Ar";
/**
Relies on replacements having one of two forms:
- `A` => `BC` : Always one -> two atoms
- `A` => `BRnCYDYEAr`: Single atom followed by `Rn`, one ot more atoms separated by Y, closing with Ar. `_Rn_Y_Ar` could be represented as  `_(_;_)`
*/
fn count(input: &str) -> i32 {
    let mut count = 0;

    for atom in split_atoms(input) {
        count += match atom {
            OPEN => 0,       // 1 counted on next atom
            CLOSE => 0,      // already counted on open
            SEPARATOR => -1, // cancel out next atom
            _ => 1,
        };
    }

    count - 1 // initial state has one atom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split_atoms(""), Vec::<String>::new());
        assert_eq!(
            split_atoms("CRnFYMgAr"),
            vec!["C", "Rn", "F", "Y", "Mg", "Ar"]
        );
    }

    #[test]
    fn test_count() {
        assert_eq!(count("AB"), 1);
        assert_eq!(count("ABCD"), 3);
        assert_eq!(count("ARnBAr"), 1);
        assert_eq!(count("ARnBCAr"), 2);
        assert_eq!(count("ARnBYCAr"), 1);
        assert_eq!(count("ABCRnABCYABCAr"), 7);
        assert_eq!(count("CRnMgYSiRnFYFArFAr"), 3);
    }
}
