use rayon::prelude::*;

pub fn main() {
    let input = include_str!("../inputs/day1.txt");

    let calibration_value_sum: u32 = input.par_lines().map(|line| {
        let first_digit = line.chars().find(|c| c.is_digit(10));
        let last_digit = line.chars().rfind(|c| c.is_digit(10));

        let (first, last) = first_digit.zip(last_digit)
            .expect("there to be a digit per line");

        String::from_iter([first, last]).parse::<u32>().expect("this to be parseable because the string only consists of two digits")
    }).sum();

    println!("Day 1 Part 1: {calibration_value_sum}");
}