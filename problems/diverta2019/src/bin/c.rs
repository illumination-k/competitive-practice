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
1. A-
2. B-
3. -A
4. -B
5. A-A
6. B-B
7. A-B
8, B-A

Aスタートはいらない
1. B-
2. -A
3. B-A

の3種類のカウントでいい。
- 2 | 3 | 1の順番のやつをまずカウント
- 3を全部つないでから、1,2を考えるもの
*/

fn counter(new_ss: &Vec<Vec<char>>) -> (usize, usize, usize) {
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    for s in new_ss.iter() {
        if s.len() == 0 {
            continue;
        }
        let start = s[0];
        let end = s[s.len() - 1];

        if start == 'B' && end == 'A' {
            count3 += 1;
        } else if start == 'B' {
            count2 += 1;
        } else if end == 'A' {
            count1 += 1;
        }
    }

    (count1, count2, count3)
}

fn count_method_1(new_ss: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let (mut count1, mut count2, mut count3) = counter(new_ss);
    let count_min = *[count1, count2, count3].iter().min().unwrap();

    ans += count_min * 2;
    count1 -= count_min;
    count2 -= count_min;
    count3 -= count_min;

    if count3 == 0 {
        ans += std::cmp::min(count1, count2);
    } else {
        // count1かcount2が0
        // count3 > 0
        if count1 == count2 {
            ans += count3 - 1
        } else {
            ans += count3
        }
    }
    ans
}

fn count_method_2(new_ss: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let (mut count1, mut count2, count3) = counter(new_ss);

    if count3 == 0 {
        ans += std::cmp::min(count1, count2)
    } else {
        if count1 == 0 && count2 == 0 {
            ans += count3 - 1
        } else if count1 == 0 || count2 == 0 {
            ans += count3
        } else {
            ans += count3 + 1;
            count1 -= 1;
            count2 -= 1;
            ans += std::cmp::min(count1, count2);
        }
    }

    ans
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    const MOD: usize = 1_000_000_007;
    const UINF: usize = std::usize::MAX;
    const IINF: isize = std::isize::MAX;

    input! {
        n: usize,
        ss: [Chars; n]
    }

    let mut t = 0;
    let mut new_ss = vec![];
    // ABは取り除いても変わらないので、ABを先に取り除いておく
    for s in ss.iter() {
        // ABを取り除く
        let mut new_s = vec![];
        let mut ab_index = vec![];
        for i in 0..s.len() - 1 {
            if s[i] == 'A' && s[i + 1] == 'B' {
                ab_index.push(i);
                ab_index.push(i + 1);
                t += 1;
            }
        }

        for i in 0..s.len() {
            if !ab_index.contains(&i) {
                new_s.push(s[i]);
            }
        }

        new_ss.push(new_s);
    }

    debug!(new_ss);

    let ans = std::cmp::max(count_method_1(&new_ss), count_method_2(&new_ss)) + t;

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
