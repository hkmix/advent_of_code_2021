use crate::utils::Solution;

pub struct Day3;

fn extractor(vec: &Vec<i32>, rule: fn(i32) -> bool) -> i64 {
    let mut result: i64 = 0;
    for num in vec {
        result <<= 1;
        if rule(*num) {
            result += 1;
        }
    }

    result
}

fn to_i64(s: &str) -> i64 {
    let mut result: i64 = 0;
    for c in s.chars() {
        result <<= 1;
        match c {
            '0' => (),
            '1' => result += 1,
            ch => panic!("Cannot handle bad bit {}", ch),
        }
    }

    result
}

impl Solution for Day3 {
    fn day_number() -> i32 {
        3
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        assert!(data.len() > 0, "Input data has size 0, cannot proceed");

        let binary_length = data.iter().map(|s| s.len()).max().unwrap();
        let mut counts = vec![0; binary_length];

        for line in data {
            for (idx, c) in line.chars().enumerate() {
                let offset = binary_length - line.len();
                match c {
                    '0' => counts[idx + offset] -= 1,
                    '1' => counts[idx + offset] += 1,
                    ch => panic!("Bad binary character {}", ch),
                }
            }
        }

        let gamma = extractor(&counts, |num| num > 0);
        let epsilon = extractor(&counts, |num| num < 0);

        Some(gamma * epsilon)
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        // Oxygen: keep most common bit, 1 if equal.
        // CO2: keep least common bit, 0 if equal.
        assert!(data.len() > 0, "Input data has size 0, cannot proceed");

        let binary_length = data.iter().map(|s| s.len()).max().unwrap();
        let numbers: Vec<i64> = data.iter().map(|s| to_i64(s)).collect();

        // Oxygen.
        let mut mask = 1i64 << (binary_length - 1);
        let mut oxygen = numbers.clone();
        while oxygen.len() > 1 {
            let (ones, zeroes): (Vec<i64>, Vec<i64>) =
                oxygen.iter().partition(|val| *val & mask > 0);
            if ones.len() >= zeroes.len() {
                oxygen = ones;
            } else {
                oxygen = zeroes;
            }
            mask >>= 1;
        }

        assert_eq!(oxygen.len(), 1, "Filtered out to bad list: {:?}", oxygen);

        // CO2.
        let mut mask = 1i64 << (binary_length - 1);
        let mut co2 = numbers.clone();
        while co2.len() > 1 {
            let (ones, zeroes): (Vec<i64>, Vec<i64>) = co2.iter().partition(|val| *val & mask > 0);
            if ones.len() >= zeroes.len() {
                co2 = zeroes;
            } else {
                co2 = ones;
            }
            mask >>= 1;
        }

        assert_eq!(co2.len(), 1, "Filtered out to bad list: {:?}", oxygen);

        Some(oxygen[0] * co2[0])
    }
}
