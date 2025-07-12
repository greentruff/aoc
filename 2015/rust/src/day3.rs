use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn visited_count(input: &String, santa_count: usize) -> usize {
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

pub fn solve(input: &String) {
    let count = visited_count(input, 1);
    println!("Part 1: {count}");

    let count = visited_count(input, 2);
    println!("Part 2: {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(visited_count(&">".into(), 1), 2);
        assert_eq!(visited_count(&"^>v<".into(), 1), 4);
        assert_eq!(visited_count(&"^v^v^v^v^v".into(), 1), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(visited_count(&"^v".into(), 2), 3);
        assert_eq!(visited_count(&"^>v<".into(), 2), 3);
        assert_eq!(visited_count(&"^v^v^v^v^v".into(), 2), 11);
    }
}
