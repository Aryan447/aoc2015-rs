use std::fs::read_to_string;

use serde_json::Value;

fn part1(input: &str) -> i32 {
    let mut sum = 0;

    let mut current = String::new();

    for ch in input.chars() {
        if ch.is_ascii_digit() || ch == '-' {
            current.push(ch);
        } else if !current.is_empty() {
            sum += current.parse::<i32>().unwrap();

            current.clear();
        }
    }

    // last number
    if !current.is_empty() {
        sum += current.parse::<i32>().unwrap();
    }

    sum
}

fn solve(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),

        Value::Array(arr) => {
            arr.iter().map(solve).sum()
        }

        Value::Object(obj) => {
            // ignore object containing "red"
            if obj.values().any(|v| v == "red") {
                0
            } else {
                obj.values().map(solve).sum()
            }
        }

        _ => 0,
    }
}

fn part2(input: &str) -> i64 {
    let json: Value =
        serde_json::from_str(input).unwrap();

    solve(&json)
}

fn main() {
    let input = read_to_string("src/input/day12.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
