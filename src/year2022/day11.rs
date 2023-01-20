use crate::challenge_result::{ChallengeResult, Solution};
use lazy_static::lazy_static;
use regex::Regex;

type MonkeyItem = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Mul(MonkeyItem),
    Add(MonkeyItem),
    Div(MonkeyItem),
    Mod(MonkeyItem),
    Square,
}

impl Operation {
    fn apply(&self, value: MonkeyItem) -> MonkeyItem {
        match self {
            Operation::Mul(x) => value * x,
            Operation::Add(x) => value + x,
            Operation::Div(x) => value / x,
            Operation::Mod(x) => value % x,
            Operation::Square => value * value,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    operation: Operation,
    test: MonkeyTest,
}

#[derive(Debug, Clone)]
struct MonkeyTest {
    divisible: MonkeyItem,
    recipient_true: usize,
    recipient_false: usize,
}

fn round(
    monkeys: &[Monkey],
    operation: Operation,
    monkey_items: &mut Vec<Vec<MonkeyItem>>,
    inspections: &mut [usize],
) {
    for (idx, (monkey, inspection)) in monkeys.iter().zip(inspections.iter_mut()).enumerate() {
        *inspection += monkey_items[idx].len();

        while let Some(item) = monkey_items[idx].pop() {
            let next_item = operation.apply(monkey.operation.apply(item));

            let recipient_idx = if next_item % monkey.test.divisible == 0 {
                monkey.test.recipient_true
            } else {
                monkey.test.recipient_false
            };

            monkey_items[recipient_idx].push(next_item);
        }
    }
}

fn rounds(
    monkeys: &[Monkey],
    items: &Vec<Vec<MonkeyItem>>,
    operation: Operation,
    num: usize,
) -> usize {
    let mut items = items.clone();
    let mut inspections: Vec<usize> = monkeys.iter().map(|_| 0).collect();

    for _ in 0..num {
        round(&monkeys, operation, &mut items, &mut inspections);
    }

    inspections.sort_unstable();
    inspections.reverse();

    inspections[0] * inspections[1]
}

pub fn run(input: &str) -> ChallengeResult {
    let (monkeys, items) = parse_monkeys(input);

    let lsm = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .reduce(lcm)
        .unwrap();

    let part1 = rounds(&monkeys, &items, Operation::Div(3), 20);
    let part2 = rounds(&monkeys, &items, Operation::Mod(lsm), 10000);

    Ok(Solution::from(part1, part2))
}

fn lcm(first: MonkeyItem, second: MonkeyItem) -> MonkeyItem {
    first * second / gcd(first, second)
}

fn gcd(first: MonkeyItem, second: MonkeyItem) -> MonkeyItem {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn parse_monkeys(s: &str) -> (Vec<Monkey>, Vec<Vec<MonkeyItem>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
                Monkey\ \d+:\s+
                    Starting\ items:\ ((?:\d+,\s*)*\d+)\s+
                    Operation:\ new\ =\ old\ ([*+])\ (\d+|old)\s+
                    Test:\ divisible\ by\ (\d+)\s+
                        If\ true:\ throw\ to\ monkey\ (\d+)\s+
                        If\ false:\ throw\ to\ monkey\ (\d+)\s*
            "
        )
        .unwrap();
    }

    RE.captures_iter(s)
        .map(|cap| {
            let items: Vec<MonkeyItem> = cap
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|v| v.parse::<MonkeyItem>().unwrap())
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

            (
                Monkey {
                    operation,
                    test: MonkeyTest {
                        divisible,
                        recipient_true,
                        recipient_false,
                    },
                },
                items,
            )
        })
        .unzip()
}
