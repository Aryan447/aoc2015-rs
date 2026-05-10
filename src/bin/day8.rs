use std::fs::read_to_string;

fn part1(input: &str) -> usize {
    let mut total = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let code_len = chars.len();

        let mut memory_len = 0;

        let mut i = 1;

        while i < chars.len() - 1 {
            if chars[i] == '\\' {
                if chars[i + 1] == 'x' { i += 4 } else { i += 2 }
            } else {
                i += 1
            }

            memory_len += 1;
        }

        total += code_len - memory_len;
    }

    total
}

fn part2(input: &str) -> usize {
    let mut total = 0;

    for line in input.lines() {
        let mut encoded_len = 2;

        for ch in line.chars() {
            if ch == '"' || ch == '\\' {
                encoded_len += 2
            } else {
                encoded_len += 1
            }
        }

        total += encoded_len - line.len();
    }

    total
}

fn main() {
    let input = read_to_string("src/input/day8.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
