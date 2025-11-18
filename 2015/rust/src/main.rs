use crate::common::read_day_input;

mod common;
mod day1;
mod day2;
mod day3;

fn solve_for_day(day: i32) {
    let input = read_day_input(day).unwrap();
    match day {
        1 => day1::solve(&input),
        2 => day2::solve(&input),
        3 => day3::solve(&input),
        _ => unimplemented!(),
    }
}

fn main() {
    for day in 1..=3 {
        println!("==== Day {day:>2} ====");
        solve_for_day(day);
    }
}
