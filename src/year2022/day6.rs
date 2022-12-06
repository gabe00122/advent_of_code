use std::collections::BTreeMap;
use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

struct Counter<T : Ord> {
    map: BTreeMap<T, u32>
}

impl <T : Ord> Counter<T> {
    fn new() -> Counter<T> {
        Counter { map: BTreeMap::new() }
    }

    fn insert(&mut self, value: T) {
        self.map.entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    fn remove(&mut self, value: T) {
        if let Some(count) = self.map.get(&value) {
            let count = count - 1;
            if count > 0 {
                self.map.insert(value, count);
            } else {
                self.map.remove(&value);
            }
        }
    }

    fn unique_len(&self) -> usize {
        self.map.len()
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let chars: Vec<char> = input.chars().collect();

    let part1 = find_sequence(&chars[..], 4);
    let part2 = find_sequence(&chars[..], 14);

    Ok(ChallengeSuccess::new(part1.unwrap() as u64, part2.unwrap() as u64))
}

fn find_sequence(chars: &[char], window_size: usize) -> Option<usize> {
    let mut counter = Counter::new();
    for &c in chars.iter().take(window_size) {
        counter.insert(c);
    }

    for (i, (&tail, &head)) in chars.iter().zip(&chars[window_size..]).enumerate() {
        if counter.unique_len() >= window_size {
            return Some(i + window_size);
        }
        counter.insert(head);
        counter.remove(tail);
    }

    None
}
