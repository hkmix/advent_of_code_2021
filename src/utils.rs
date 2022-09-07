use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum SolutionMode {
    SAMPLE,
    INPUT,
}

#[derive(Copy, Clone, Debug)]
pub struct Part(i32);

enum Colour {
    RESET,
    BOLD,
}

impl Colour {
    pub fn to_string(&self) -> &'static str {
        #[cfg(unix)]
        {
            match self {
                Colour::RESET => "\x1b[0m",
                Colour::BOLD => "\x1b[1m",
            }
        }

        #[cfg(not(unix))]
        {
            return "";
        }
    }
}

#[derive(Debug)]
struct ReadablePathBuf(std::path::PathBuf);

fn base_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("res")
}

impl From<std::path::PathBuf> for ReadablePathBuf {
    fn from(pathbuf: std::path::PathBuf) -> Self {
        Self(pathbuf)
    }
}

impl SolutionMode {
    fn to_path(&self, problem_number: i32) -> ReadablePathBuf {
        let day_str = format!("day{problem_number}");
        let day_root = base_path().join(&day_str);

        match self {
            SolutionMode::SAMPLE => day_root.join("sample.txt").into(),
            SolutionMode::INPUT => day_root.join("input.txt").into(),
        }
    }
}

impl ReadablePathBuf {
    fn read_part_as_vec(&self) -> Vec<String> {
        match std::fs::read_to_string(&self.0) {
            Ok(res) => res.trim().split("\n").map(str::to_string).collect(),
            Err(error) => panic!(
                "Failed to read problem data at path: {:?}\nError was: {}",
                self.0, error
            ),
        }
    }
}

fn option_string<T: Display>(opt: &Option<T>) -> String {
    match opt {
        Some(val) => format!("{}", val),
        None => "Not implemented.".to_string(),
    }
}

pub trait Solution {
    /// Should return the day number -- used to read input files.
    fn day_number() -> i32;

    /// The part 1 solution to be implemented.
    fn solution_impl_1(_data: Vec<String>) -> Option<i64>;

    /// The part 2 solution to be implemented.
    fn solution_impl_2(_data: Vec<String>) -> Option<i64>;

    fn get_func(part: Part) -> fn(Vec<String>) -> Option<i64> {
        match part {
            Part(1) => Self::solution_impl_1,
            Part(2) => Self::solution_impl_2,
            Part(p) => panic!("Solution does not exist for part {}!", p),
        }
    }

    fn sample(part: Part) -> Option<i64> {
        Self::get_func(part)(
            SolutionMode::SAMPLE
                .to_path(Self::day_number())
                .read_part_as_vec(),
        )
    }

    fn input(part: Part) -> Option<i64> {
        Self::get_func(part)(
            SolutionMode::INPUT
                .to_path(Self::day_number())
                .read_part_as_vec(),
        )
    }

    fn run() {
        println!(
            "{}==> Day {}{}",
            Colour::BOLD.to_string(),
            Self::day_number(),
            Colour::RESET.to_string()
        );

        let part1_sample = Self::sample(Part(1));
        let part1_input = Self::input(Part(1));

        println!("===> Sample 1\n{}", option_string(&part1_sample));
        println!("===> Input 1\n{}", option_string(&part1_input));

        // Only run the later parts if the initial parts were implemented.
        if part1_sample.is_some() {
            println!("===> Sample 2\n{}", option_string(&Self::sample(Part(2))));
        }

        if part1_input.is_some() {
            println!("===> Input 2\n{}", option_string(&Self::input(Part(2))));
        }

        println!();
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct XY<T: Eq> {
    pub x: T,
    pub y: T,
}

impl<T: Eq> XY<T> {
    pub fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}

impl<T: Display + Eq> Display for XY<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: FromStr + Copy + Eq> From<&str> for XY<T> {
    fn from(s: &str) -> Self {
        let splits = s
            .split(',')
            .map(str::trim)
            .flat_map(str::parse::<T>)
            .collect::<Vec<T>>();

        assert_eq!(splits.len(), 2, "Cannot parse XY from \"{}\"", s);
        match splits[..] {
            [x, y] => XY { x, y },
            _ => unreachable!(),
        }
    }
}
