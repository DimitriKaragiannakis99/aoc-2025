use fancy_regex::Regex;
use once_cell::sync::Lazy;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*)\1+$").unwrap());
const DAY02_PATH: &str = "/home/pappou/Dev/aoc-2025/inputs/day02.txt";

pub fn gpt_algo(bound: &str) -> bool {
    let length = bound.len();

    for i in 1..=length / 2 {
        if length % i == 0 {
            let pattern = &bound[..i];
            if pattern.repeat(length / i) == bound {
                return true;
            }
        }
    }
    false
}

pub fn use_regex(hay: &String) -> bool {
    RE.is_match(hay).unwrap()
}

pub fn check_repeating(num: u64) -> bool {
    let str_num = num.to_string();
    let len = str_num.len();

    if len % 2 != 0 {
        return false;
    }
    let mid = len / 2;

    str_num[0..mid] == str_num[mid..]
}

pub fn check_range(range: &str, repeats: &mut Vec<u64>) -> () {
    // split range into upper and lower bound (expect format 11-22)
    let bounds: Vec<&str> = range.split('-').collect();
    let lower_bound = bounds[0].parse::<u64>().unwrap();
    let upper_bound = bounds[1].parse::<u64>().unwrap() + 1;

    for num in lower_bound..upper_bound {
        let num_str = num.to_string();
        if use_regex(&num_str) {
            repeats.push(num);
        }
    }
}

pub fn solve() -> u64 {
    // read in file contents
    let contents: String = std::fs::read_to_string(DAY02_PATH).unwrap();

    // split contents by ',' delimiter into array
    let ranges: Vec<&str> = contents.split(',').collect();

    // pass ranges one by one to check_range func
    let mut result: Vec<u64> = Vec::new();

    for range in ranges.iter() {
        check_range(range, &mut result);
    }

    result.iter().sum()
}

pub fn solve_with_gpt_algo() -> u64 {
    let contents: String = std::fs::read_to_string(DAY02_PATH).unwrap();
    let ranges: Vec<&str> = contents.split(',').collect();
    let mut result: Vec<u64> = Vec::new();

    for range in ranges.iter() {
        let bounds: Vec<&str> = range.split('-').collect();
        let lower_bound = bounds[0].parse::<u64>().unwrap();
        let upper_bound = bounds[1].parse::<u64>().unwrap() + 1;

        for num in lower_bound..upper_bound {
            let num_str = num.to_string();
            if gpt_algo(&num_str) {
                result.push(num);
            }
        }
    }
    result.iter().sum()
}
