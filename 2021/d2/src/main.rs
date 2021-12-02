use std::fs::read_to_string;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u32 {
    let mut forward = 0;
    let mut up = 0;
    let mut down = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        match line.chars().next().unwrap() {
            'f' => forward += line.chars().nth(8).unwrap().to_digit(10).unwrap(),
            'u' => up += line.chars().nth(3).unwrap().to_digit(10).unwrap(),
            'd' => down += line.chars().nth(5).unwrap().to_digit(10).unwrap(),
            _ => unreachable!(),
        }
    }

    forward * (down - up)
}

fn part_two() -> u32 {
    let mut aim: i32 = 0;
    let mut depth = 0;
    let mut forward = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        match line.chars().next().unwrap() {
            'f' => {
                let x = line.chars().nth(8).unwrap().to_digit(10).unwrap();
                forward += x;
                depth += aim as u32 * x;
            }
            'u' => aim -= line.chars().nth(3).unwrap().to_digit(10).unwrap() as i32,
            'd' => aim += line.chars().nth(5).unwrap().to_digit(10).unwrap() as i32,
            _ => unreachable!(),
        }
    }

    forward * depth
}
