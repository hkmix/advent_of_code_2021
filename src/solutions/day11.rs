use std::collections::HashSet;

use crate::solutions::day9::Grid;
use crate::utils::Solution;

pub struct Day11;

impl Grid {
    fn increment(&mut self, row: i64, col: i64) -> bool {
        if let Some(val_ref) = self.at_mut(row, col) {
            *val_ref += 1;
            *val_ref == 10
        } else {
            false
        }
    }

    fn increment_adjacent(&mut self, row: i64, col: i64) -> HashSet<(i64, i64)> {
        let mut overflowed: HashSet<(i64, i64)> = HashSet::new();
        for d_row in -1..=1 {
            for d_col in -1..=1 {
                if let Some(val_ref) = self.at_mut(row + d_row, col + d_col) {
                    *val_ref += 1;
                    if *val_ref >= 10 {
                        overflowed.insert((row + d_row, col + d_col));
                    }

                    // For first overflow, apply additional overflow.
                    if *val_ref == 10 {
                        for (r, c) in self.increment_adjacent(row + d_row, col + d_col) {
                            overflowed.insert((r, c));
                        }
                    }
                }
            }
        }

        overflowed
    }

    fn is_all_zeroes(&self) -> bool {
        self.data
            .iter()
            .all(|vec_i64| vec_i64.iter().all(|&val| val == 0))
    }
}

trait Automata<T> {
    fn step(&mut self) -> T;
}

impl Automata<i64> for Grid {
    fn step(&mut self) -> i64 {
        let mut overflowed: HashSet<(i64, i64)> = HashSet::new();
        for row in 0..self.data.len() as i64 {
            for col in 0..self.data[0].len() as i64 {
                if self.increment(row, col) {
                    overflowed.insert((row, col));
                    for (r, c) in self.increment_adjacent(row, col) {
                        overflowed.insert((r, c));
                    }
                }
            }
        }

        for (row, col) in &overflowed {
            if let Some(val_ref) = self.at_mut(*row, *col) {
                *val_ref = 0;
            }
        }

        overflowed.len() as i64
    }
}

impl Solution for Day11 {
    fn day_number() -> i32 {
        11
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let mut grid = Grid::from_data(data);
        let mut res: i64 = 0;
        for _ in 0..100 {
            res += grid.step();
        }

        res.into()
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let mut grid = Grid::from_data(data);
        for step in 0.. {
            if grid.is_all_zeroes() {
                return (step as i64).into();
            }
            grid.step();
        }

        unreachable!()
    }
}
