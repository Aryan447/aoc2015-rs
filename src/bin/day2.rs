use std::fs::read_to_string;

fn dimensions(line: &str) -> [u32; 3] {
    let mut parts = line.split('x').map(|x| x.parse().unwrap());

    [
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    ]
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let [l, w, h] = dimensions(line);

            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;

            let surface_area = 2 * side1 + 2 * side2 + 2 * side3;
            let slack = side1.min(side2).min(side3);

            surface_area + slack
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dims = dimensions(line);

            dims.sort();

            let ribbon_wrap = 2 * dims[0] + 2 * dims[1];
            let bow = dims[0] * dims[1] * dims[2];

            ribbon_wrap + bow
        })
        .sum()
}

fn main() {
    let input = read_to_string("src/input/day2.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
