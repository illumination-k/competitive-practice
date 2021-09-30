#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, hash::Hash, ops::*};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[derive(Debug, Clone)]
struct MultiSet<T> {
    inner_map: BTreeMap<T, usize>,
}

impl<T: Hash + Ord + Copy> MultiSet<T> {
    pub fn new() -> Self {
        Self {
            inner_map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, x: T) -> Option<T> {
        *self.inner_map.entry(x).or_insert(0) += 1;
        Some(x)
    }

    pub fn erase_one(&mut self, x: T) -> Option<T> {
        if let Some(count) = self.inner_map.get_mut(&x) {
            *count -= 1;
            if *count == 0 {
                self.inner_map.remove(&x);
            }
            Some(x)
        } else {
            None
        }
    }

    pub fn erase_all(&mut self, x: T) -> Option<T> {
        self.inner_map.remove(&x);
        Some(x)
    }

    pub fn min(&self) -> Option<T> {
        self.inner_map.iter().nth(0).map(|x| *x.0)
    }

    pub fn max(&self) -> Option<T> {
        self.inner_map.iter().last().map(|x| *x.0)
    }
}

#[derive(Debug)]
enum Action {
    Insert,
    Delete,
}

#[derive(Debug)]
struct Event {
    val: usize,
    action: Action,
}

#[fastout]
fn run() -> impl AtCoderFormat {
    input! {
        n: usize, q: usize,
        mut stx: [(usize, usize, usize); n],
        d: [usize; q]
    }

    let mut events = vec![];

    for &(s, t, x) in stx.iter() {
        events.push((
            s.saturating_sub(x),
            Event {
                val: x,
                action: Action::Insert,
            },
        ));
        events.push((
            t.saturating_sub(x),
            Event {
                val: x,
                action: Action::Delete,
            },
        ));
    }

    events.sort_by(|a, b| a.1.val.cmp(&b.1.val));
    let mut res: MultiSet<usize> = MultiSet::new();

    let mut ans = vec![];
    let mut k = 0;
    for i in 0..d.len() {
        debug!(i, res);
        while k < events.len() && events[k].0 <= d[i] {
            match events[k].1.action {
                Action::Insert => {
                    res.insert(events[k].1.val);
                }
                Action::Delete => {
                    res.erase_one(events[k].1.val);
                }
            }
            k += 1;
        }

        match res.min() {
            Some(x) => {
                ans.push(x as i64);
            }
            None => {
                ans.push(-1);
            }
        }
    }

    ans
}

fn main() {
    println!("{}", run().format());
}

#[cfg(test)]
mod test {
    use super::*;
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
