#![feature(conservative_impl_trait)]

fn input() -> impl Iterator<Item=char> + Clone {
    include_str!("inputs/day1.txt")
        .chars()
}

fn numbers() -> impl Iterator<Item=u32> {
    input()
        .filter(|c| c != &'\n')
        .map(|c| c.to_digit(10).unwrap())
}

fn numbers_wrapped(wrap_size: usize) -> impl Iterator<Item=u32> {
    input()
        .cycle()
        .filter(|c| c != &'\n')
        .map(|c| c.to_digit(10).unwrap())
        .skip(wrap_size)
}

fn main() {
    let halfway = (input().count() - 1) / 2;
    let data = numbers().zip(numbers_wrapped(halfway));
    let mut sum = 0;

    for (i, j) in data {
        if i == j {
            sum += i;
        }
    }

    println!("{:?}", sum);
}
