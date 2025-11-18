use crate::aoc::Solution;

pub fn solve(input: &str) -> Solution {
    let (row, column) = parse(input);
    let part1 = value_at_pos(row, column);
    let part2 = "free";
    Solution::new(25, &part1, &part2)
}

fn parse(input: &str) -> (usize, usize) {
    // To continue, please consult the code grid in the manual.  Enter the code at row 2947, column 3029.
    let parts: Vec<&str> = input.split_whitespace().collect();
    let (row, column) = (
        parts[15].trim_end_matches(',').parse().unwrap(),
        parts[17].trim_end_matches('.').parse().unwrap(),
    );
    (row, column)
}

fn value_at_pos(target_row: usize, target_column: usize) -> usize {
    let mut value = 20151125;

    let mut row = 1;
    let mut column = 1;

    while !(row == target_row && column == target_column) {
        if row == 1 {
            row = column + 1;
            column = 1;
        } else {
            row -= 1;
            column += 1;
        }
        value = (value * 252533) % 33554393;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(value_at_pos(4, 4), 9380097);
        assert_eq!(value_at_pos(6, 6), 27995004);
    }
}
