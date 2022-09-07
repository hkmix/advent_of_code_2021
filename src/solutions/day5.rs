use crate::utils::{Solution, XY};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub struct Day5;

struct Line {
    start: XY<i64>,
    end: XY<i64>,
}

fn range(start: i64, end: i64, include_end: bool) -> Vec<i64> {
    let order = if start < end {
        (start, end)
    } else {
        (end, start)
    };

    let vec = if include_end {
        (order.0..=order.1).collect()
    } else {
        (order.0..order.1).collect()
    };

    if start == order.0 {
        vec
    } else {
        vec.into_iter().rev().collect()
    }
}

impl Line {
    pub fn new(start: XY<i64>, end: XY<i64>) -> Self {
        Line { start, end }
    }

    pub fn get_points(&self, allow_diagonals: bool) -> Vec<XY<i64>> {
        if self.start.x == self.end.x {
            range(self.start.y, self.end.y, true)
                .into_iter()
                .map(|y| XY::new(self.start.x, y))
                .collect()
        } else if self.start.y == self.end.y {
            range(self.start.x, self.end.x, true)
                .into_iter()
                .map(|x| XY::new(x, self.start.y))
                .collect()
        } else if !allow_diagonals {
            vec![]
        } else {
            range(self.start.x, self.end.x, true)
                .into_iter()
                .zip(range(self.start.y, self.end.y, true))
                .map(|(x, y)| XY::new(x, y))
                .collect()
        }
    }

    fn from_string(s: &str) -> Self {
        let splits = s.split(" -> ").collect::<Vec<&str>>();
        assert_eq!(splits.len(), 2, "Line \"{}\" malformed", s);

        let mut xys = splits.into_iter().map(XY::from).collect::<Vec<XY<i64>>>();
        Line::new(xys.pop().unwrap(), xys.pop().unwrap())
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl Solution for Day5 {
    fn day_number() -> i32 {
        5
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let lines = data
            .iter()
            .map(String::as_str)
            .map(Line::from_string)
            .collect::<Vec<Line>>();

        let mut seen: HashSet<XY<i64>> = HashSet::new();
        let mut duped: HashSet<XY<i64>> = HashSet::new();
        for line in lines {
            for point in line.get_points(false) {
                if seen.contains(&point) {
                    duped.insert(point);
                } else {
                    seen.insert(point);
                }
            }
        }

        Some(duped.len() as i64)
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let lines = data
            .iter()
            .map(String::as_str)
            .map(Line::from_string)
            .collect::<Vec<Line>>();

        let mut seen: HashSet<XY<i64>> = HashSet::new();
        let mut duped: HashSet<XY<i64>> = HashSet::new();
        for line in lines {
            for point in line.get_points(true) {
                if seen.contains(&point) {
                    duped.insert(point);
                } else {
                    seen.insert(point);
                }
            }
        }

        Some(duped.len() as i64)
    }
}
