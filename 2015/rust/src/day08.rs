use crate::aoc::Solution;

pub fn solve(input: &str) -> Solution {
    let repr_count: usize = input.lines().map(len_repr).sum();
    let parsed_count: usize = input.lines().map(len_parsed).sum();
    let escaped_count: usize = input.lines().map(len_escaped).sum();

    let part1 = repr_count - parsed_count;
    let part2 = escaped_count - repr_count;

    Solution::new(8, &part1, &part2)
}

fn len_repr(input: &str) -> usize {
    input.len()
}
fn len_parsed(input: &str) -> usize {
    let mut count = 0;
    let mut idx = 1;
    while idx < input.len() - 1 {
        count += 1;
        if input.chars().nth(idx).unwrap() == '\\' {
            match input.chars().nth(idx + 1).unwrap() {
                '\\' => idx += 1,
                '"' => idx += 1,
                'x' => idx += 3,
                _ => {
                    dbg!(input);
                    unimplemented!()
                }
            }
        }
        idx += 1;
    }
    count
}

fn len_escaped(input: &str) -> usize {
    let mut count = 2;
    for c in input.chars() {
        match c {
            '\\' => count += 2,
            '"' => count += 2,
            _ => count += 1,
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_parsed() {
        assert_eq!(len_parsed(r#""""#), 0);
        assert_eq!(len_parsed(r#""abc""#), 3);
        assert_eq!(len_parsed(r#""aaa\"aaa""#), 7);
        assert_eq!(len_parsed(r#""\x27""#), 1);
    }
}
