use crate::aoc::{AocError, Solution, get_day_input};
use std::env::args;

mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn solve_for_day(day: u32) -> Result<Solution, AocError> {
    let input = get_day_input(2015, day)?;
    let solution = match day {
        1 => day1::solve(&input),
        2 => day2::solve(&input),
        3 => day3::solve(&input),
        4 => day4::solve(&input),
        5 => day5::solve(&input),
        6 => day6::solve(&input),
        7 => day7::solve(&input),
        8 => day8::solve(&input),
        9 => day9::solve(&input),
        _ => unimplemented!(),
    };
    Ok(solution)
}

fn print_solution(solution: Solution) {
    println!("==== Day {:>2} ====", solution.day);
    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
}

fn main() -> Result<(), AocError> {
    let args = args().skip(1).collect::<Vec<_>>();
    if !args.is_empty() {
        let day = args[0].parse::<u32>().unwrap();
        let solution = solve_for_day(day)?;
        print_solution(solution);
    } else {
        for day in 1..=9 {
            let solution = solve_for_day(day)?;
            print_solution(solution);
        }
    }

    Ok(())
}
