pub fn lcm(first: MonkeyItem, second: MonkeyItem) -> MonkeyItem {
    first * second / gcd(first, second)
}

pub fn gcd(first: MonkeyItem, second: MonkeyItem) -> MonkeyItem {
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