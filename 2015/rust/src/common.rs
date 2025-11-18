use std::{fs::read_to_string, io};

pub fn read_day_input(day: i32) -> io::Result<String> {
    read_to_string(format!("input/day{day}.txt"))
}
