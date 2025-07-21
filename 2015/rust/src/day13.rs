use crate::aoc::Solution;
use crate::utils::StringToId;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution {
    let happiness_list = input.lines().map(parse_line).collect::<Vec<_>>();

    let mut guest_ids = StringToId::new();
    let mut happiness = HashMap::new();

    for (guest, neighbour, h) in &happiness_list {
        let guest_id = guest_ids.get(guest).unwrap();
        let neighbour_id = guest_ids.get(neighbour).unwrap();
        happiness.insert((guest_id, neighbour_id), *h);
    }

    let part1 = (0..guest_ids.len())
        .permutations(guest_ids.len())
        .map(|perm| {
            let mut ring = perm.to_vec();
            ring.push(perm[0]);

            ring.windows(2)
                .map(|w| {
                    happiness.get(&(w[0], w[1])).unwrap() + happiness.get(&(w[1], w[0])).unwrap()
                })
                .sum::<i32>()
        })
        .max()
        .unwrap();

    let part2 = (0..guest_ids.len())
        .permutations(guest_ids.len())
        .map(|perm| {
            perm.windows(2)
                .map(|w| {
                    happiness.get(&(w[0], w[1])).unwrap() + happiness.get(&(w[1], w[0])).unwrap()
                })
                .sum::<i32>()
        })
        .max()
        .unwrap();

    Solution::new(13, &part1, &part2)
}

fn parse_line(line: &str) -> (String, String, i32) {
    // Alice would gain 54 happiness units by sitting next to Bob.
    let split = line.split_whitespace().collect::<Vec<_>>();
    let guest = split[0];
    let neighbour = split[10].trim_end_matches('.');

    let gain = split[2];
    let happiness = split[3].parse::<i32>().unwrap();
    let happiness = if gain == "gain" {
        happiness
    } else {
        -happiness
    };

    (guest.to_string(), neighbour.to_string(), happiness)
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn part_1_test() {
        assert_eq!(solve(SAMPLE).part1, "330");
    }
}
