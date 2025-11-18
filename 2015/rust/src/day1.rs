pub fn solve(input: &String) {
    let part1: i32 = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();
    println!("Part 1: {part1}");

    let mut running_sum: i32 = 0;
    for (index, c) in input.chars().enumerate() {
        running_sum += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if running_sum < 0 {
            let pos = index + 1;
            println!("Part 2: {pos}");
            break;
        }
    }
}
