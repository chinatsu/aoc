#[macro_use] mod inputreader;
mod aoc01;
mod aoc02;

fn main() {
    println!("Solution to 01 pt. 1: {}", aoc01::part_1());
    println!("Solution to 01 pt. 2: {}", aoc01::part_2());
    println!("Solution to 02 pt. 1: {}", aoc02::part_1());
    println!("Solution to 02 pt. 2: {}", aoc02::part_2());
}
