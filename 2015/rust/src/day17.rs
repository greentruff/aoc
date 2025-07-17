use std::collections::HashMap;

use crate::aoc::Solution;

const TOTAL: u32 = 150;

pub fn solve(input: &str) -> Solution {
    let containers: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut bitmap = Bitmap::new(containers.len());
    let mut combinations_by_count = HashMap::<u32, u32>::new();

    while bitmap.next() {
        let mut count = 0;
        let mut sum = 0;
        for (idx, size) in containers.iter().enumerate() {
            if bitmap.get(idx).unwrap_or(false) {
                count += 1;
                sum += *size;
            }
        }

        if sum == TOTAL {
            let count_for_size = combinations_by_count.entry(count).or_insert(0);
            *count_for_size += 1;
        }
    }

    let part1: u32 = combinations_by_count.values().sum();
    let part2 = combinations_by_count
        .get(combinations_by_count.keys().min().unwrap())
        .unwrap();

    Solution::new(17, &part1, &part2)
}

struct Bitmap {
    data: u32,
    count: usize,
}

impl Bitmap {
    fn new(count: usize) -> Self {
        Bitmap { data: 0, count }
    }

    fn get(&self, key: usize) -> Option<bool> {
        if key < self.count {
            Some((1 << key & self.data) != 0)
        } else {
            None
        }
    }

    fn next(&mut self) -> bool {
        if self.data >= 1 << self.count {
            false
        } else {
            self.data += 1;
            true
        }
    }
}
