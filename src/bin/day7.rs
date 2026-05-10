use std::{collections::HashMap, fs::read_to_string};

fn get_value(
    wire: &str,
    circuit: &HashMap<String, String>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    // literal number
    if let Ok(n) = wire.parse::<u16>() {
        return n;
    }

    // cached
    if let Some(&v) = cache.get(wire) {
        return v;
    }

    let expr = circuit.get(wire).unwrap();

    let parts: Vec<&str> = expr.split_whitespace().collect();

    let value = match parts.as_slice() {
        [x] => get_value(x, circuit, cache),

        ["NOT", x] => !get_value(x, circuit, cache),

        [a, "AND", b] => get_value(a, circuit, cache) & get_value(b, circuit, cache),

        [a, "OR", b] => get_value(a, circuit, cache) | get_value(b, circuit, cache),

        [a, "LSHIFT", b] => get_value(a, circuit, cache) << b.parse::<u16>().unwrap(),

        [a, "RSHIFT", b] => get_value(a, circuit, cache) >> b.parse::<u16>().unwrap(),

        _ => unreachable!(),
    };

    cache.insert(wire.to_string(), value);

    value
}

fn part1(input: &str) -> u16 {
    let mut circuit = HashMap::new();

    for line in input.lines() {
        let (expr, wire) = line.split_once(" -> ").unwrap();

        circuit.insert(wire.to_string(), expr.to_string());
    }

    let mut cache = HashMap::new();

    get_value("a", &circuit, &mut cache)
}

fn part2(input: &str) -> u16 {
    let mut map: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let (expr, wire) = line.split_once(" -> ").unwrap();

        map.insert(wire.to_string(), expr.to_string());
    }

    let a = get_value("a", &map, &mut HashMap::new());

    map.insert("b".to_string(), a.to_string());

    get_value("a", &map, &mut HashMap::new())
}

fn main() {
    let input = read_to_string("src/input/day7.input").unwrap();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
