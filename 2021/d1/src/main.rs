use std::fs::read_to_string;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u32 {
    let mut cmp = u32::MAX;
    let mut acc = 0;

    read_to_string("input.txt").unwrap().lines().for_each(|x| {
        let x = x.parse().unwrap();
        if x > cmp {
            acc += 1;
        }
        cmp = x;
    });

    acc
}

fn part_two() -> u32 {
    let mut cmp = u32::MAX;
    let mut acc = 0;
    let file = read_to_string("input.txt").unwrap();
    let vec: Vec<_> = file.lines().collect();

    vec.windows(3).for_each(|window| {
        let sum = window.iter().map(|x| x.parse::<u32>().unwrap()).sum();
        if sum > cmp {
            acc += 1;
        }
        cmp = sum;
    });

    acc
}
