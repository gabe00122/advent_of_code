use num::{PrimInt};

pub fn lcm<T>(first: T, second: T) -> T
where
    T: PrimInt,
{
    first * second / gcd(first, second)
}

pub fn gcd<T>(first: T, second: T) -> T
where
    T: PrimInt,
{
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res.is_zero() {
            return min;
        }

        max = min;
        min = res;
    }
}