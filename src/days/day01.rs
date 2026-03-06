use num::integer::div_floor;
use std::fs;

const DAY01_FILE_PATH: &str = "/home/pappou/Dev/aoc-2025/inputs/day01.txt";

pub fn process_command(dial: i32, command: &str) -> (i32, i32, i32) {
    let direction = command.chars().next().unwrap();
    let increment: i32 = command[1..].parse().unwrap();

    match direction {
        'L' => {
            let updated_dial = (dial - increment).rem_euclid(100);
            let count_inc_day01 = if updated_dial == 0 { 1 } else { 0 };
            let count_inc_day02 = div_floor(increment - dial + 99, 100);
            return (updated_dial, count_inc_day01, count_inc_day02);
        }
        'R' => {
            let updated_dial = (dial + increment).rem_euclid(100);
            let count_inc_day01 = if updated_dial == 0 { 1 } else { 0 };
            let count_inc_day02 = div_floor(dial + increment, 100);
            return (updated_dial, count_inc_day01, count_inc_day02);
        }
        _ => {
            panic!("Invalid command");
        }
    }
}

pub fn solve() -> (i32, i32) {
    let contents = fs::read_to_string(&DAY01_FILE_PATH).unwrap();
    let parts: Vec<&str> = contents.lines().collect();
    let mut dial: i32 = 50;
    let mut count_day01: i32 = 0;
    let mut count_day02: i32 = 0;

    for command in parts {
        let (updated_dial, updated_count_day01, updated_count_day02) =
            process_command(dial, command);
        dial = updated_dial;
        count_day01 += updated_count_day01;
        count_day02 += updated_count_day02;
    }
    (count_day01, count_day02)
}
