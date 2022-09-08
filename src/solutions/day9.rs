use std::collections::BinaryHeap;
use std::collections::HashSet;

use crate::utils::Solution;

pub struct Day9;

#[derive(Debug)]
struct Grid {
    pub data: Vec<Vec<i64>>,
}

impl Grid {
    fn from_data(data: Vec<String>) -> Self {
        Grid {
            data: data
                .into_iter()
                .map(|s| {
                    s.chars()
                        .map(|c| str::parse::<i64>(&c.to_string()))
                        .map(Result::unwrap)
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>(),
        }
    }

    fn at(&self, row: i64, col: i64) -> Option<i64> {
        if self.data.is_empty() {
            return None;
        }

        if row < 0
            || col < 0
            || row as usize >= self.data.len()
            || col as usize >= self.data[0].len()
        {
            None
        } else {
            Some(self.data[row as usize][col as usize])
        }
    }

    fn dfs_count(&self, row: i64, col: i64, seen: &mut HashSet<(i64, i64)>) -> i64 {
        if self.at(row, col).unwrap_or(9) == 9 || seen.contains(&(row, col)) {
            return 0;
        }

        let mut count: i64 = 1;
        seen.insert((row, col));
        count += self.dfs_count(row - 1, col, seen);
        count += self.dfs_count(row + 1, col, seen);
        count += self.dfs_count(row, col - 1, seen);
        count += self.dfs_count(row, col + 1, seen);

        count
    }
}

fn less_than(lhs: i64, rhs: Option<i64>) -> bool {
    match rhs {
        Some(val) => lhs < val,
        None => true,
    }
}

impl Solution for Day9 {
    fn day_number() -> i32 {
        9
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let grid = Grid::from_data(data);
        let mut total: i64 = 0;
        for (row_idx, row) in grid.data.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                let (row_idx_i64, col_idx_i64) = (row_idx as i64, col_idx as i64);
                if less_than(*val, grid.at(row_idx_i64, col_idx_i64 - 1))
                    && less_than(*val, grid.at(row_idx_i64, col_idx_i64 + 1))
                    && less_than(*val, grid.at(row_idx_i64 - 1, col_idx_i64))
                    && less_than(*val, grid.at(row_idx_i64 + 1, col_idx_i64))
                {
                    total += val + 1;
                }
            }
        }

        Some(total)
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut largest: BinaryHeap<i64> = BinaryHeap::with_capacity(4);
        let grid = Grid::from_data(data);

        // Any DFS/BFS will do, just separate by whether the number is 9.
        for row_idx in 0..grid.data.len() as i64 {
            for col_idx in 0..grid.data[0].len() as i64 {
                let count = grid.dfs_count(row_idx, col_idx, &mut seen);
                largest.push(count);

                if largest.len() == 4 {
                    let mut largest_next: BinaryHeap<i64> = BinaryHeap::with_capacity(4);
                    for _ in 0..3 {
                        largest_next.push(largest.pop().unwrap());
                    }

                    largest = largest_next;
                }
            }
        }

        let mut product: i64 = 1;
        for _ in 0..largest.len() {
            product *= largest.pop().unwrap();
        }

        Some(product)
    }
}
