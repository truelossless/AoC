use std::fs::read_to_string;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn build_vents() -> Vec<((u32, u32), (u32, u32))> {
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut vent = line.split(" -> ").map(|coords| {
                let mut coord = coords.split(',').map(|s| s.parse().unwrap());
                (coord.next().unwrap(), coord.next().unwrap())
            });

            (vent.next().unwrap(), vent.next().unwrap())
        })
        .collect()
}

fn intersects(vents: impl Iterator<Item = ((u32, u32), (u32, u32))>) -> u32 {
    let mut matrix = vec![vec![0; 1000]; 1000];

    vents.for_each(|((x1, y1), (x2, y2))| {
        let x_offset = (x1 as i32 - x2 as i32).abs();
        let y_offset = (y1 as i32 - y2 as i32).abs();

        let sign = |offset, a, b| -> i32 {
            if offset != 0 && a > b {
                -1
            } else if offset != 0 {
                1
            } else {
                0
            }
        };
        let sx = sign(x_offset, x1, x2);
        let sy = sign(y_offset, y1, y2);

        for o in 0..=x_offset.max(y_offset) {
            matrix[(x1 as i32 + o * sx) as usize][(y1 as i32 + o * sy) as usize] += 1;
        }
    });

    matrix.iter().flatten().filter(|coord| **coord >= 2).count() as u32
}

fn part_one() -> u32 {
    intersects(
        build_vents()
            .into_iter()
            .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2),
    )
}

fn part_two() -> u32 {
    intersects(build_vents().into_iter())
}
