#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::data_structures::bmultiset;
use competitive_internal_mod::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
enum Action {
    Insert,
    Delete,
    Query,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Event {
    pos: usize,
    action: Action,
}

#[fastout]
fn run() -> impl AtCoderFormat {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut events = vec![];
    let mut queries = vec![];
    for &(a, b) in ab.iter() {
        events.push(Event {
            pos: a,
            action: Action::Insert,
        });
        events.push(Event {
            pos: a + b,
            action: Action::Delete,
        });

        queries.push(Event {
            pos: a,
            action: Action::Query,
        });
        queries.push(Event {
            pos: a + b,
            action: Action::Query,
        });
    }

    for query in queries.into_iter().unique() {
        events.push(query)
    }

    events.sort_by(|a, b| a.pos.cmp(&b.pos));
    let mut ans = vec![0; n];

    let mut count = 0;
    let mut last_pos = 0;
    for event in events.iter() {
        debug!(event, last_pos, count);
        match event.action {
            Action::Insert => {
                count += 1;
            }
            Action::Delete => {
                count -= 1;
            }
            Action::Query => {
                if count != 0 {
                    ans[count - 1] += event.pos - last_pos;
                }
                last_pos = event.pos;
            }
        }
    }

    debug!(events);
    debug!(ans);
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
    pub mod data_structures {
        pub mod bmultiset {
            use std::collections::BTreeMap;

            /// Ordered MultiSet
            #[derive(Debug, Clone)]
            pub struct BMultiSet<T> {
                pub inner_map: BTreeMap<T, usize>,
            }

            impl<T: Ord> BMultiSet<T> {
                pub fn new() -> Self {
                    Self {
                        inner_map: BTreeMap::new(),
                    }
                }

                /// Insert Value
                pub fn insert(&mut self, x: T) {
                    *self.inner_map.entry(x).or_insert(0) += 1;
                }

                /// Decrement count of the value.  
                /// If count is zero, remove this value.
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

                /// Return count of value
                pub fn count(&self, x: &T) -> Option<&usize> {
                    self.inner_map.get(x)
                }

                /// Remove value regradless of count
                pub fn erase_all(&mut self, x: T) -> Option<T> {
                    self.inner_map.remove(&x);
                    Some(x)
                }

                pub fn min(&self) -> Option<&T> {
                    self.inner_map.iter().nth(0).map(|x| x.0)
                }

                pub fn max(&self) -> Option<&T> {
                    self.inner_map.iter().last().map(|x| x.0)
                }

                pub fn is_empty(&self) -> bool {
                    self.inner_map.is_empty()
                }

                pub fn iter(&self) -> std::collections::btree_map::Keys<'_, T, usize> {
                    self.inner_map.keys()
                }

                pub fn contains(&self, x: &T) -> bool {
                    self.inner_map.contains_key(&x)
                }

                pub fn len(&self) -> usize {
                    self.inner_map.len()
                }

                pub fn clear(&mut self) {
                    self.inner_map.clear()
                }
            }

            pub struct FindIterator<T> {
                now: usize,
                limit: usize,
                val: T,
            }

            impl<'a, T> Iterator for FindIterator<&'a T> {
                type Item = &'a T;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.now < self.limit {
                        self.now += 1;
                        Some(self.val)
                    } else {
                        None
                    }
                }
            }

            impl<T: Ord> BMultiSet<T> {
                pub fn find(&self, x: T) -> FindIterator<T> {
                    let limit = *self.count(&x).unwrap_or(&0);
                    FindIterator {
                        now: 0,
                        limit: limit,
                        val: x,
                    }
                }
            }
        }
    }
}
