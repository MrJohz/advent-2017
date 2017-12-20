extern crate itertools;

use itertools::Itertools;

fn checksum<T: Iterator<Item=u32>>(numbers: T) -> u32 {
    let (max, min) = numbers
        .fold((u32::min_value(), u32::max_value()), |acc, i| {
            let (mut max, mut min) = acc;
            if i > max { max = i }
            if i < min { min = i }
            (max, min)
        });

    max - min
}

fn divisble<T: Iterator<Item=u32>>(numbers: T) -> u32 {
    let pair = numbers
        .combinations(2)
        .filter(|each| (each[0] % each[1]) == 0 || (each[1] % each[0]) == 0)
        .take(1)
        .next()
        .unwrap();

    if pair[0] > pair[1] {
        pair[0] / pair[1]
    } else {
        pair[1] / pair[0]
    }
}

fn main() {
    let sum = include_str!("inputs/day2.txt")
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|i| i.parse::<u32>().unwrap());

            divisble(numbers)
        })
        .sum::<u32>();


    println!("{:?}", sum);
}
