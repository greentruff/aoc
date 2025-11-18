use crate::aoc::{AocError, get_day_input};
use std::env::args;

mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn solve_for_day(day: u32) -> Result<(), AocError> {
    let input = get_day_input(2015, day)?;
    match day {
        1 => day1::solve(&input),
        2 => day2::solve(&input),
        3 => day3::solve(&input),
        4 => day4::solve(&input),
        5 => day5::solve(&input),
        _ => unimplemented!(),
    }
    Ok(())
}

fn main() -> Result<(), AocError> {
    let args = args().skip(1).collect::<Vec<_>>();
    if !args.is_empty() {
        let day = args[0].parse::<u32>().unwrap();
        solve_for_day(day)?;
    } else {
        for day in 1..=5 {
            println!("==== Day {day:>2} ====");
            solve_for_day(day)?;
        }
    }

    Ok(())
}
