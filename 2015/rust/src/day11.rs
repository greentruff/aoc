use crate::aoc::Solution;

pub fn solve(input: &str) -> Solution {
    let part1 = next_passwd(input.trim());
    let part2 = next_passwd(part1.as_str());
    Solution::new(11, &part1, &part2)
}

fn next_passwd(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    increment_pwd(&mut chars);

    while !is_valid_pwd(chars.as_slice()) {
        increment_pwd(&mut chars);
    }

    chars.into_iter().collect::<String>()
}

fn increment_pwd(input: &mut [char]) {
    let mut idx = input.len() - 1;
    while input[idx] == 'z' {
        input[idx] = 'a';
        idx -= 1;
    }

    input[idx] = char::from_u32((input[idx] as u32) + 1).unwrap();
}

const BANNED_LETTERS: [char; 3] = ['i', 'l', 'o'];
fn is_valid_pwd(input: &[char]) -> bool {
    let mut found_straight = false;
    let mut pairs = Vec::new();

    if BANNED_LETTERS.contains(&input[0]) || BANNED_LETTERS.contains(&input[1]) {
        return false;
    }
    if input[0] == input[1] {
        pairs.push(0);
    }

    for idx in 2..input.len() {
        let a = input[idx - 2];
        let b = input[idx - 1];
        let c = input[idx];

        if BANNED_LETTERS.contains(&c) {
            return false;
        }

        let pair_idx = idx - 1;
        if b == c && !pairs.contains(&(pair_idx - 1)) {
            pairs.push(pair_idx);
        }

        if (a as u32 + 1 == b as u32) && (b as u32 + 1 == c as u32) {
            found_straight = true;
        }
    }

    pairs.len() >= 2 && found_straight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_pwd() {
        fn is_valid_str(s: &str) -> bool {
            is_valid_pwd(s.chars().collect::<Vec<_>>().as_slice())
        }
        assert_eq!(is_valid_str("hijklmmn"), false);
        assert_eq!(is_valid_str("abbceffg"), false);
        assert_eq!(is_valid_str("abbcegjk"), false);
        assert_eq!(is_valid_str("abcdffaa"), true);
        assert_eq!(is_valid_str("ghjaabcc"), true);
    }

    #[test]
    fn test_increment_pwd() {
        fn increment_pwd_str(s: &str) -> String {
            let mut s = s.chars().collect::<Vec<_>>();
            increment_pwd(&mut s);
            s.into_iter().collect()
        }
        assert_eq!(increment_pwd_str("abcdef"), "abcdeg");
        assert_eq!(increment_pwd_str("abcdez"), "abcdfa");
        assert_eq!(increment_pwd_str("abzzzz"), "acaaaa");
    }

    #[test]
    fn test_next_passwd() {
        assert_eq!(next_passwd("abcdefgh"), "abcdffaa");
        assert_eq!(next_passwd("ghijklmn"), "ghjaabcc");
    }
}
