use crate::aoc::Solution;

pub fn solve(input: impl AsRef<str>) -> Solution {
    let part1: i32 = input
        .as_ref()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();

    let mut running_sum: i32 = 0;
    let part2: usize;
    for (index, c) in input.as_ref().chars().enumerate() {
        running_sum += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if running_sum < 0 {
            let pos = index + 1;
            part2 = pos;
            return Solution::new(1, &part1, &part2);
        }
    }

    unreachable!()
}
