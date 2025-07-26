use crate::aoc::{get_day_input, AocError, Solution};
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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod utils;

const SOLVERS: [fn(&str) -> Solution; 22] = [
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
    day11::solve,
    day12::solve,
    day13::solve,
    day14::solve,
    day15::solve,
    day16::solve,
    day17::solve,
    day18::solve,
    day19::solve,
    day20::solve,
    day21::solve,
    day22::solve,
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
