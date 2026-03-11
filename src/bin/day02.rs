use std::time::SystemTime;

use aoc_2025::days::day02;

fn main() {
    // algo 1
    let start_algo1 = SystemTime::now();
    let day_02_answer: u64 = day02::solve();
    let end_algo1 = SystemTime::now();
    let elapsed_time_algo1 = end_algo1.duration_since(start_algo1);

    // algo 2
    let start_algo2 = SystemTime::now();
    let day_02_answer_algo2 = day02::solve_with_gpt_algo();
    let end_algo2 = SystemTime::now();
    let elapsed_time_algo2 = end_algo2.duration_since(start_algo2);

    println!(
        "Day 02 Answer: {}\nElapsed Time Algo #1: {:?}\nDay 02 Answer: {}\nElapsed Time Algo #2: {:?}",
        day_02_answer, elapsed_time_algo1, day_02_answer_algo2, elapsed_time_algo2
    );
}
