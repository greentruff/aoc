use crate::aoc::Solution;
use crate::day15::PartitionIterState::{Done, Generating, Start};

pub fn solve(input: &str) -> Solution {
    let ingredients = input.lines().map(parse_line).collect::<Vec<_>>();

    let part1 = int_partitions(100, ingredients.len())
        .map(|counts| ingredients_score(&ingredients, &counts))
        .max()
        .unwrap();
    let part2 = int_partitions(100, ingredients.len())
        .filter(|counts| {
            let calories = ingredients
                .iter()
                .enumerate()
                .map(|(idx, ingredient)| ingredient.calories * counts[idx])
                .sum::<i32>();
            calories == 500
        })
        .map(|counts| ingredients_score(&ingredients, &counts))
        .max()
        .unwrap();
    Solution::new(15, &part1, &part2)
}

#[derive(Eq, PartialEq, Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn ingredients_score(ingredients: &[Ingredient], counts: &[i32]) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for idx in 0..ingredients.len() {
        let ingredient = &ingredients[idx];
        let count = counts[idx];

        capacity += ingredient.capacity * count;
        durability += ingredient.durability * count;
        flavor += ingredient.flavor * count;
        texture += ingredient.texture * count;
    }

    i32::max(0, capacity) * i32::max(0, durability) * i32::max(0, flavor) * i32::max(0, texture)
}

fn parse_line(line: &str) -> Ingredient {
    let parts = line[line.find(": ").unwrap() + 1..]
        .trim()
        .split(", ")
        .map(|x| x.split_whitespace().last().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    Ingredient {
        capacity: parts[0],
        durability: parts[1],
        flavor: parts[2],
        texture: parts[3],
        calories: parts[4],
    }
}

#[derive(PartialEq)]
enum PartitionIterState {
    Start,
    Generating,
    Done,
}
struct PartitionIter {
    total: i32,
    counts: Vec<i32>,
    running_sum: Vec<i32>,
    state: PartitionIterState,
}

fn int_partitions(sum: i32, parts: usize) -> PartitionIter {
    assert!(
        parts >= 2 || parts <= sum as usize,
        "Invalid number of parts ({parts}) passed"
    );
    PartitionIter {
        total: sum,
        counts: vec![0; parts],
        running_sum: Vec::with_capacity(parts),
        state: Start,
    }
}

impl Iterator for PartitionIter {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            Start => {
                self.state = Generating;
                let len = self.counts.len();
                self.counts[len - 1] = self.total;
                Some(self.counts.clone())
            }
            Generating => {
                let len = self.counts.len();
                let mut sum = 0;
                let mut idx = 0;

                self.running_sum.clear();
                for count in &self.counts {
                    sum += *count;
                    self.running_sum.push(sum);
                    if sum == self.total {
                        break;
                    }
                    idx += 1;
                }

                let inc_idx = idx - 1;
                self.counts[inc_idx] += 1;
                for i in idx..len - 1 {
                    self.counts[i] = 0;
                }
                self.counts[len - 1] = self.total - (self.running_sum[inc_idx] + 1);

                if self.counts[0] == self.total {
                    self.state = Done;
                }

                Some(self.counts.clone())
            }
            Done => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_parse() {
        let actual = SAMPLE.lines().map(parse_line).collect::<Vec<_>>();
        let expected = vec![
            Ingredient {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            },
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_partitions() {
        assert_eq!(int_partitions(6, 4).count(), 84);
        assert_eq!(int_partitions(100, 4).count(), 176_851);
    }
}
