use std::{fmt::Debug, hash::Hash};

use gxhash::{HashMap, HashMapExt};

trait Number: Eq + Hash + Copy {
    fn new(n: u64) -> Self;

    fn is_zero(&self) -> bool;

    fn even_length(&self) -> bool;

    fn split(&mut self) -> Self;

    fn multiply(&mut self, n: u64);
}

const MAX_DIGITS: usize = 12;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Digits {
    digits: [u8; MAX_DIGITS],
}

impl Digits {
    fn length(&self) -> usize {
        self.digits
            .iter()
            .rposition(|i| *i != 0)
            .map(|l| l + 1)
            .unwrap()
    }
}

impl Number for Digits {
    // fn split(&mut self) ->
    fn is_zero(&self) -> bool {
        self.digits.iter().all(|i| *i == 0)
    }
    fn even_length(&self) -> bool {
        self.length() % 2 == 0
    }

    fn new(mut n: u64) -> Self {
        let mut digits = [0; MAX_DIGITS];
        for d in &mut digits {
            *d = (n % 10) as u8;
            n /= 10;
        }
        Digits { digits }
    }

    fn split(&mut self) -> Self {
        let mut digits = [0; MAX_DIGITS];
        let middle: usize = self.length() / 2;
        // for i in 0..middle {
        //     digits[i] = self.digits[i + middle];
        // }
        digits[..middle].copy_from_slice(&self.digits[middle..(middle + middle)]);
        for i in 0..middle {
            self.digits[i + middle] = 0;
        }
        Digits { digits }
    }

    fn multiply(&mut self, n: u64) {
        let mut carry = 0;
        for d in &mut self.digits {
            carry += (*d as u64) * n;

            *d = (carry % 10) as u8;
            carry /= 10;
        }
    }

    // fn split(&)
}

// #[derive(PartialEq, Eq, Hash, Clone, Copy)]
// struct Simple {
//     i: u64,
// }

// impl Debug for Simple {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.i)
//     }
// }

// impl Number for Simple {
//     fn new(i: u64) -> Self {
//         Self { i }
//     }

//     fn is_zero(&self) -> bool {
//         self.i == 0
//     }

//     fn even_length(&self) -> bool {
//         let s = format!("{}", self.i);
//         let len = s.len();
//         len % 2 == 0
//     }

//     fn split(&mut self) -> Self {
//         let s = format!("{}", self.i);
//         let middle = s.len() / 2;
//         let left: u64 = s[..middle].parse().unwrap();
//         let right: u64 = s[middle..].parse().unwrap();
//         self.i = left;
//         Self::new(right)
//     }

//     fn multiply(&mut self, n: u64) {
//         self.i *= n;
//     }
// }

fn blink_len<T: Number + Debug>(x: &mut T, times: u8, cache: &mut HashMap<(T, u8), u64>) -> u64 {
    if times == 0 {
        // println!("ENTRY {:?}", x);
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

    if times == 1 {
        return if x.even_length() { 2 } else { 1 };
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
            let mut num = Digits::new(n);
            blink_len(&mut num, 75, &mut cache)
        })
        .sum()
}
