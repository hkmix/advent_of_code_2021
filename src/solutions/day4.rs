use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::Iterator;

use crate::utils::Solution;
use crate::utils::XY;

pub struct Day4;

#[derive(Debug)]
struct BingoBoard {
    spaces: Vec<Vec<i64>>,
    called: Vec<Vec<bool>>,
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.spaces {
            if let Err(e) = writeln!(
                f,
                "{}",
                row.iter()
                    .map(|val| format!("{:2}", val))
                    .collect::<Vec<String>>()
                    .join(" ")
            ) {
                return Err(e);
            }
        }

        Ok(())
    }
}

impl BingoBoard {
    fn from_data_iter<'a>(iter: &mut impl Iterator<Item = &'a String>) -> Option<Self> {
        let mut spaces = Vec::with_capacity(5);
        let mut taken = 0;
        while taken < 5 {
            let line = iter.next()?;
            if line.is_empty() {
                continue;
            }

            let numbers: Vec<i64> = line
                .split(' ')
                .filter(|val| !val.is_empty())
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect();

            spaces.push(numbers);
            taken += 1;
        }

        Some(BingoBoard {
            spaces,
            called: vec![vec![false; 5]; 5],
        })
    }

    fn get_locations(&self) -> HashMap<i64, XY<usize>> {
        let mut res: HashMap<i64, XY<usize>> = HashMap::new();

        for (row_idx, row) in self.spaces.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                res.insert(*value, XY::new(row_idx, col_idx));
            }
        }

        res
    }

    fn call_value(
        &mut self,
        value: i64,
        locations: &HashMap<i64, XY<usize>>,
        allow_diagonals: bool,
    ) -> bool {
        let maybe_xy = locations.get(&value);
        if maybe_xy.is_none() {
            return false;
        }

        let xy = maybe_xy.unwrap();
        self.called[xy.x][xy.y] = true;

        // Checks.
        // Horizontal.
        if self.called[xy.x].iter().all(|val| *val) {
            return true;
        }
        // Vertical.
        else if self.called.iter().all(|row| row[xy.y]) {
            return true;
        }
        // Diagonal \.
        else if allow_diagonals
            && xy.x == xy.y
            && self.called.iter().enumerate().all(|(idx, row)| row[idx])
        {
            return true;
        }
        // Diagonal /.
        else if allow_diagonals
            && xy.x + xy.y == 4
            && self
                .called
                .iter()
                .enumerate()
                .all(|(idx, row)| row[4 - idx])
        {
            return true;
        }

        false
    }

    fn unmarked_sum(&self) -> i64 {
        let mut sum: i64 = 0;
        for row in 0usize..=4usize {
            for col in 0usize..=4usize {
                if !self.called[row][col] {
                    sum += self.spaces[row][col];
                }
            }
        }

        sum
    }
}

struct BingoGame {
    number_order: Vec<i64>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn from_data_iter<'a>(iter: &mut impl Iterator<Item = &'a String>) -> Self {
        let number_order = iter
            .next()
            .unwrap()
            .split(',')
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect();

        let mut boards = vec![];
        loop {
            let board = BingoBoard::from_data_iter(iter);
            if board.is_none() {
                break;
            }

            boards.push(board.unwrap());
        }

        BingoGame {
            number_order,
            boards,
        }
    }
}

impl Display for BingoGame {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Err(e) = writeln!(
            f,
            "Call order: {}",
            self.number_order
                .iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ) {
            return Err(e);
        }

        if let Err(e) = writeln!(f) {
            return Err(e);
        }

        for board in &self.boards {
            if let Err(e) = writeln!(f, "{}", board) {
                return Err(e);
            }
        }

        Ok(())
    }
}

impl Solution for Day4 {
    fn day_number() -> i32 {
        4
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let mut bingo_game = BingoGame::from_data_iter(&mut data.iter());

        let order = &bingo_game.number_order;
        let mut lowest: Option<usize> = None;
        let mut score: i64 = 0;
        for board in bingo_game.boards.iter_mut() {
            for (num_idx, num) in order.iter().enumerate() {
                let locations = board.get_locations();
                let bingo = board.call_value(*num, &locations, false);
                if bingo {
                    if lowest.is_none() || num_idx + 1 < lowest.unwrap() {
                        lowest = Some(num_idx + 1);
                        score = num * board.unmarked_sum();
                    }
                    break;
                }
            }
        }

        Some(score)
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let mut bingo_game = BingoGame::from_data_iter(&mut data.iter());

        let order = &bingo_game.number_order;
        let mut highest: Option<usize> = None;
        let mut score: i64 = 0;
        for board in bingo_game.boards.iter_mut() {
            for (num_idx, num) in order.iter().enumerate() {
                let locations = board.get_locations();
                let bingo = board.call_value(*num, &locations, false);
                if bingo {
                    if highest.is_none() || num_idx + 1 > highest.unwrap() {
                        highest = Some(num_idx + 1);
                        score = num * board.unmarked_sum();
                    }
                    break;
                }
            }
        }

        Some(score)
    }
}
