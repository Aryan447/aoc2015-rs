const INPUT: &str = "hepxcrrq";

fn increment(password: &mut Vec<char>) {
    for i in (0..password.len()).rev() {
        if password[i] == 'z' {
            password[i] = 'a';
        } else {
            password[i] = ((password[i] as u8) + 1) as char;

            break;
        }
    }
}

fn has_straight(password: &[char]) -> bool {
    for i in 0..password.len() - 2 {
        let a = password[i] as u8;
        let b = password[i + 1] as u8;
        let c = password[i + 2] as u8;

        if a + 1 == b && b + 1 == c {
            return true;
        }
    }
    false
}

fn has_no_forbidden(password: &[char]) -> bool {
    !password.iter().any(|&c| matches!(c, 'i' | 'o' | 'l'))
}

fn has_two_pairs(password: &[char]) -> bool {
    let mut pairs = 0;

    let mut i = 0;

    while i < password.len() - 1 {
        if password[i] == password[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    pairs >= 2
}

fn is_valid(password: &[char]) -> bool {
    has_straight(password) && has_no_forbidden(&password) && has_two_pairs(&password)
}

fn part1(input: &str) -> String {
    let mut password: Vec<char> = input.chars().collect();

    loop {
        increment(&mut password);
        if is_valid(&password) {
            return password.iter().collect();
        }
    }
}

fn part2(input: &str) -> String {
    part1(input)
}

fn main() {
    let p1 = part1(INPUT);

    println!("part1: {}", p1);
    println!("part2: {}", part2(&p1));
}
