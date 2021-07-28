#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{input, fastout};
use proconio::marker::*;
use whiteread::parse_line;
use std::collections::*;
use num::*;
use num_traits::*;
use superslice::*;
use std::ops::*;

use itertools::Itertools;
use itertools_num::ItertoolsNum;

fn main() {
    input!{
        h: usize, w: usize,
        s: [Chars; h]
    }

    // n列目の障害物の位置を記録
    let mut col_obs = vec![vec![-1]; w];

    // n行目の障害物の位置を記録
    let mut row_obs = vec![vec![-1]; h];

    for x in 0..w {
        for y in 0..h {
            if s[y][x] == '#' {
                col_obs[x].push(y as isize);
                row_obs[y].push(x as isize);
            }
        }
    }

    (0..w).for_each(|x| col_obs[x].push(h as isize));
    (0..h).for_each(|y| row_obs[y].push(w as isize));
    dbg!(&col_obs);
    dbg!(&row_obs);

    // 二分探索で自分に一番近い位置の障害物の番号を探す

    for x in 0..w {
        println!("--- {} ---", x);
        for y in 0..h {
            if s[y][x] == '#' {
                continue;
            }

            // 横方向で自分に一番近いかつ低い障害物の場所
            let lx = row_obs[y].lower_bound(&(x as isize));
            println!("x: {} y: {}: lx: {}", x, y, row_obs[y][lx]);
        }
    }
}
