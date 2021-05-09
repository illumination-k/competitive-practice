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
