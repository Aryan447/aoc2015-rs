use std::fs::read_to_string;

fn floor_change(floor: &mut i32, ch: char) {
    match ch {
        '(' => *floor += 1,
        ')' => *floor -= 1,
        _ => {}
    }
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;

    for ch in input.chars() {
        floor_change(&mut floor, ch);
    }

    floor
}

fn part2(input: &str) -> i32 {
    let mut floor = 0;

    for (pos, ch) in input.chars().enumerate() {
        floor_change(&mut floor, ch);

        if floor == -1 {
            return (pos + 1) as i32;
        }
    }

    0
}

fn main() {
    let input = read_to_string("src/input/day1.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
