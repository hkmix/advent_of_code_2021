pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use crate::utils::Solution;

#[allow(dead_code)]
pub fn run_latest() {
    day9::Day9::run();
}

#[allow(dead_code)]
pub fn run_all() {
    day1::Day1::run();
    day2::Day2::run();
    day3::Day3::run();
    day4::Day4::run();
    day5::Day5::run();
    day6::Day6::run();
    day7::Day7::run();
    day8::Day8::run();
    run_latest();
}
