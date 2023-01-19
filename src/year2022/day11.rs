use lazy_static::lazy_static;
use regex::Regex;
use crate::challenge_result::{ChallengeResult, Solution};

type MonkeyItem = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    test: MonkeyTest,
    inspections: usize,
}

#[derive(Debug, Clone)]
struct MonkeyTest {
    divisible: MonkeyItem,
    recipient_true: usize,
    recipient_false: usize,
}

fn round(monkeys: &mut [Monkey]) {
    for idx in 0..monkeys.len() {
        monkeys[idx].inspections += monkeys[idx].items.len();
        let items: Vec<MonkeyItem> = monkeys[idx].items.drain(..).collect();
        let operation = monkeys[idx].operation;
        let test = monkeys[idx].test.clone();

        for item in items {
            let next_item = operation.apply(item) / 3;

            let recipient_idx = if next_item % test.divisible == 0 {
                test.recipient_true
            } else {
                test.recipient_false
            };

            monkeys[recipient_idx].items.push(next_item);
        }
    }
}

fn print_items(monkeys: &[Monkey]) {
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", idx, monkey.items);
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..20 {
        round(&mut monkeys);
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.inspections);
    monkeys.reverse();

    let part1 = monkeys[0].inspections * monkeys[1].inspections;

    Ok(Solution::from(part1, 0))
}

fn parse_monkeys(s: &str) -> Vec<Monkey> {
    lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                Monkey\ \d+:\s+
                    Starting\ items:\ ((?:\d+,\s*)*\d+)\s+
                    Operation:\ new\ =\ old\ ([*+])\ (\d+|old)\s+
                    Test:\ divisible\ by\ (\d+)\s+
                        If\ true:\ throw\ to\ monkey\ (\d+)\s+
                        If\ false:\ throw\ to\ monkey\ (\d+)\s*
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

        let divisible: MonkeyItem = cap.get(4).unwrap().as_str().parse().unwrap();
        let recipient_true: usize = cap.get(5).unwrap().as_str().parse().unwrap();
        let recipient_false: usize = cap.get(6).unwrap().as_str().parse().unwrap();

        Monkey { items, operation, test: MonkeyTest { divisible, recipient_true, recipient_false }, inspections: 0 }
    }).collect()
}