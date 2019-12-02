use super::*;

#[test]
fn mass_of_12_should_be_2() {
    assert_eq!(2, fuel_needed_for(12))
}

#[test]
fn mass_of_14_should_be_2() {
    assert_eq!(2, fuel_needed_for(14))
}

#[test]
fn mass_of_1969_should_be_654() {
    assert_eq!(654, fuel_needed_for(1969))
}

#[test]
fn mass_of_100756_should_need_33583() {
    assert_eq!(33583, fuel_needed_for(100756))
}

#[test]
fn mass_of_14_should_need_2_recursive_fuel() {
    assert_eq!(2, recursive_fuel_needed_for(14))
}

#[test]
fn mass_of_1969_should_need_966_recursive_fuel() {
    assert_eq!(966, recursive_fuel_needed_for(1969))
}

#[test]
fn mass_of_100756_should_need_50346_recursive_fuel() {
    assert_eq!(50346, recursive_fuel_needed_for(100756))
}
