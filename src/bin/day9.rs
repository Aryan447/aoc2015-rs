use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

fn parse_input(input: &str) -> (HashMap<(&str, &str), u32>, Vec<&str>) {
    let mut distances = HashMap::new();

    let mut cities = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let from = parts[0];
        let to = parts[2];
        let dist: u32 = parts[4].parse().unwrap();

        distances.insert((from, to), dist);
        distances.insert((to, from), dist);

        cities.insert(from);
        cities.insert(to);
    }

    (distances, cities.into_iter().collect())
}

fn solve(input: &str, find_min: bool) -> u32 {
    let (distances, cities) = parse_input(input);

    let mut answer = if find_min { u32::MAX } else { u32::MIN };

    for perm in cities.iter().permutations(cities.len()) {
        let mut total = 0;

        for pair in perm.windows(2) {
            total += distances[&(*pair[0], *pair[1])];
        }

        if find_min {
            answer = answer.min(total);
        } else {
            answer = answer.max(total);
        }
    }

    answer
}

fn part1(input: &str) -> u32 {
    solve(input, true)
}

fn part2(input: &str) -> u32 {
    solve(input, false)
}

fn main() {
    let input = read_to_string("src/input/day9.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
