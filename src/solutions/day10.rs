use std::collections::{HashMap, VecDeque};

use crate::utils::{Part, Solution};

pub struct Day10;

fn value_of(c: Option<char>) -> i64 {
    match c {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        _ => 0,
    }
}

fn part2_score(s: &str) -> i64 {
    let scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut score: i64 = 0;
    for c in s.chars() {
        score *= 5;
        score += scores.get(&c).unwrap();
    }

    score
}

fn first_unmatched(s: &str) -> Option<char> {
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push_back(c);
            }
            ')' | ']' | '}' | '>' => {
                let back = stack.pop_back();
                if back.is_none() || pairs.get(&c).unwrap() != &back.unwrap() {
                    return Some(c);
                }
            }
            ch => panic!("Unsupported char '{}'", ch),
        }
    }

    None
}

fn all_unmatched(s: &str) -> Option<String> {
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push_back(c);
            }
            ')' | ']' | '}' | '>' => {
                let back = stack.pop_back();
                if back.is_none() || pairs.get(&c).unwrap() != &back.unwrap() {
                    return None;
                }
            }
            ch => panic!("Unsupported char '{}'", ch),
        }
    }

    if !stack.is_empty() {
        Some(stack.iter().rev().collect::<String>())
    } else {
        None
    }
}

impl Solution for Day10 {
    fn day_number() -> i32 {
        10
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        data.iter()
            .map(String::as_str)
            .map(first_unmatched)
            .map(value_of)
            .sum::<i64>()
            .into()
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let mut scores = data
            .iter()
            .map(String::as_str)
            .map(all_unmatched)
            .filter(Option::is_some)
            .map(|s| part2_score(&s.unwrap()))
            .collect::<Vec<i64>>();

        scores.sort_unstable();
        scores[scores.len() / 2].into()
    }
}
