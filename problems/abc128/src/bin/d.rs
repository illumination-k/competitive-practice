#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use itertools::iproduct;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::marker::*;
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::*;
use std::ops::*;
use superslice::*;
use whiteread::parse_line;

use competitive_internal_mod::format::*;

#[macro_export]
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
        n: usize, k: usize,
        v: [isize; n]
    }

    let vq: VecDeque<isize> = v.into_iter().collect();

    // 何も取らない、が許されているので0が最小
    let mut ans = 0;

    for mut com in (0..k).combinations_with_replacement(2) {
        com.push(k);
        let left = com[0];
        let right = com[1] - com[0];
        let ret = com[2] - com[1];

        debug!(left, right, ret);

        let mut tvq = vq.clone();
        let mut bq = BinaryHeap::new();

        for _ in 0..left {
            if let Some(l) = tvq.pop_front() {
                bq.push(Reverse(l));
            }
        }

        for _ in 0..right {
            if let Some(r) = tvq.pop_back() {
                bq.push(Reverse(r));
            }
        }

        debug!(bq);

        for _ in 0..ret {
            if let Some(Reverse(b)) = bq.pop() {
                if b > 0 {
                    bq.push(Reverse(b));
                    break;
                }
            }
        }

        debug!(bq);

        let sum = bq.iter().map(|&Reverse(x)| x).sum::<isize>();
        ans = std::cmp::max(sum, ans);
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
