use std::{fmt::Debug, hash::Hash};

use gxhash::{HashMap, HashMapExt};

trait Number: Eq + Hash + Copy {
    fn new(n: u64) -> Self;

    fn is_zero(&self) -> bool;

    fn even_length(&self) -> bool;

    fn split(&mut self) -> Self;

    fn multiply(&mut self, n: u64);
}

// const MAX_DIGITS: usize = 12;

// struct Number {
//     digits: [u8; MAX_DIGITS],
// }
// impl Number {
//     // fn split(&mut self) ->
//     fn is_zero(&self) -> bool {
//         self.digits.iter().all(|i| *i == 0)
//     }
//     fn even_length(&self) -> bool {
//         self.digits
//             .iter()
//             .rposition(|i| *i != 0)
//             .map(|p| p % 2 == 1)
//             .unwrap()
//     }

//     // fn split(&)
// }

// impl Debug
//

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Simple {
    i: u64,
}

impl Debug for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.i)
    }
}

impl Number for Simple {
    fn new(i: u64) -> Self {
        Self { i }
    }

    fn is_zero(&self) -> bool {
        self.i == 0
    }

    fn even_length(&self) -> bool {
        let s = format!("{}", self.i);
        let len = s.len();
        len % 2 == 0
    }

    fn split(&mut self) -> Self {
        let s = format!("{}", self.i);
        let middle = s.len() / 2;
        let left: u64 = s[..middle].parse().unwrap();
        let right: u64 = s[middle..].parse().unwrap();
        self.i = left;
        Self::new(right)
    }

    fn multiply(&mut self, n: u64) {
        self.i *= n;
    }
}

fn blink_len<T: Number + Debug>(x: &mut T, times: u8, cache: &mut HashMap<(T, u8), u64>) -> u64 {
    if times == 0 {
        return 1;
    } else if x.is_zero() {
        *x = T::new(1);
        return blink_len(x, times - 1, cache);
    }
    let key = (*x, times);
    let entry = cache.get(&key);
    if let Some(e) = entry {
        return *e;
    }
    let r = if x.even_length() {
        let mut other: T = x.split();
        blink_len(x, times - 1, cache) + blink_len(&mut other, times - 1, cache)
    } else {
        x.multiply(2024);
        blink_len(x, times - 1, cache)
    };
    cache.insert(key, r);
    r
}

pub fn part1(input: &str) -> u64 {
    let mut cache = HashMap::new();
    input
        .split(' ')
        .map(|s| s.parse().unwrap())
        .map(|n| {
            let mut num = Simple::new(n);
            blink_len(&mut num, 75, &mut cache)
        })
        .sum()
}
