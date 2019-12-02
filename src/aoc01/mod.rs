use std::io;
use crate::inputreader::get_lines_from;

pub fn fuel_needed_for(mass: i32) -> i32 {
    ((mass as f64 / 3.0).floor() - 2.0) as i32
}

pub fn recursive_fuel_needed_for(mut mass: i32) -> i32 {
    let mut sum = 0i32;
    loop {
        mass = fuel_needed_for(mass);
        if mass > 0 {
            sum += mass;
        } else {
            break;
        }
    }
    sum
}

pub fn part_1() -> io::Result<i32> {
    Ok(get_lines_from("resources/aoc01.txt")?
        .map(|line| fuel_needed_for(line.unwrap().parse::<i32>().unwrap()))
        .sum())
}

pub fn part_2() -> io::Result<i32> {
    Ok(get_lines_from("resources/aoc01.txt")?
        .map(|line| recursive_fuel_needed_for(line.unwrap().parse::<i32>().unwrap()))
        .sum())
}

#[cfg(test)]
mod aoc01_test;
