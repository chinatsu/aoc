pub fn csv_to_numbers() -> Vec<u8> {
    csv_from!("aoc02")
        .map(|val| val.parse::<u8>().unwrap_or(99))
        .collect()
}

#[cfg(test)]
mod aoc02_test;
