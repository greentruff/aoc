use crate::common::read_day_input;
use std::env::args;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn solve_for_day(day: i32) {
    let input = read_day_input(day).unwrap();
    match day {
        1 => day1::solve(&input),
        2 => day2::solve(&input),
        3 => day3::solve(&input),
        4 => day4::solve(&input),
        5 => day5::solve(&input),
        _ => unimplemented!(),
    }
}

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    if args.len() > 0 {
        let day = args[0].parse::<i32>().unwrap();
        solve_for_day(day);
    } else {
        for day in 1..=5 {
            println!("==== Day {day:>2} ====");
            solve_for_day(day);
        }
    }
}
