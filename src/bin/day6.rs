use std::fs::read_to_string;

enum Command {
    On,
    Off,
    Toggle,
}

fn parse_line(line: &str) -> (Command, usize, usize, usize, usize) {
    let (command, rest) = if let Some(rest) = line.strip_prefix("turn on ") {
        (Command::On, rest)
    } else if let Some(rest) = line.strip_prefix("turn off ") {
        (Command::Off, rest)
    } else {
        (Command::Toggle, line.strip_prefix("toggle ").unwrap())
    };

    let (start, end) = rest.split_once(" through ").unwrap();

    let (x1, y1) = start.split_once(',').unwrap();
    let (x2, y2) = end.split_once(',').unwrap();

    (
        command,
        x1.parse().unwrap(),
        y1.parse().unwrap(),
        x2.parse().unwrap(),
        y2.parse().unwrap(),
    )
}

fn part1(input: &str) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];

    for line in input.lines() {
        let (command, x1, y1, x2, y2) = parse_line(line);

        for x in x1..=x2 {
            for y in y1..=y2 {
                match command {
                    Command::On => grid[x][y] = true,
                    Command::Off => grid[x][y] = false,
                    Command::Toggle => grid[x][y] = !grid[x][y],
                }
            }
        }
    }

    grid.iter().flatten().filter(|&&x| x).count()
}

fn part2(input: &str) -> u32 {
    let mut grid = vec![vec![0u32; 1000]; 1000];

    for line in input.lines() {
        let (command, x1, y1, x2, y2) = parse_line(line);

        for x in x1..=x2 {
            for y in y1..=y2 {
                match command {
                    Command::On => grid[x][y] += 1,

                    Command::Off => grid[x][y] = grid[x][y].saturating_sub(1),

                    Command::Toggle => grid[x][y] += 2,
                }
            }
        }
    }

    grid.iter().flatten().sum()
}

fn main() {
    let input = read_to_string("src/input/day6.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
