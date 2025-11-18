use crate::aoc::Solution;
use std::collections::HashMap;
use std::hash::Hash;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_part1_nice(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();

    let mut vowel_count = 0;
    if VOWELS.contains(&chars[0]) {
        vowel_count += 1;
    }

    let mut has_double = false;
    for idx in 1..chars.len() {
        let cur = chars[idx];
        let prev = chars[idx - 1];

        if VOWELS.contains(&cur) {
            vowel_count += 1;
        }

        if (prev == 'a' && cur == 'b')
            || (prev == 'c' && cur == 'd')
            || (prev == 'p' && cur == 'q')
            || (prev == 'x' && cur == 'y')
        {
            return false;
        }

        if cur == prev {
            has_double = true;
        }
    }

    vowel_count >= 3 && has_double
}

#[derive(Eq, PartialEq, Hash)]
struct Pair {
    first: char,
    second: char,
}

fn is_part2_nice(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    let mut has_double_pair = false;
    let mut has_split_pair = false;

    let mut seen = HashMap::new();
    let initial_pair = Pair {
        first: chars[0],
        second: chars[1],
    };
    seen.insert(initial_pair, 0);
    for idx in 2..chars.len() {
        let a = chars[idx - 2];
        let b = chars[idx - 1];
        let c = chars[idx];

        let pair = Pair {
            first: b,
            second: c,
        };
        let pair_idx = idx - 1;
        let previous_idx = seen.entry(pair).or_insert(idx - 1);
        if (pair_idx - *previous_idx) >= 2 {
            has_double_pair = true;
        }

        if a == c {
            has_split_pair = true;
        }
    }

    has_double_pair && has_split_pair
}

pub fn solve(input: &str) -> Solution {
    let mut part1_count = 0;
    let mut part2_count = 0;
    for line in input.lines() {
        if is_part1_nice(line) {
            part1_count += 1;
        }
        if is_part2_nice(line) {
            part2_count += 1;
        }
    }

    Solution::new(5, &part1_count, &part2_count)
}

#[cfg(test)]
mod tests {
    use super::is_part1_nice;
    use super::is_part2_nice;

    #[test]
    fn test_part1_nice() {
        assert_eq!(is_part1_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_part1_nice("aaa"), true);
    }

    #[test]
    fn test_part1_naughty() {
        assert_eq!(is_part1_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_part1_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_part1_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_part2_nice() {
        assert_eq!(is_part2_nice("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_part2_nice("xxyxx"), true);
    }

    #[test]
    fn test_part2_naughty() {
        assert_eq!(is_part2_nice("uurcxstgmygtbstg"), false);
        assert_eq!(is_part2_nice("ieodomkazucvgmuy"), false);
    }
}
