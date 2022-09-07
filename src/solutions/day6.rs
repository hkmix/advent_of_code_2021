use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::utils::Solution;

pub struct Day6;

struct LanternfishAutomata {
    // Mapping of {counter_value => number of lanternfish}.
    counters: HashMap<i64, i64>,
}

impl LanternfishAutomata {
    fn from_input(s: &str) -> Self {
        let mut counters: HashMap<i64, i64> = HashMap::new();
        for counter in s.split(',').map(str::parse::<i64>).map(Result::unwrap) {
            match counters.entry(counter) {
                Entry::Occupied(mut e) => {
                    e.insert(e.get() + 1);
                }
                Entry::Vacant(e) => {
                    e.insert(1);
                }
            };
        }

        LanternfishAutomata { counters }
    }

    fn step(&mut self) {
        let new_fish = *self.counters.get(&0).unwrap_or(&0);
        for counter in 1..=8 {
            self.counters
                .insert(counter - 1, *self.counters.get(&counter).unwrap_or(&0));
        }

        match self.counters.entry(6) {
            Entry::Occupied(mut e) => {
                e.insert(e.get() + new_fish);
            }
            Entry::Vacant(e) => {
                e.insert(new_fish);
            }
        }

        self.counters.insert(8, new_fish);
    }

    fn size(&self) -> i64 {
        self.counters.values().sum()
    }
}

impl Solution for Day6 {
    fn day_number() -> i32 {
        6
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        assert_eq!(
            data.len(),
            1,
            "Lanternfish automata requires one line of input"
        );

        let mut automata = LanternfishAutomata::from_input(&data[0]);
        for _ in 0..80 {
            automata.step();
        }

        Some(automata.size())
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        assert_eq!(
            data.len(),
            1,
            "Lanternfish automata requires one line of input"
        );

        let mut automata = LanternfishAutomata::from_input(&data[0]);
        for _ in 0..256 {
            automata.step();
        }

        Some(automata.size())
    }
}
