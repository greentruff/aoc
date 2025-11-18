use crate::aoc::AocError::HttpError;
use directories::ProjectDirs;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::{env, fs};
use thiserror::Error;

const COOKIE_ENV: &str = "AOC_COOKIE";
const COOKIE_FILE: &str = "cookie.txt";

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Could not determine config folder")]
    NoConfigPath,
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Could not find cookie")]
    NoCookie,
    #[error("Error downloading input: {0}")]
    HttpError(#[from] ureq::Error),
}

pub fn get_day_input(year: u32, day: u32) -> Result<String, AocError> {
    let project_dirs =
        ProjectDirs::from("com", "truff", "aoc").ok_or_else(|| AocError::NoConfigPath)?;
    let config_dir = project_dirs.config_dir();

    let year_path = PathBuf::from(config_dir).join(year.to_string());
    create_dir_all(&year_path)?;

    let input_day_path = PathBuf::from(&year_path).join(format!("day{day}.txt"));

    if !input_day_path.exists() {
        let cookie = get_cookie(config_dir)?;
        let input = fetch_day_input(year, day, cookie.as_str())?;
        fs::write(&input_day_path, &input)?;
        return Ok(input);
    }

    read_to_string(input_day_path.as_path()).map_err(AocError::IOError)
}

fn get_cookie(config_dir: &Path) -> Result<String, AocError> {
    if let Ok(cookie) = env::var(COOKIE_ENV) {
        return Ok(cookie);
    }
    if let Ok(cookie) = fs::read_to_string(config_dir.join(COOKIE_FILE)) {
        return Ok(cookie.trim().to_string());
    }

    Err(AocError::NoCookie)
}

fn fetch_day_input(year: u32, day: u32, cookie: &str) -> Result<String, AocError> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let mut body = ureq::get(&url)
        .header("Cookie", format!("session={cookie}"))
        .call()
        .map_err(HttpError)?;

    Ok(body.body_mut().read_to_string()?)
}

pub struct Solution {
    pub day: u32,
    pub part1: String,
    pub part2: String,
}
impl Solution {
    pub fn new(day: u32, part1: &impl ToString, part2: &impl ToString) -> Self {
        Solution {
            day,
            part1: part1.to_string(),
            part2: part2.to_string(),
        }
    }
}
