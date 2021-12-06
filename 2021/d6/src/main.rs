use std::fs::read_to_string;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn fishes(days: u32) -> u64 {
    let mut fish_days = vec![0; 9];

    read_to_string("input.txt")
        .unwrap()
        .split(',')
        .for_each(|day| fish_days[day.parse::<usize>().unwrap()] += 1);

    for _ in 0..days {
        let new_fishes = fish_days.remove(0);
        fish_days.push(new_fishes);
        fish_days[6] += new_fishes;
    }

    fish_days.iter().sum()
}

fn part_one() -> u64 {
    fishes(80)
}

fn part_two() -> u64 {
    fishes(256)
}
