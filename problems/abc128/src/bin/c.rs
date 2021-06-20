#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::{input, fastout};
use proconio::marker::*;
use whiteread::parse_line;
use std::collections::*;
use num::*;
use num_traits::*;
use superslice::*;
use std::ops::*;

use itertools::Itertools;
use itertools::iproduct;
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

    let (n, m): (usize, usize) = parse_line().unwrap();

    let mut switchs = vec![vec![]; m];
    for _ in 0..n {
        let v: Vec<usize> = parse_line().unwrap();
    }

    let p: Vec<usize> = parse_line().unwrap();

    debug!(p);
    
    ""
}

fn main() {
    println!("{}", solve().format());
}
