#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;
use whiteread::parse_line;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use maplit::*;

use competitive::format::*;
use utils::debug;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn get_index(c: char) -> Option<usize> {
    match c {
        'a' => Some(1),
        't' => Some(2),
        'c' => Some(3),
        'o' => Some(4),
        'd' => Some(5),
        'e' => Some(6),
        'r' => Some(7),
        _ => None,
    }
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        n: usize,
        s: Chars,
    }

    /*
    DPっぽい -> わからん
    -> 耳DPというものらしい
    */

    // dp[pos in s][atcoderのn文字目]
    let mut dp = vec![vec![0; 8]; n + 1];
    dp[0][0] = 1;
    for pos in 0..n {
        let c = s[pos];
        for j in 0..8 {
            dp[pos + 1][j] = dp[pos][j];
            if let Some(idx) = get_index(c) {
                if idx - 1 == j {
                    dp[pos + 1][j + 1] += dp[pos][j]
                }
            }
        }

        for j in 0..8 {
            dp[pos + 1][j] %= MOD;
        }
    }

    debug!(dp);
    ""
}

fn main() {
    println!("{}", solve().format());
}

pub mod utils {
    #[allow(unused_macros)]
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }
    pub(crate) use debug;
}
