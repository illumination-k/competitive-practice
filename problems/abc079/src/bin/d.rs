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

use competitive_internal_mod::format::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;
// vecから最小値を出す
fn cost_sum(perm: &Vec<&usize>, c: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    for i in 0..perm.len() - 1 {
        sum += c[*perm[i]][*perm[i + 1]]
    }
    sum += c[*perm[perm.len() - 1]][1];
    sum
}

// 各数字から1に変換する方法の最小値を求める
fn get_min_path(c: &Vec<Vec<usize>>) -> Vec<usize> {
    let vals = vec![0, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut min_paths = vec![UINF; 10];
    min_paths[1] = 0;
    for k in 1..=8 {
        for perm in vals.iter().permutations(k) {
            // debug!(perm);
            // 0 -> 9 -> 2 -> 1のとき c[0][9] + c[9][2] + c[2][1]
            let sum = cost_sum(&perm, c);
            let idx = *perm[0];
            min_paths[idx] = std::cmp::min(sum, min_paths[idx]);
        }
    }

    min_paths
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        h: usize, w: usize,
        c: [[usize; 10]; 10],
        a: [[isize; w]; h]
    }

    let min_paths = get_min_path(&c);

    debug!(min_paths);
    let mut ans = 0;
    for x in 0..w {
        for y in 0..h {
            if a[y][x] == -1 {
                continue;
            }

            ans += min_paths[a[y][x] as usize];
        }
    }
    ans
}

fn main() {
    println!("{}", solve().format());
}

mod competitive_internal_mod {
    pub mod format {
        use std::vec::Vec;

        /// Trait of format for atcoder
        ///    
        /// bool -> Yes or No  
        /// vec![a, b ,c] -> "a\nb\nc"  
        /// vec![vec![0, 1], vec![1, 0]] -> "0 1\n1 0"  
        pub trait AtCoderFormat {
            fn format(&self) -> String;
        }

        macro_rules! impl_format {
            ($t: ty) => {
                impl AtCoderFormat for $t {
                    fn format(&self) -> String {
                        self.to_string()
                    }
                }

                impl AtCoderFormat for Vec<$t> {
                    fn format(&self) -> String {
                        self.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    }
                }

                impl AtCoderFormat for Vec<Vec<$t>> {
                    fn format(&self) -> String {
                        self.iter()
                            .map(|x| {
                                x.iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            })
                            .collect::<Vec<String>>()
                            .join("\n")
                    }
                }
            };
        }

        impl_format!(usize);
        impl_format!(u128);
        impl_format!(u64);
        impl_format!(u32);
        impl_format!(u16);
        impl_format!(u8);
        impl_format!(isize);
        impl_format!(i128);
        impl_format!(i64);
        impl_format!(i32);
        impl_format!(i16);
        impl_format!(i8);
        impl_format!(f32);
        impl_format!(f64);
        impl_format!(&str);
        impl_format!(String);

        impl AtCoderFormat for char {
            fn format(&self) -> String {
                self.to_string()
            }
        }

        impl AtCoderFormat for Vec<char> {
            fn format(&self) -> String {
                self.iter().collect::<String>()
            }
        }

        impl AtCoderFormat for Vec<Vec<char>> {
            fn format(&self) -> String {
                self.iter()
                    .map(|v| v.format())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }

        impl AtCoderFormat for bool {
            fn format(&self) -> String {
                if self == &true {
                    "Yes".to_string()
                } else {
                    "No".to_string()
                }
            }
        }

        impl AtCoderFormat for Vec<bool> {
            fn format(&self) -> String {
                self.iter()
                    .map(|x| x.format())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }
    }
}
