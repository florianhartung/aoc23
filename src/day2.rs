use std::ops::Not;

use itertools::*;

pub fn main() {
    let input = include_str!("../inputs/day2.txt");

    let answer: u32 = input.lines()
        .filter_map(|line| {
            let (id, content) = {
                let v = line.split(":").collect_vec();
                (v[0].split(' ').skip(1).next().unwrap().trim().parse::<u32>().unwrap(), v[1])
            };

            content.split(';').any(|set| {
                let mut r = 0;
                let mut g= 0;
                let mut b = 0;
                set.split(',').for_each(|group| {
                    let mut parts = group.trim().split(" ").map(str::trim);
                    let num = parts.next().unwrap().parse::<u32>().unwrap();
                    match parts.next().unwrap() {
                        "red" => r += num,
                        "green" => g += num,
                        "blue" => b += num,
                        x => panic!("was {x}"),
                    }
                });
                r > 12 || g > 13 || b > 14
            }).not().then_some(id)
        }).sum();

    println!("Day 2 Part 1: {answer}");
}
