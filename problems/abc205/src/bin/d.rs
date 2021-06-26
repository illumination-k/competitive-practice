#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use rand::seq;
use std::{collections::*, ops::*};
use superslice::*;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;

use competitive_internal_mod::format::*;
use utils::debug;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn count(n: usize, a: Vec<usize>, query: Vec<usize>) -> Vec<usize> {
    let mut store = vec![(0, 0, 0)];
    let mut sequence_num = 0;
    let mut last = a[0];
    for i in 0..n {
        // 連続数を記録
        sequence_num += 1;
        if i == n - 1 {
            store.push((last, a[i], sequence_num))
        } else {
            if a[i] + 1 == a[i + 1] {
                continue;
            } else {
                store.push((last, a[i], sequence_num));
                last = a[i + 1];
            }
        }
    }

    let mut ans = vec![];

    for &k in query.iter() {
        let mut f_i = store.lower_bound_by(|a| a.0.cmp(&k));
        // let b_i = store.lower_bound_by(|x| x.1.cmp(&k));

        if f_i >= store.len() {
            ans.push(k + n);
            continue;
        }
        if store[f_i].0 > k {
            f_i -= 1;
        }
        let nk = k + store[f_i].2;
        // ans.push(nk);

        let mut nf_i = store.lower_bound_by(|x| x.0.cmp(&nk));
        debug!(nk, nf_i);
        if nf_i >= store.len() {
            ans.push(k + n);
            continue;
        }

        if store[nf_i].0 > k {
            nf_i -= 1;
        }

        ans.push(k + store[nf_i].2);
        ans.extend(iter)
    }

    debug!(store);
    ans
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        n: usize, q: usize,
        pa: [usize; n],
        query: [usize; q]
    }
    let mut a = pa.into_iter().unique().collect_vec();
    a.sort();

    /*
    連続した部分を圧縮する。
    */

    count(n, a, query)
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

#[cfg(test)]
mod test {
    use super::*;
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
