use super::*;

#[test]
fn first_value_in_csv_should_be_1() {
    assert_eq!(&1, csv_to_numbers().first().unwrap())
}

#[test]
fn state_1_0_0_0_99_should_compile_to_2_0_0_0_99() {
    assert_eq!(vec![2, 0, 0, 0, 99], run(&mut vec![1, 0, 0, 0, 99], 0, 0))
}

#[test]
fn state_2_3_0_3_99_should_compile_to_2_3_0_6_99() {
    assert_eq!(vec![2, 3, 0, 6, 99], run(&mut vec![2, 3, 0, 3, 99], 3, 0))
}

#[test]
fn state_2_4_4_5_99_0_should_compile_to_2_4_4_5_99_9801() {
    assert_eq!(vec![2, 4, 4, 5, 99, 9801], run(&mut vec![2, 4, 4, 5, 99, 0], 4, 4))
}

#[test]
fn state_1_1_1_4_99_5_6_0_99_should_compile_to_30_1_1_4_2_5_6_0_99() {
    assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], run(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 1, 1))
}
