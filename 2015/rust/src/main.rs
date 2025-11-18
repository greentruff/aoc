use crate::common::read_day_input;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

fn solve_for_day(day: i32) {
    let input = read_day_input(day).unwrap();
    match day {
        1 => day1::solve(&input),
        2 => day2::solve(&input),
        3 => day3::solve(&input),
        4 => day4::solve(&input),
        _ => unimplemented!(),
    }
}

fn main() {
    for day in 1..=4 {
        println!("==== Day {day:>2} ====");
        solve_for_day(day);
    }
}
