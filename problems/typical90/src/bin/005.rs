#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::*;
use std::ops::*;
use superslice::*;
use whiteread::parse_line;

use itertools::iproduct;
use itertools::Itertools;
use itertools_num::ItertoolsNum;

use competitive::format::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    const MOD: usize = 1_000_000_007;
    const UINF: usize = std::usize::MAX;
    const IINF: isize = std::isize::MAX;

    input! {
        n: usize, b: usize, k: usize,
        c: [usize; k]
    }

    /*
    1. 桁DP
    https://torus711.hatenablog.com/entry/20150423/1429794075
    smallerは1 <= ci <= 9なので必要ない
    2. 行列累乗
    */
    let mut dp = vec![vec![0; b]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..b {
            // 動ける範囲
            for &ci in c.iter() {
                let next = (10 * j + ci) % b;
                dp[i + 1][next] += dp[i][j];
                dp[i + 1][next] %= MOD;
            }
        }
    }
    debug!(dp);
    dp[n][0]
}

fn main() {
    println!("{}", solve().format());
}
