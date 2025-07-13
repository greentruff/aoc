use crate::aoc::Solution;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn visited_count(input: &str, santa_count: usize) -> usize {
    let mut visited = HashSet::new();

    let mut positions = Vec::with_capacity(santa_count);
    for _ in 0..santa_count {
        let c = Coord { x: 0, y: 0 };
        visited.insert(c.clone());
        positions.push(c);
    }
    let mut current_pos = 0;

    for c in input.chars() {
        let pos = &mut positions[current_pos];
        match c {
            '>' => pos.x += 1,
            '<' => pos.x -= 1,
            '^' => pos.y += 1,
            'v' => pos.y -= 1,
            _ => continue,
        }
        visited.insert(pos.clone());
        current_pos = (current_pos + 1) % santa_count;
    }

    visited.len()
}

pub fn solve(input: impl AsRef<str>) -> Solution {
    let part1 = visited_count(input.as_ref(), 1);
    let part2 = visited_count(input.as_ref(), 2);

    Solution::new(3, &part1, &part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(visited_count(">", 1), 2);
        assert_eq!(visited_count("^>v<", 1), 4);
        assert_eq!(visited_count("^v^v^v^v^v", 1), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(visited_count("^v", 2), 3);
        assert_eq!(visited_count("^>v<", 2), 3);
        assert_eq!(visited_count("^v^v^v^v^v", 2), 11);
    }
}
