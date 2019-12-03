pub fn csv_to_numbers() -> Vec<u128> {
    csv_from!("aoc02")
        .map(|val| val.parse::<u128>().unwrap_or(99))
        .collect()
}


pub fn run(program: &mut Vec<u128>) -> Vec<u128> {
    program.to_vec()
}

#[cfg(test)]
mod aoc02_test;
