use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use crate::challenge_result::{ChallengeResult, Solution};

type MonkeyItem = i32;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Mul(MonkeyItem),
    Add(MonkeyItem),
    Square,
}

impl Operation {
    fn apply(&self, value: MonkeyItem) -> MonkeyItem {
        match self {
            Operation::Mul(x) => x * value,
            Operation::Add(x) => x + value,
            Operation::Square => value * value,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<MonkeyItem>,
    operation: Operation,
    test: MonkeyItem,
    test_true: usize,
    test_false: usize,
}

fn parse_monkeys(s: &str) -> Vec<Monkey> {
    lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                Monkey\ \d+:\s+
                    Starting\ items:\ ((?:\d+,\s*)+\d+)\s+
                    Operation:\ new\ =\ old\ ([*+])\ (\d+|old)\s+
                    Test:\ divisible\ by\ (\d+)\s+
                        If\ true:\ throw\ to\ monkey\ (\d+)\s+
                        If\ false:\ throw\ to\ monkey\ (\d+)\s+
            ").unwrap();
    }

    RE.captures_iter(s).map(|cap| {
        let items: Vec<MonkeyItem> = cap.get(1).unwrap().as_str()
            .split(", ").map(|v| v.parse::<MonkeyItem>().unwrap())
            .collect();

        let operator_raw = cap.get(2).unwrap().as_str();
        let operation_source_raw = cap.get(3).unwrap().as_str();

        let operation = if operation_source_raw == "old" {
            Operation::Square
        } else {
            let operation_source = operation_source_raw.parse::<MonkeyItem>().unwrap();

            if operator_raw == "*" {
                Operation::Mul(operation_source)
            } else {
                Operation::Add(operation_source)
            }
        };

        let test: MonkeyItem = cap.get(4).unwrap().as_str().parse().unwrap();
        let test_true: usize = cap.get(5).unwrap().as_str().parse().unwrap();
        let test_false: usize = cap.get(6).unwrap().as_str().parse().unwrap();

        Monkey { items, operation, test, test_true, test_false }
    }).collect()
}

pub fn run(input: &str) -> ChallengeResult {
    let monkeys = parse_monkeys(input);

    println!("{:?}", monkeys);

    Ok(Solution::from(0, 0))
}
