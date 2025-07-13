use crate::aoc::Solution;

pub fn solve(input: &str) -> Solution {
    let chars = input.trim().as_bytes();

    let mut applied = Vec::from(chars);
    for _ in 0..40 {
        applied = look_and_say(&applied);
    }
    let part1 = applied.len();

    for _ in 0..10 {
        applied = look_and_say(&applied);
    }
    let part2 = applied.len();

    Solution::new(10, &part1, &part2)
}

pub fn look_and_say(input: &[u8]) -> Vec<u8> {
    let mut idx = 0;
    let mut output = Vec::new();

    while idx < input.len() {
        let ch = input[idx];
        let start = idx;
        while idx < input.len() && input[idx] == ch {
            idx += 1;
        }
        output.push(b'0' + (idx - start) as u8);
        output.push(ch);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn look_and_say_str(input: &str) -> String {
        unsafe { String::from_utf8_unchecked(look_and_say(input.as_bytes())) }
    }

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say_str("1"), "11");
        assert_eq!(look_and_say_str("11"), "21");
        assert_eq!(look_and_say_str("21"), "1211");
        assert_eq!(look_and_say_str("111221"), "312211");
    }
}
