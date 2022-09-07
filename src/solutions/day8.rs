use std::collections::{BTreeSet, HashMap};

use crate::utils::Solution;

pub struct Day8;

#[derive(Debug)]
struct Splits {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

fn to_set(s: &str) -> BTreeSet<char> {
    s.chars().collect()
}

impl Splits {
    fn from_input(s: &str) -> Self {
        let mut splits = s.split(" | ").map(str::to_string).collect::<Vec<String>>();
        assert_eq!(splits.len(), 2, "Input has wrong number of splits.");

        let outputs = splits
            .pop()
            .unwrap()
            .split(' ')
            .map(str::to_string)
            .collect::<Vec<String>>();

        let inputs = splits
            .pop()
            .unwrap()
            .split(' ')
            .map(str::to_string)
            .collect::<Vec<String>>();

        assert_eq!(inputs.len(), 10, "Input not size 10.");

        Splits { inputs, outputs }
    }

    fn derive_mappings(&self) -> HashMap<BTreeSet<char>, i64> {
        // Full decode
        //
        //  aaaa
        // b    c
        // b    c
        //  dddd
        // e    f
        // e    f
        //  gggg
        //
        // Easy numbers:
        //   - 1: Always length 2.
        //   - 4: Always length 4.
        //   - 7: Always length 3.
        //   - 8: Always length 7.
        //
        // Inferences:
        //   - Length 6 items:
        //     - 8 - 7: Gives bdeg. In length 6s, only 6 has this ==> derives 6.
        //     - 4 - 1: Gives bd. In leftover 6s, only 9 has this ==> derives 9.
        //     - By default, only remaining 6 is 0 ==> derives 0.
        //   - Length 5 items:
        //     - Using bd again, in length 5s, only 5 has this ==> derives 5.
        //     - In leftover 5s, only 3 contains 1 ==> derives 3.
        //     - Only left is 2 ==> derives 2.
        let mut res: HashMap<BTreeSet<char>, i64> = HashMap::new();
        let mut res_rev: HashMap<i64, BTreeSet<char>> = HashMap::new();

        // Get easy ones.
        for segments in &self.inputs {
            match segments.len() {
                2 => {
                    res.insert(to_set(segments), 1);
                    res_rev.insert(1, to_set(segments));
                }
                4 => {
                    res.insert(to_set(segments), 4);
                    res_rev.insert(4, to_set(segments));
                }
                3 => {
                    res.insert(to_set(segments), 7);
                    res_rev.insert(7, to_set(segments));
                }
                7 => {
                    res.insert(to_set(segments), 8);
                    res_rev.insert(8, to_set(segments));
                }
                _ => (),
            }
        }

        for num in [1, 4, 7, 8] {
            assert!(
                res_rev.contains_key(&num),
                "Failed to acquire easy category value {}.",
                num
            );
        }

        // Infer 6.
        let bdeg = res_rev
            .get(&8)
            .unwrap()
            .difference(res_rev.get(&7).unwrap())
            .cloned()
            .collect();

        for segments in &self.inputs {
            if segments.len() != 6 {
                continue;
            }

            if to_set(segments).is_superset(&bdeg) {
                res.insert(to_set(segments), 6);
                res_rev.insert(6, to_set(segments));
                break;
            }
        }

        assert!(
            res_rev.contains_key(&6),
            "Failed to acquire derived category value 6.",
        );

        // Infer 9.
        let bd = res_rev
            .get(&4)
            .unwrap()
            .difference(res_rev.get(&1).unwrap())
            .cloned()
            .collect();

        for segments in &self.inputs {
            if segments.len() != 6 || res.contains_key(&to_set(segments)) {
                continue;
            }

            if to_set(segments).is_superset(&bd) {
                res.insert(to_set(segments), 9);
                res_rev.insert(9, to_set(segments));
                break;
            }
        }

        assert!(
            res_rev.contains_key(&9),
            "Failed to acquire derived category value 9.",
        );

        // Infer 0.
        for segments in &self.inputs {
            if segments.len() != 6 || res.contains_key(&to_set(segments)) {
                continue;
            }

            res.insert(to_set(segments), 0);
            res_rev.insert(0, to_set(segments));
            break;
        }

        assert!(
            res_rev.contains_key(&0),
            "Failed to acquire derived category value 0.",
        );

        // Infer 5.
        for segments in &self.inputs {
            if segments.len() != 5 {
                continue;
            }

            if to_set(segments).is_superset(&bd) {
                res.insert(to_set(segments), 5);
                res_rev.insert(5, to_set(segments));
                break;
            }
        }

        assert!(
            res_rev.contains_key(&5),
            "Failed to acquire derived category value 5.",
        );

        // Infer 3.
        for segments in &self.inputs {
            if segments.len() != 5 || res.contains_key(&to_set(segments)) {
                continue;
            }

            if to_set(segments).is_superset(res_rev.get(&1).unwrap()) {
                res.insert(to_set(segments), 3);
                res_rev.insert(3, to_set(segments));
                break;
            }
        }

        assert!(
            res_rev.contains_key(&3),
            "Failed to acquire derived category value 3.",
        );

        // Infer 2.
        for segments in &self.inputs {
            if segments.len() != 5 || res.contains_key(&to_set(segments)) {
                continue;
            }

            res.insert(to_set(segments), 2);
            res_rev.insert(2, to_set(segments));
            break;
        }

        assert!(
            res_rev.contains_key(&3),
            "Failed to acquire derived category value 2.",
        );

        res
    }
}

impl Solution for Day8 {
    fn day_number() -> i32 {
        8
    }

    fn solution_impl_1(data: Vec<String>) -> Option<i64> {
        let all_splits = data
            .iter()
            .map(String::as_str)
            .map(Splits::from_input)
            .collect::<Vec<Splits>>();

        all_splits
            .iter()
            .map(|splits| {
                splits
                    .outputs
                    .iter()
                    .filter(|output| matches!(output.len(), 2 | 3 | 4 | 7))
                    .count() as i64
            })
            .sum::<i64>()
            .into()
    }

    fn solution_impl_2(data: Vec<String>) -> Option<i64> {
        let all_splits = data
            .iter()
            .map(String::as_str)
            .map(Splits::from_input)
            .collect::<Vec<Splits>>();

        let mut sum: i64 = 0;
        for splits in all_splits {
            let mut cur_sum: i64 = 0;
            let mappings = splits.derive_mappings();
            for output in &splits.outputs {
                cur_sum *= 10;
                cur_sum += mappings.get(&to_set(output.as_str())).unwrap();
            }

            sum += cur_sum;
        }

        sum.into()
    }
}
