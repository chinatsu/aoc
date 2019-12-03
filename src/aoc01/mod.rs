pub fn fuel_for(mass: i32) -> i32 {
    ((mass as f64 / 3.0).floor() - 2.0) as i32
}

pub fn recursive_fuel_for(mass: i32) -> i32 {
    fn iter(mass: i32, sum: i32) -> i32 {
        let fuel = fuel_for(mass);
        if fuel <= 0 { sum }
        else { iter(fuel, sum + fuel) }
    }
    iter(mass, 0)
}

pub fn part_1() -> i32 {
    lines_from!("aoc01").map(|line| fuel_for(line.unwrap().parse::<i32>().unwrap())).sum()
}

pub fn part_2() -> i32 {
    lines_from!("aoc01").map(|line| recursive_fuel_for(line.unwrap().parse::<i32>().unwrap())).sum()
}

#[cfg(test)]
mod aoc01_test;
