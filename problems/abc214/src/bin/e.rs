#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;

use proconio::{fastout, input, marker::*, source::auto::AutoSource};
use std::{
    cmp::Reverse,
    collections::*,
    io::{BufRead, BufReader},
    ops::*,
};
use superslice::*;
use utils::*;
use whiteread::parse_line;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn inner(mut lr: Vec<(usize, usize)>) -> bool {
    let mut q = BinaryHeap::new();
    lr.push((UINF, UINF));
    lr.sort_unstable();
    let mut x = 1;

    for &(l, r) in lr.iter() {
        while x < l && !q.is_empty() {
            if let Some(Reverse(top)) = q.pop() {
                if top < x {
                    return false;
                }
            }
            x += 1;
        }
        x = l;
        q.push(Reverse(r))
    }

    true
}

fn solve(n: usize) -> bool {
    let mut lr = vec![];
    for _ in 0..n {
        let tmp: (usize, usize) = parse_line().unwrap();
        lr.push(tmp)
    }

    inner(lr)
}

#[fastout]
fn run<R: BufRead>(_: AutoSource<R>) -> impl AtCoderFormat {
    let t: usize = parse_line().unwrap();
    let mut ans = vec![];
    for _ in 0..t {
        let n: usize = parse_line().unwrap();
        ans.push(solve(n))
    }

    ans
}

fn main() {
    println!(
        "{}",
        run(AutoSource::new(BufReader::new(std::io::stdin()))).format()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use competitive::test_utility::*;

    #[test]
    fn test_solve() {
        for _ in 0..1000 {
            let n = gen_number(0, 1000);
            let mut lr = vec![];
            for _ in 0..n {
                let s = gen_number(0, 10000);
                lr.push((s, gen_number(s, s + 10000)));
            }

            let _ = inner(lr);
        }
    }
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

        macro_rules! impl_formats {
            ($($t: ty), *) => {
                $(impl_format!{$t})*
            };
        }

        impl_formats!(
            usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f32, f64, &str, String
        );

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
