use crate::aoc::Solution;
use serde_json::{Value, json};

pub fn solve(input: &str) -> Solution {
    let part1 = count_numbers(input, false);
    let part2 = count_numbers(input, true);

    Solution::new(12, &part1, &part2)
}

fn count_numbers(input: &str, ignore_red: bool) -> i64 {
    let data: Value = serde_json::from_str(input).unwrap();
    let mut sum = 0;
    let mut pending = vec![data];
    while let Some(current) = pending.pop() {
        if let Value::Number(n) = &current {
            sum += n.as_i64().unwrap();
        }

        let values = if let Value::Array(array) = &current {
            array.clone()
        } else if let Value::Object(obj) = &current {
            if ignore_red && obj.values().any(|v| v == &json!("red")) {
                vec![]
            } else {
                obj.values().cloned().collect::<Vec<Value>>()
            }
        } else {
            vec![]
        };

        for value in values {
            match value {
                Value::Number(n) => {
                    sum += n.as_i64().unwrap();
                }
                Value::Array(_) | Value::Object(_) => {
                    pending.push(value.clone());
                }
                _ => {}
            }
        }
    }
    sum
}
