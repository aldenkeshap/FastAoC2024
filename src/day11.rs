use std::{fmt::Debug, hash::Hash};

use gxhash::{HashMap, HashMapExt};

trait Number: Eq + Hash + Copy {
    fn new(n: u64) -> Self;

    fn flip_zero_one(&mut self) -> bool;

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
    fn is_zero(&self) -> bool {
        self.digits.iter().all(|i| *i == 0)
    }
}

impl Number for Digits {
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

    fn flip_zero_one(&mut self) -> bool {
        let zero = self.is_zero();
        if zero {
            self.digits[0] = 1;
        }
        zero
    }
}

fn blink_len_old<T: Number + Debug>(
    x: &mut T,
    times: u8,
    cache: &mut HashMap<(T, u8), u64>,
) -> u64 {
    if times == 0 {
        // println!("ENTRY {:?}", x);
        return 1;
    } else if x.flip_zero_one() {
        return blink_len_old(x, times - 1, cache);
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
        blink_len_old(x, times - 1, cache) + blink_len_old(&mut other, times - 1, cache)
    } else {
        x.multiply(2024);
        blink_len_old(x, times - 1, cache)
    };
    cache.insert(key, r);
    r
}

// fn blink_len<T: Number + Debug>(
//     x: &mut T,
//     mut times: u8,
//     cache: &mut HashMap<(T, u8), u64>,
// ) -> u64 {
//     let mut additional = [0; 76];
//     let mut xs = [T::new(0); 76];
//     let times_start = times;
//     while times > 0 {
//         xs[times as usize] = *x;
//         if x.flip_zero_one() {
//         } else {
//             let key = (*x, times);
//             let entry = cache.get(&key).copied();
//             if let Some(e) = entry {
//                 let mut sum = e;
//                 for t in (times as usize)..=(times_start as usize) {
//                     sum += additional[t];
//                     let k = (xs[t], t as u8);
//                     cache.insert(k, sum);
//                 }
//                 return sum;
//             }

//             if x.even_length() {
//                 let mut other: T = x.split();
//                 let other_len = blink_len(&mut other, times - 1, cache);
//                 additional[times as usize] = other_len;
//             } else {
//                 x.multiply(2024);
//             };
//         }

//         times -= 1;
//     }
//     let mut sum = 1;
//     for t in 1..=(times_start as usize) {
//         sum += additional[t];
//         let k = (xs[t], t as u8);
//         cache.insert(k, sum);
//     }
//     additional.iter().sum::<u64>()
// }

pub fn part1(input: &str) -> u64 {
    let mut cache = HashMap::new();
    input
        .split(' ')
        .map(|s| s.parse().unwrap())
        .map(|n| {
            let mut num = Digits::new(n);

            blink_len_old(&mut num, 75, &mut cache)
        })
        .sum()
}
