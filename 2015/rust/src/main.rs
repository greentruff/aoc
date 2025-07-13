use crate::aoc::{AocError, Solution, get_day_input};
use std::env::args;

mod aoc;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

const SOLVERS: [fn(&str) -> Solution; 10] = [
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
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
