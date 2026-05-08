use std::{collections::HashSet, fs::read_to_string};

fn move_santa(direction: char, x: &mut i32, y: &mut i32) {
    match direction {
        '^' => *x += 1,
        'v' => *x -= 1,
        '>' => *y += 1,
        '<' => *y -= 1,
        _ => (),
    }
}

fn part1(input: &str) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    seen.insert((x, y));

    for direction in input.chars() {
        move_santa(direction, &mut x, &mut y);

        seen.insert((x, y));
    }

    seen.len()
}

fn part2(input: &str) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    let mut santa = (0, 0);
    let mut robo = (0, 0);

    seen.insert((0, 0));

    for (i, direction) in input.chars().enumerate() {
        let current = if i & 1 == 0 { &mut santa } else { &mut robo };

        move_santa(direction, &mut current.0, &mut current.1);

        seen.insert(*current);
    }

    seen.len()
}

fn main() {
    let input = read_to_string("src/input/day3.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
