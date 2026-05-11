const INPUT: &str = "1113122113";

fn next_sequence(input: &str) -> String {
    let mut res = String::new();

    let chars: Vec<char> = input.chars().collect();

    let mut counter = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            counter += 1;
        } else {
            res.push_str(&counter.to_string());
            res.push(chars[i - 1]);

            counter = 1;
        }
    }

    res.push_str(&counter.to_string());
    res.push(*chars.last().unwrap());

    res
}

fn part1(input: &str) -> usize {
    let mut current = input.to_string();

    for _ in 0..40 {
        current = next_sequence(&current);
    }

    current.len()
}

fn part2(input: &str) -> usize {
    let mut current = input.to_string();

    for _ in 0..50 {
        current = next_sequence(&current);
    }

    current.len()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
