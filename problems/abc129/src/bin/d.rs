#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::cmp::Reverse;
use std::{collections::*, ops::*};
use superslice::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use competitive::format::*;
use competitive::matrix2d::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[fastout]
fn run() -> impl AtCoderFormat {
    input! {
        h: usize, w: usize,
        s: [String; h]
    }

    let mat = Matrix2D::from_str_slice(&s);

    let mut row_index = vec![vec![0]; h];
    let mut col_index = vec![vec![0]; w];
    
    for y in 0..h {
        for (i, &r) in mat.row(y).enumerate() {
            if r == '#' {
                row_index[y].push(i as i64)
            }
        }
        row_index[y].push(w as i64);
    }
    
    for x in 0..w {
        for (i, c) in mat.col(x).enumerate() {
            if c == '#' {
                col_index[x].push(i as i64)
            }
        }
        col_index[x].push(h as i64);
    }

    debug!(col_index);

    for (x, y) in mat.index_iter() {
        if mat[(x, y)] == '#' {
            continue;
        }

        let r_index = row_index[y].upper_bound(&(x as i64));
        let c_index = col_index[x].upper_bound(&(y as i64));


        println!("------ x: {}, y: {} ------", x, y);
        println!("r_index: {}, rval: {}", r_index, row_index[y][r_index]);
        println!("c_index: {}, cval: {}", c_index, col_index[x][c_index]);

        let row_sum = row_index[y][r_index] - row_index[y][r_index - 1];
        let col_sum = col_index[x][c_index] - col_index[x][c_index - 1];
        println!("sum: {}", row_sum + col_sum - 1);
    }
    0
}

fn main() {
    println!("{}", run().format());
}

#[cfg(test)]
mod test {
    use super::*;
}

pub mod utils {
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

    pub(crate) use chmax;
    pub(crate) use chmin;
    pub(crate) use debug;
    pub(crate) use max;
    pub(crate) use min;
}

