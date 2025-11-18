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

const SOLVERS: [fn(&str) -> Solution; 9] = [
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
    day8::solve,
    day9::solve,
];

fn solve_for_day(day: usize) -> Result<Solution, AocError> {
    let input = get_day_input(2015, day)?;

    if day > SOLVERS.len() {
        unimplemented!()
    }

    Ok(SOLVERS[day - 1](&input))
}

fn print_solution(solution: Solution) {
    println!("==== Day {:>2} ====", solution.day);
    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
}

fn main() -> Result<(), AocError> {
    let args = args().skip(1).collect::<Vec<_>>();
    if !args.is_empty() {
        let day = args[0].parse::<usize>().unwrap();
        let solution = solve_for_day(day)?;
        print_solution(solution);
    } else {
        for day in 1..=SOLVERS.len() {
            let solution = solve_for_day(day)?;
            print_solution(solution);
        }
    }

    Ok(())
}
