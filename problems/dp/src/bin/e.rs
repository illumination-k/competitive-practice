#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use competitive::format::*;
use utils::debug;
const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;
#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {}
    ""
}
fn main() {
    println!("{}", solve().format());
}
#[cfg(test)]
mod test {
    use super::*;
}
pub mod utils {
    #[allow(unused_macros)]
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }
    #[allow(unused_macros)]
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
    #[allow(unused_macros)]
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
    #[allow(unused_macros)]
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
    #[allow(unused_macros)]
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
    #[allow(unused_imports)]
    pub(crate) use chmax;
    #[allow(unused_imports)]
    pub(crate) use chmin;
    #[allow(unused_imports)]
    pub(crate) use debug;
    #[allow(unused_imports)]
    pub(crate) use max;
    #[allow(unused_imports)]
    pub(crate) use min;
}
