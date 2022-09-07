use crate::utils;

use std::collections::VecDeque;

pub struct Day1;

struct SumQueue {
    vec: VecDeque<i64>,
    sum: i64,
    testable: bool,
}

impl SumQueue {
    pub fn new(size: usize) -> Self {
        SumQueue {
            vec: VecDeque::with_capacity(size),
            sum: 0,
            testable: false,
        }
    }

    pub fn sum(&self) -> i64 {
        self.sum
    }

    pub fn testable(&self) -> bool {
        self.testable
    }

    pub fn at_capacity(&self) -> bool {
        self.vec.len() == self.vec.capacity()
    }

    pub fn push(&mut self, num: i64) -> &mut Self {
        if self.at_capacity() {
            self.sum -= self.vec.pop_front().unwrap();
            self.testable = true;
        }

        self.vec.push_back(num);
        self.sum += num;
        self
    }
}

impl utils::Solution for Day1 {
    fn day_number() -> i32 {
        1
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let mut prev: Option<i64> = None;
        let mut count = 0i64;
        for line in data {
            let parsed: i64 = line.parse().unwrap();
            if prev.is_none() {
                prev = parsed.into();
            }

            if parsed > prev.unwrap() {
                count += 1;
            }

            prev = parsed.into();
        }

        Some(count)
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let mut prev: Option<i64> = None;
        let mut count = 0i64;
        let mut queue = SumQueue::new(3);
        for line in data {
            let parsed: i64 = line.parse().unwrap();
            let new_sum = queue.push(parsed).sum();
            if prev.is_none() {
                prev = new_sum.into();
                continue;
            }

            if queue.testable() && new_sum > prev.unwrap() {
                count += 1;
            }

            prev = new_sum.into();
        }

        Some(count)
    }
}
