const DAY02_PATH: &str = "/home/pappou/Dev/aoc-2025/inputs/day02.txt";

pub fn solve() {
    // read in file contents
    let contents: String = std::fs::read_to_string(DAY02_PATH).unwrap();

    // split contents by ',' delimiter into array
    let ranges: Vec<&str> = contents.split(',').collect();

    println!("{:?}", ranges)
}
