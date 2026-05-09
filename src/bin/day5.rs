use std::fs::read_to_string;

fn is_nice_part1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut has_forbidden = false;

    let mut prev = '\0';

    for ch in s.chars() {
        // vowels
        if matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowel_count += 1;
        }

        // double letters
        if ch == prev {
            has_double = true;
        }

        // forbidden pairs
        if matches!(
            (prev, ch),
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
        ) {
            has_forbidden = true;
            break;
        }

        prev = ch;
    }

    vowel_count >= 3 && has_double && !has_forbidden
}

fn is_nice_part2(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    let mut has_pair = false;
    let mut has_repeat = false;

    // pair appears twice without overlapping
    for i in 0..chars.len().saturating_sub(1) {
        for j in i + 2..chars.len().saturating_sub(1) {
            if chars[i] == chars[j] && chars[i + 1] == chars[j + 1] {
                has_pair = true;
            }
        }
    }

    // repeating letter with one between
    for i in 0..chars.len().saturating_sub(2) {
        if chars[i] == chars[i + 2] {
            has_repeat = true;
        }
    }

    has_pair && has_repeat
}

fn part1(input: &str) -> usize {
    input.lines().filter(|line| is_nice_part1(line)).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|line| is_nice_part2(line)).count()
}

fn main() {
    let input = read_to_string("src/input/day5.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
