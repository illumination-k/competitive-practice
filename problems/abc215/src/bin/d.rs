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
    collections::*,
    io::{BufRead, BufReader},
    ops::*,
};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[fastout]
fn run<R: BufRead>(source: AutoSource<R>) -> impl AtCoderFormat {
    input! {
        from source,
        n: usize, m: usize,
        a: [usize; n]
    }

    let max = max!(*a.iter().max().unwrap(), m);
    let mut ng = vec![false; max + 1];

    for &aa in a.iter() {
        ng[aa] = true;
    }

    let mut ng_numbers = HashSet::new();
    for i in 2..=max {
        let mut flag = false;
        // iの倍数がngならiがng
        for j in (i..=max).step_by(i) {
            debug!(i, j);
            if ng[j] {
                flag = true;
            }
        }

        if flag {
            ng_numbers.insert(i);
        }
    }
    let mut ok = vec![true; m + 1];
    for &i in ng_numbers.iter() {
        for j in (i..=m).step_by(i) {
            ok[j] = false;
        }
    }
    debug!(ng, ok);
    let ans = std::iter::once(1)
        .chain((2..=m).filter(|&x| ok[x]))
        .collect_vec();

    println!("{}", ans.len());
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
