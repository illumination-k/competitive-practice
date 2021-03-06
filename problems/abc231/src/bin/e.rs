#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*, source::auto::AutoSource};
use std::{
    collections::*,
    io::{BufRead, BufReader},
    ops::*, cmp::Reverse,
};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn get_coin(val: usize, coins: &[usize]) -> (usize, Option<usize>) {
    let i = coins.lower_bound(&val);

    if i >= coins.len() {
        (coins[i], None)
    } else {
        if val == coins[i] {
            (coins[i], None)
        } else {    
            (coins[i], Some(coins[i+1]))
        }
    }
}

fn exact_coin_num(mut val: usize, coins: &[usize]) -> usize {
    let mut sum = 0;
    let mut lower_index = coins.upper_bound(&val);
    while lower_index != 0 && val != 0 {
        debug!(lower_index, val);
        let coin_num = val / coins[lower_index];
        val -= coins[lower_index] * coin_num;
        sum += coin_num;
        lower_index = coins.upper_bound(&val);
    }
    sum
}


#[fastout]
fn run<R: BufRead>(mut source: AutoSource<R>) -> impl AtCoderFormat {
    input! {
        from &mut source,
        n: usize, x: usize,
        coins: [usize; n]
    }

    let mut bq: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    0
}

fn main() {
    println!(
        "{}",
        run(AutoSource::new(BufReader::new(std::io::stdin()))).format()
    );
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    use competitive::test_utility::*;

    #[test]
    fn test_get_exact_num() {
        let coins = vec![2, 4, 8, 16];
        dbg!(exact_coin_num(12, &coins));
    }
}

pub mod utils {
    use num_traits::PrimInt;
    pub fn ceil<T>(lhs: T, rhs: T) -> T 
        where
            T: PrimInt
    {
        let one = rhs / rhs;
        (lhs + rhs - one) / rhs
    }
    
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }

    macro_rules! chmin {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_min = min!($($cmps),+);
            if $base > cmp_min {
                $base = cmp_min;
                true;
            } else {
                false;
            }
        }};
    }

    macro_rules! chmax {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_max = max!($($cmps),+);
            if $base < cmp_max {
                $base = cmp_max;
                true;
            } else {
                false;
            }
        }};
    }

    macro_rules! min {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::min($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::min($a, min!($($rest),+))
        }};
    }

    macro_rules! max {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::max($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::max($a, max!($($rest),+))
        }};
    }

    use std::process::Output;

    pub(crate) use chmax;
    pub(crate) use chmin;
    pub(crate) use debug;
    pub(crate) use max;
    pub(crate) use min;
}
