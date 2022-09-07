use crate::utils::{Solution, XY};

pub struct Day2;

impl Solution for Day2 {
    fn day_number() -> i32 {
        2
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let mut pos: XY<i64> = XY::new(0, 0);
        for line in data {
            match &line.split(" ").collect::<Vec<&str>>()[..] {
                ["forward", val] => pos.x += val.parse::<i64>().unwrap(),
                ["down", val] => pos.y += val.parse::<i64>().unwrap(),
                ["up", val] => pos.y -= val.parse::<i64>().unwrap(),
                _ => panic!("Unexpected command: {}", line),
            }
        }

        Some(pos.x * pos.y)
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let mut pos: XY<i64> = XY::new(0, 0);
        let mut aim = 0i64;
        for line in data {
            match &line.split(" ").collect::<Vec<&str>>()[..] {
                ["down", val] => aim += val.parse::<i64>().unwrap(),
                ["up", val] => aim -= val.parse::<i64>().unwrap(),
                ["forward", val] => {
                    let parsed = val.parse::<i64>().unwrap();
                    pos.x += parsed;
                    pos.y += aim * parsed;
                }
                _ => panic!("Unexpected command: {}", line),
            }
        }

        Some(pos.x * pos.y)
    }
}
