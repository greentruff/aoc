use crate::aoc::Solution;
use itertools::Itertools;

// TODO: fast-mode: based on input, other partitions do not need to be proven if a first partition has the correct sum
pub fn solve(input: &str) -> Solution {
    let packages = input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<u32>>();

    let total = packages.iter().sum::<u32>();

    let smallest_groups = find_smallest_group_with_3_partition(&packages, total);
    let part1: u64 = smallest_groups
        .iter()
        .map(|group| group.iter().map(|x| u64::from(*x)).product::<u64>())
        .min()
        .unwrap();

    let smallest_groups = find_smallest_group_with_4_partition(&packages, total);
    let part2 = smallest_groups
        .iter()
        .map(|group| group.iter().map(|x| u64::from(*x)).product::<u64>())
        .min()
        .unwrap();
    Solution::new(24, &part1, &part2)
}

fn find_smallest_group_with_3_partition(packages: &[u32], total: u32) -> Vec<Vec<u32>> {
    let mut smallest_groups = vec![];
    let mut size = 1;
    while smallest_groups.is_empty() {
        smallest_groups = find_groups_of_size_3_partitions(size, packages, total);
        size += 1;
    }
    smallest_groups
}

fn find_groups_of_size_3_partitions(size: usize, packages: &[u32], total: u32) -> Vec<Vec<u32>> {
    let partition_sum = total / 3;
    let mut found = Vec::new();

    for candidate_indices in (0..packages.len()).permutations(size) {
        let is_sorted = candidate_indices.is_sorted();
        let has_partition_sum =
            candidate_indices.iter().map(|i| packages[*i]).sum::<u32>() == partition_sum;
        if is_sorted && has_partition_sum {
            let mut others = Vec::with_capacity(packages.len() - size);
            for (idx, p) in packages.iter().enumerate() {
                if !candidate_indices.contains(&idx) {
                    others.push(*p);
                }
            }
            if can_partition_equally(&others) {
                found.push(candidate_indices.iter().map(|i| packages[*i]).collect());
            }
        }
    }
    found
}

fn find_smallest_group_with_4_partition(packages: &[u32], total: u32) -> Vec<Vec<u32>> {
    let mut smallest_groups = vec![];
    let mut size = 1;
    while smallest_groups.is_empty() {
        smallest_groups = find_groups_of_size_4_partitions(size, packages, total);
        size += 1;
    }
    smallest_groups
}

fn find_groups_of_size_4_partitions(size: usize, packages: &[u32], total: u32) -> Vec<Vec<u32>> {
    let partition_sum = total / 4;
    let mut found = Vec::new();

    for candidate_indices in (0..packages.len()).permutations(size) {
        let is_sorted = candidate_indices.is_sorted();
        let has_partition_sum =
            candidate_indices.iter().map(|i| packages[*i]).sum::<u32>() == partition_sum;
        if is_sorted && has_partition_sum {
            let mut others = Vec::with_capacity(packages.len() - size);
            for (idx, p) in packages.iter().enumerate() {
                if !candidate_indices.contains(&idx) {
                    others.push(*p);
                }
            }
            if !find_smallest_group_with_3_partition(packages, total).is_empty() {
                found.push(candidate_indices.iter().map(|i| packages[*i]).collect());
            }
        }
    }
    found
}

struct MarkerTable(Vec<bool>, usize);
impl MarkerTable {
    fn new(i: usize, j: usize) -> Self {
        Self(vec![false; i * j], j)
    }
    fn get(&self, i: usize, j: usize) -> bool {
        self.0[i * self.1 + j]
    }
    fn set(&mut self, i: usize, j: usize, val: bool) {
        let pos = i * self.1 + j;
        self.0[pos] = val;
    }
}

// https://en.wikipedia.org/wiki/Pseudopolynomial_time_number_partitioning
fn can_partition_equally(numbers: &[u32]) -> bool {
    let n = numbers.len();
    let target = numbers.iter().sum::<u32>() as usize / 2;

    let mut marker_table = MarkerTable::new(target + 1, n + 1);

    for j in 0..=n {
        marker_table.set(0, j, true);
    }
    for i in 1..=target {
        marker_table.set(i, 0, false);
    }

    for sub_sum in 1..=target {
        for (idx, cur_num) in numbers.iter().enumerate() {
            let flag = if sub_sum >= *cur_num as usize {
                marker_table.get(sub_sum, idx) || marker_table.get(sub_sum - *cur_num as usize, idx)
            } else {
                marker_table.get(sub_sum, idx)
            };
            marker_table.set(sub_sum, idx + 1, flag);
        }
    }

    marker_table.get(target, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert!(can_partition_equally(&[3, 1, 1, 2, 2, 1]));

        assert!(can_partition_equally(&[1, 2, 3, 4]));
        assert!(can_partition_equally(&[10, 8, 2, 7, 5, 4, 3, 1]));
        assert!(can_partition_equally(&[11, 7, 2, 8, 5, 4, 3]));
        assert!(can_partition_equally(&[11, 9, 7, 5, 4, 3, 1]));
        assert!(can_partition_equally(&[11, 9, 10, 4, 3, 2, 1]));

        assert!(!can_partition_equally(&[17, 2, 3, 4]));
        assert!(!can_partition_equally(&[110, 7, 2, 8, 5, 4, 3]));
        assert!(!can_partition_equally(&[99, 9, 10, 4, 3, 2, 1]));
    }

    #[test]
    fn test_example() {
        assert_eq!(
            find_groups_of_size_3_partitions(2, &[1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 20),
            vec![vec![9, 11]]
        );
    }
}
