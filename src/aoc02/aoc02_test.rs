use super::*;

#[test]
fn first_value_in_csv_should_be_1() {
    assert_eq!(&1, csv_to_numbers().first().unwrap())
}
