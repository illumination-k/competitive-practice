#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;

use competitive_internal_mod::format::*;
use utils::{chmax, debug};

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn stupid_ans(n: usize, t: &[usize]) {
    let mut ans = UINF;
    let ts: HashSet<usize> = (0..n).collect();
    for bit in 0..1 << n {
        let mut v = HashSet::new();
        for i in 0..n {
            if 1 << i & bit != 0 {
                v.insert(i);
            }
        }

        let a1 = v.iter().fold(0, |acc, &i| acc + t[i]);
        let a2 = (&ts - &v).iter().fold(0, |acc, &i| acc + t[i]);
        ans = std::cmp::min(ans, std::cmp::max(a1, a2));
    }

    println!("{}", ans);
}

fn ok(x: usize, t: &[usize]) -> bool {
    let mut dp = vec![vec![false; x + 1]; t.len() + 1];
    dp[0][0] = true;

    for i in 0..t.len() {
        for j in 0..=x {
            dp[i + 1][j] |= dp[i][j];
            if j >= t[i] {
                dp[i + 1][j] |= dp[i][j - t[i]]
            }
        }
    }

    dp[t.len()][x]
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        n: usize,
        mut t: [usize; n]
    }
    t.sort();

    let sum_t: usize = t.iter().sum();
    let mut ans = UINF;
    for x in (0..=sum_t / 2).rev() {
        if ok(x, &t) {
            ans = sum_t - x;
            break;
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
    fn test_stupid_ans() {
        let t = vec![3, 14, 15, 9, 26, 5, 35, 89, 79];
        stupid_ans(9, &t);
    }
}

pub mod utils {
    #[allow(unused_macros)]
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }

    #[allow(unused_macros)]
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

    #[allow(unused_macros)]
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

    #[allow(unused_macros)]
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

    #[allow(unused_macros)]
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

    #[allow(unused_imports)]
    pub(crate) use chmax;

    #[allow(unused_imports)]
    pub(crate) use chmin;

    #[allow(unused_imports)]
    pub(crate) use debug;

    #[allow(unused_imports)]
    pub(crate) use max;

    #[allow(unused_imports)]
    pub(crate) use min;
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
