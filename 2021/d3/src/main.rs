use std::fs::read_to_string;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u32 {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let line_count = lines.clone().count();
    let gamma = lines
        .fold([0; 12], |mut acc, line| {
            line.chars().enumerate().for_each(|(i, c)| {
                if c == '1' {
                    acc[i] += 1;
                }
            });
            acc
        })
        .map(|one_count| if one_count > line_count / 2 { '1' } else { '0' })
        .iter()
        .collect::<String>();

    let epsilon = gamma
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();

    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

// from now on, i'll be doing the challenges in a more imperative way,
// as it's going to be harder and harder to map these in a functional style.
fn part_two() -> u32 {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();

    let rating = |keep_most_common: bool| -> u32 {
        let mut filtered_lines = lines.clone().collect::<Vec<_>>();
        let (most_common, least_common) = if keep_most_common {
            ('1', '0')
        } else {
            ('0', '1')
        };

        for i in 0.. {
            let one_count = filtered_lines
                .iter()
                .filter(|line| line.chars().nth(i).unwrap() == '1')
                .count();

            let selected_bit = if one_count >= (filtered_lines.len() as f32 / 2.).ceil() as usize {
                most_common
            } else {
                least_common
            };

            filtered_lines = filtered_lines
                .iter()
                .filter(|line| line.chars().nth(i).unwrap() == selected_bit)
                .copied()
                .collect();

            if filtered_lines.len() == 1 {
                break;
            }
        }

        u32::from_str_radix(filtered_lines[0], 2).unwrap()
    };

    rating(true) * rating(false)
}
