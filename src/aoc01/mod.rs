use std::io;
use crate::inputreader::get_lines_from;

pub fn fuel_needed_for(mass: u32) -> u32 {
    ((mass as f64 / 3.0).floor() - 2.0) as u32
}

pub fn sum_fuel_requirements_from_input() -> io::Result<u32> {
    Ok(get_lines_from("resources/aoc01.txt")?
        .map(|line| fuel_needed_for(line.unwrap().parse::<u32>().unwrap()))
        .sum())
}

#[cfg(test)]
mod aoc01_test;
