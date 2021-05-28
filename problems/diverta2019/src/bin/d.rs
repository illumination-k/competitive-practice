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

/*
N % m < m

if m < sqrt(N)

N / m > sqrt(N)
N % m < m < sqrt(N)
ので矛盾。なので

m >= sqrt(N)を調べる必要がある。計算量は改善しない。

mod側に周期があるはず
div側には？

8の場合
3 4 5 6 7 8
2 2 1 1 1 1
2 0 3 2 1 0

m >= sqrt(N)なので
n / m <= sqrt(N)
n % m <= m

n / m = xとすると
n = x * m + x
n = x * (m + 1)

if n % x == 0 {
    m = n / x - 1;
    if m >= sqrt(n) {}
}

x <= sqrt(N)なので、すべてのxについて調べればいい？
*/

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn all_search(n: usize) -> usize {
    let lower = ((n as f64).sqrt() - 1.) as usize;
    let mut ret = 0;
    for i in lower..=n {
        if n / i == n % i {
            debug!(i);
            ret += i;
        }
    }
    ret
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        n: usize
    }

    let mut ans = 0;

    let upper = ((n as f64).sqrt() + 1.) as usize;

    for x in 1..=upper {
        if n % x == 0 {
            let m = n / x - 1;
            if m == 0 {
                continue;
            }
            if n / m == n % m {
                ans += m;
            }
        }
    }

    ans
}

fn main() {
    println!("{}", solve().format());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let ans = all_search(30);
        println!("{}", ans)
    }
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
        impl_format!(char);

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
