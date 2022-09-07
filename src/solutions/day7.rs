use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::utils::Solution;

pub struct Day7;

fn counterize(data: Vec<String>) -> HashMap<i64, i64> {
    assert_eq!(data.len(), 1, "Input data must have only one line of input");
    let mut res: HashMap<i64, i64> = HashMap::new();
    for val in data[0]
        .split(',')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>()
    {
        match res.entry(val) {
            Entry::Occupied(mut e) => {
                e.insert(*e.get() + 1);
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    res
}

fn get_abs_delta_sum(counts: &HashMap<i64, i64>, endpoint: i64) -> i64 {
    counts
        .iter()
        .map(|(value, count)| count * (value - endpoint).abs())
        .sum()
}

fn triangle(val: i64) -> i64 {
    val * (val + 1) / 2
}

fn get_abs_delta_triangle_sum(counts: &HashMap<i64, i64>, endpoint: i64) -> i64 {
    counts
        .iter()
        .map(|(value, count)| count * triangle((value - endpoint).abs()))
        .sum()
}

impl Solution for Day7 {
    fn day_number() -> i32 {
        7
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let counts = counterize(data);
        (*counts.keys().min().unwrap()..=*counts.keys().max().unwrap())
            .map(|c| get_abs_delta_sum(&counts, c))
            .min()
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let counts = counterize(data);
        (*counts.keys().min().unwrap()..=*counts.keys().max().unwrap())
            .map(|c| get_abs_delta_triangle_sum(&counts, c))
            .min()
    }
}
