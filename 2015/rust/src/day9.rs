use crate::aoc::Solution;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: impl AsRef<str>) -> Solution {
    let distance_list = input.as_ref().lines().map(parse_line).collect::<Vec<_>>();

    let mut city_ids = HashMap::new();
    let mut distances = HashMap::new();

    for (city1, city2, dx) in distance_list {
        let next_id = city_ids.len();
        let from = *city_ids.entry(city1).or_insert(next_id);
        let next_id = city_ids.len();
        let to = *city_ids.entry(city2).or_insert(next_id);

        distances.insert(order_pair(from, to), dx);
    }

    if let MinMax(part1, part2) = (0..city_ids.len())
        .permutations(city_ids.len())
        .map(|perm| {
            perm.windows(2)
                .map(|w| *distances.get(&order_pair(w[0], w[1])).unwrap())
                .sum::<u32>()
        })
        .minmax()
    {
        return Solution::new(9, &part1, &part2);
    }

    unreachable!()
}

fn order_pair<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}

fn parse_line(line: &str) -> (String, String, u32) {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let caps = re.captures(line).unwrap();

    (caps[1].into(), caps[2].into(), caps[3].parse().unwrap())
}
