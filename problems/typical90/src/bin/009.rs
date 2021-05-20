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

use competitive::format::*;
use competitive::geometry::point::*;
use utils::debug;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        n: usize,
        p: [(f64, f64); n]
    }

    /*
    O(N^3)はさすがに間に合わなさそう
    2点固定して二分探索すればO(N^2log(N))だからセーフ
    どうすればいい？
    -> 偏角なるものを使えばいいらしい
    */

    let points = p.into_iter().map(|x| Point::from_tuple(x)).collect_vec();
    const PI: f64 = std::f64::consts::PI;

    let mut ans = 0.;

    for (i, &o) in points.iter().enumerate() {
        // (declination, index)
        let mut declinations = vec![];
        for (j, &a) in points.iter().enumerate() {
            let t: Point<f64> = o - a;
            let declination = if t.x.atan2(t.y).to_degrees() >= 0. {
                t.x.atan2(t.y).to_degrees()
            } else {
                t.x.atan2(t.y).to_degrees() + 360.0
            };

            declinations.push((declination, j));
        }

        // sort for binary search
        declinations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        debug!(declinations);

        for &(declination, j) in declinations.iter() {
            if i == j {
                continue;
            }
            let _ideal = declination + 180.0;
            let ideal = if _ideal < 360.0 {
                _ideal
            } else {
                _ideal - 360.0
            };
            debug!(ideal);
            let idx = declinations.lower_bound_by(|(f, _)| f.partial_cmp(&ideal).unwrap());
            let k = if idx == declinations.len() {
                idx - 1
            } else {
                idx
            };
            ans = ans.max(
                (declinations[k].0 - declination)
                    .abs()
                    .min(360.0 - (declinations[k].0 - declination).abs()),
            );
            debug!(ans);
        }
    }

    ans
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
