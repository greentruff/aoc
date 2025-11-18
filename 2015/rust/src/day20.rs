use crate::aoc::Solution;

pub fn solve(input: &str) -> Solution {
    let target: u32 = input.trim().parse().unwrap();

    let part1 = find_first_house(target, 10, u32::MAX);
    let part2 = find_first_house(target, 11, 50);
    Solution::new(20, &part1, &part2)
}

fn find_first_house(target: u32, mult: u32, max_houses: u32) -> u32 {
    let mut counts = vec![0_u32; 1_000_000].into_boxed_slice();

    for i in 0..counts.len() {
        let mut current_idx = i;
        let mut count = 0;
        while current_idx < counts.len() && count < max_houses {
            counts[current_idx] += i as u32 + 1;
            current_idx += i + 1;
            count += 1;
        }
    }

    let (n, _) = counts
        .iter()
        .enumerate()
        .find(|(_, count)| **count * mult >= target)
        .unwrap();

    n as u32 + 1
}
