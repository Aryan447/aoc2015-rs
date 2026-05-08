use md5;

const INPUT: &str = "yzbqklnj";

fn find_hash(prefix: &str) -> u32 {
    let mut n: u32 = 0;

    loop {
        let s = format!("{INPUT}{n}");

        let hash = format!("{:x}", md5::compute(s));

        if hash.starts_with(prefix) {
            return n;
        }
        n += 1;
    }
}

fn main() {
    println!("part1: {}", find_hash("00000"));
    println!("part2: {}", find_hash("000000"));
}
